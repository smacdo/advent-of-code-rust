use std::{path::PathBuf, str::FromStr};

use chrono::{Datelike, Duration};
use regex::Regex;
use thiserror::Error;

use crate::{
    cache::{PuzzleCache, UserDataCache},
    data::{CheckResult, Puzzle},
    settings::ClientOptions,
    utils::get_puzzle_unlock_time,
    Answer, Day, Part, Year,
};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("the answer was submitted too soon, please wait until {} trying again", .0)]
    TooSoon(chrono::DateTime<chrono::Utc>),
    #[error("the session id `{}` is invalid or has expired", .0)]
    BadSessionId(String),
    #[error("a puzzle could not be found for day {} year {}", .0, .1)]
    PuzzleNotFound(Day, Year),
    #[error("please wait submitting answers to the Advent of Code service until {}", .0)]
    SubmitTimeOut(chrono::DateTime<chrono::Utc>),
    #[error("a correct answer has already been submitted for day {} year {}", .0, .1)]
    AlreadySubmittedAnswer(Day, Year),
    #[error("an unknown HTTP {} error was returned by the Advent of Code service", .0)]
    UnknownHttpError(reqwest::StatusCode),
}

pub trait Client {
    fn years(&self) -> Vec<Year>;
    fn days(&self, year: Year) -> Option<Vec<Day>>;
    fn get_input(&self, day: Day, year: Year) -> Result<String, ClientError>;
    fn submit_answer(
        &mut self,
        answer: Answer,
        part: Part,
        day: Day,
        year: Year,
    ) -> Result<CheckResult, ClientError>;
    fn get_puzzle(&self, day: Day, year: Year) -> Puzzle;
}

#[derive(Debug)]
pub struct WebClient {
    config: ClientConfig,
    http_client: reqwest::blocking::Client,
    puzzle_cache: PuzzleCache,
    user_cache: UserDataCache,
}

impl WebClient {
    const ADVENT_OF_CODE_DOMAIN: &'static str = "adventofcode.com";
    const ADVENT_OF_CODE_URL: &'static str = "https://adventofcode.com";

    pub fn new() -> Self {
        Self::with_options(Default::default())
    }

    pub fn with_options(options: ClientOptions) -> Self {
        // Convert client options into a actual configuration values.
        // TODO: validate config settings are sane.
        let config = ClientConfig::new(options);

        // Create an HTTP client for interacting with the Advent of Code website.
        // TODO: verify dev@smacdo.com email OK
        let cookies: reqwest::cookie::Jar = Default::default();
        let cookie_data = format!(
            "session={}; Domain={}",
            config.session_id,
            Self::ADVENT_OF_CODE_DOMAIN
        );

        tracing::debug!(
            "adding session id to cookie jar with value `{}`",
            cookie_data
        );

        cookies.add_cookie_str(
            &cookie_data,
            &Self::ADVENT_OF_CODE_URL.parse::<reqwest::Url>().unwrap(),
        );

        let http_client = reqwest::blocking::ClientBuilder::new()
            .cookie_provider(cookies.into())
            .user_agent("github.com/smacdo/advent-of-code-rust [email: dev@smacdo.com]")
            .build()
            .expect("unexpected error when constructing reqwest http client");

        let puzzle_dir = config.puzzle_dir.clone();
        let encryption_token = config.encryption_token.clone();

        // TODO: lets callers specify the user data cache.
        // TODO: create a default user data cache.
        Self {
            config,
            http_client,
            puzzle_cache: PuzzleCache::new(puzzle_dir, Some(encryption_token)),
            user_cache: UserDataCache::new(""),
        }
    }
}

impl Client for WebClient {
    fn years(&self) -> Vec<Year> {
        let start_time = self.config.start_time;
        let unlock_time = get_puzzle_unlock_time(start_time.year().into());

        let mut end_year = start_time.year();

        if start_time < unlock_time {
            end_year -= 1;
        }

        (2015..(end_year + 1)).map(|y| y.into()).collect()
    }

    fn days(&self, year: Year) -> Option<Vec<Day>> {
        let start_time = self.config.start_time;
        let eastern_start_time = start_time.with_timezone(&chrono_tz::US::Eastern);
        let requested_year = year.0 as i32;

        match eastern_start_time.year().cmp(&requested_year) {
            std::cmp::Ordering::Less => None,
            std::cmp::Ordering::Equal => Some(
                (1..(eastern_start_time.day() + 1))
                    .map(|d| d.into())
                    .collect(),
            ),
            std::cmp::Ordering::Greater => Some((0..25).map(|d| d.into()).collect()),
        }
    }

    fn get_input(&self, day: Day, year: Year) -> Result<String, ClientError> {
        tracing::trace!("get_input(day=`{day}`, year=`{year}`)",);

        // TODO: Convert expects and unwraps into errors.

        // Check if the input for this puzzle is cached locally before requesting
        // it from the Advent of Code service.
        // TODO: Return an error if the result is anything other than missing file.
        if let Ok(input) = self.puzzle_cache.load_input(day, year) {
            return Ok(input);
        }

        // Fetch the puzzle input from the Advent of Code service.
        let url = format!("{}/{}/day/{}/input", Self::ADVENT_OF_CODE_URL, year, day);

        tracing::debug!(
            "url to get puzzle input for day {} year {}: `{}`",
            day,
            year,
            url
        );

        let response = self.http_client.get(url).send().unwrap();
        tracing::debug!("server responed with HTTP {}", response.status());

        match response.status() {
            reqwest::StatusCode::OK => {
                // Cache the puzzle input on disk so we don't have to refetch it
                // from the Advent of Code service.
                let input = response.text().unwrap();
                self.puzzle_cache.save_input(&input, day, year).unwrap();

                // Return the input to the caller.
                Ok(input)
            }
            reqwest::StatusCode::BAD_REQUEST => Err(ClientError::BadSessionId(
                self.config.session_id.to_string(),
            )),
            reqwest::StatusCode::NOT_FOUND => {
                // TODO: Return "Not available _yet_" if the requested data in the future.
                Err(ClientError::PuzzleNotFound(day, year))
            }
            _ => Err(ClientError::UnknownHttpError(response.status())),
        }

        // TODO: Handle:
        // Please don't repeatedly request this endpoint before it unlocks! The calendar countdown is synchronized with the server time; the link will be enabled on the calendar the instant this puzzle becomes available.
    }

    fn submit_answer(
        &mut self,
        answer: Answer,
        part: Part,
        day: Day,
        year: Year,
    ) -> Result<CheckResult, ClientError> {
        // TODO: Convert expects and unwraps into errors.
        tracing::trace!(
            "submit_answer(answer=`{:?}`, part=`{}`, day=`{}`, year=`{}`)",
            answer,
            part,
            day,
            year
        );

        // Check the cache to see if this answer can be checked locally without
        // having to hit the server.
        //
        // TODO: Warn if the input isn't available because it's likely something
        //       went wrong if we're submitting an answer without having cached
        //       the input.
        let mut answers = self
            .puzzle_cache
            .load_answers(part, day, year)
            .unwrap_or_default();

        if let Some(check_result) = answers.check(&answer) {
            tracing::debug!("answer check result was found in the cache {check_result:?}");
            return Ok(check_result);
        }

        // Check if there is an active time out on new submissions prior to
        // submitting to the advent of code service.
        let mut user = self.user_cache.load(&self.config.session_id);

        if let Some(submit_wait_until) = user.submit_wait_until {
            if chrono::Utc::now() <= submit_wait_until {
                tracing::warn!("user cannot submit an answer until {submit_wait_until}");
                return Err(ClientError::SubmitTimeOut(submit_wait_until));
            } else {
                // TODO: remove the timeout and save.
                tracing::debug!("user submission timeout has expired, ignoring");
            }
        }

        // Submit to the answer to Advent of Code service.
        let url = format!("{}/{}/day/{}/answer", Self::ADVENT_OF_CODE_URL, year, day);

        tracing::debug!(
            "creating url to post puzzle answer for part {:?} day {} year {} answer `{}` with url = `{}`",
            part,
            day,
            year,
            answer,
            url
        );

        let response = self
            .http_client
            .post(url)
            .form(&[
                (
                    "level",
                    if part == Part::One {
                        "1".to_string()
                    } else {
                        "2".to_string()
                    },
                ),
                ("answer", answer.to_string()),
            ])
            .send()
            .unwrap();
        tracing::debug!("server responed with HTTP {}", response.status());

        // Exit early if there were any fatal HTTP errors.
        if response.status().is_client_error() || response.status().is_server_error() {
            return match response.status() {
                reqwest::StatusCode::BAD_REQUEST => Err(ClientError::BadSessionId(
                    self.config.session_id.to_string(),
                )),
                reqwest::StatusCode::NOT_FOUND => {
                    // TODO: Return "Not available _yet_" if the requested data in the future.
                    Err(ClientError::PuzzleNotFound(day, year))
                }
                _ => Err(ClientError::UnknownHttpError(response.status())),
            };
        }

        // Read the contents of the response.
        let response_text = response.text().unwrap();
        tracing::debug!("got advent of code servier response for answer: {answer}");

        // Look for a minimum wait time in the text.
        let extract_wait_time_funcs = &[
            Self::extract_error_time_to_wait,
            Self::extract_one_minute_time_to_wait,
            Self::extract_wrong_answer_time_to_wait,
        ];

        if let Some(time_to_wait) = extract_wait_time_funcs
            .iter()
            .filter_map(|f| f(&response_text))
            .next()
        {
            // Write back the amount of time to wait to avoid hitting the server
            // on future submissions.
            let wait_until = chrono::Utc::now() + time_to_wait;
            tracing::debug!("setting time to wait ({time_to_wait}) to be {wait_until}");

            user.submit_wait_until = Some(wait_until);
            self.user_cache.save(&user);
        }

        // Handle special cases.
        // TODO: Remove this special casing if possible.
        // TODO: Look into "You don't seem to be solving the right level.  Did you already complete it?"
        //       Is this only returned for errors on solved levels?
        if response_text.contains("gave an answer too recently") {
            return Err(ClientError::SubmitTimeOut(user.submit_wait_until.unwrap()));
        }

        if response_text.contains("you already complete it") {
            return Err(ClientError::AlreadySubmittedAnswer(day, year));
        }

        // Translate the response text into a result.
        let responses_texts = &[
            ("not the right answer", CheckResult::Wrong),
            ("the right answer", CheckResult::Correct),
            ("answer is too low", CheckResult::TooLow),
            ("answer is too high", CheckResult::TooHigh),
        ];

        let check_result = responses_texts
            .iter()
            .find(|x| response_text.contains(x.0))
            .map(|x| x.1.clone())
            .unwrap_or_else(|| panic!("expected server response text to map to predetermined response in LUT. Response:\n```\n{response_text}\n```\n"));

        // Write the response to the answers database and then save it back to
        // the puzzle cache.
        match check_result {
            CheckResult::Correct => {
                tracing::debug!("Setting correct answer as {answer}");
                answers.set_correct_answer(answer);
            }
            CheckResult::Wrong => {
                tracing::debug!("Setting wrong answer {answer}");
                answers.add_wrong_answer(answer);
            }
            CheckResult::TooLow => {
                tracing::debug!("Setting low bounds wrong answer {answer}");
                answers.set_low_bounds(answer);
            }
            CheckResult::TooHigh => {
                tracing::debug!("Setting high bounds wrong answer {answer}");
                answers.set_high_bounds(answer);
            }
        };

        // TODO: Report errors.
        tracing::debug!("Saving answers database to puzzle cache");
        self.puzzle_cache
            .save_answers(&answers, part, day, year)
            .unwrap();

        Ok(check_result)
    }

    fn get_puzzle(&self, day: Day, year: Year) -> Puzzle {
        self.puzzle_cache.load_puzzle(day, year).unwrap()
    }

    // TODO: personal leaderboard
    // TODO: list of private leaderboards
    // TODO: show private leaderboard
}

impl WebClient {
    fn extract_one_minute_time_to_wait(response: &str) -> Option<Duration> {
        match response.contains("Please wait one minute before trying again") {
            true => Some(Duration::minutes(5)),
            false => None,
        }
    }

    fn extract_wrong_answer_time_to_wait(response: &str) -> Option<Duration> {
        let regex = Regex::new(r"please wait (\d) minutes?").unwrap();
        regex
            .captures(response)
            .map(|c| Duration::minutes(c[1].parse::<i64>().unwrap()))
    }

    fn extract_error_time_to_wait(response: &str) -> Option<Duration> {
        let regex = Regex::new(r"You have (\d+)m( (\d+)s)? left to wait").unwrap();
        regex.captures(response).map(|c| {
            let mut time_to_wait = Duration::minutes(c[1].parse::<i64>().unwrap());

            if let Some(secs) = c.get(3) {
                time_to_wait += Duration::seconds(secs.as_str().parse::<i64>().unwrap());
            }

            time_to_wait
        })
    }
}

impl Default for WebClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug)]
pub struct ClientConfig {
    pub session_id: String,
    pub puzzle_dir: PathBuf,
    pub encryption_token: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

impl ClientConfig {
    pub fn new(options: ClientOptions) -> Self {
        // TODO: convert panics into Errors
        // TODO: verify directory exists
        Self {
            session_id: options.session_id.expect("session id must be set"),
            puzzle_dir: options
                .puzzle_dir
                .unwrap_or(PathBuf::from_str(".puzzles").unwrap()),
            encryption_token: options
                .encryption_token
                .expect("encryption token must be set"),
            start_time: options.fake_time.unwrap_or(chrono::Utc::now()),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime, TimeZone};
    use chrono_tz::US::Eastern;

    use super::*;

    fn web_client_with_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> WebClient {
        WebClient::with_options(
            ClientOptions::new()
                .with_session_id("UNIT_TEST_SESSION_ID")
                .with_encryption_token("UNIT_TEST_PASSWORD")
                .with_puzzle_dir("DO_NOT_USE")
                .with_fake_time(
                    Eastern
                        .from_local_datetime(
                            &NaiveDate::from_ymd_opt(year, month, day)
                                .unwrap()
                                .and_time(NaiveTime::from_hms_opt(hour, min, sec).unwrap()),
                        )
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                ),
        )
    }

    #[test]
    fn list_years_when_date_is_after_start() {
        let client = web_client_with_time(2018, 12, 1, 0, 0, 0);
        assert_eq!(
            client.years(),
            vec![Year(2015), Year(2016), Year(2017), Year(2018)]
        );
    }

    #[test]
    fn list_years_when_date_is_before_start() {
        let client = web_client_with_time(2018, 11, 30, 23, 59, 59);
        assert_eq!(client.years(), vec![Year(2015), Year(2016), Year(2017)]);
    }

    #[test]
    fn list_years_when_date_aoc_start() {
        let client = web_client_with_time(2015, 3, 10, 11, 15, 7);
        assert_eq!(client.years(), vec![]);
    }

    #[test]
    fn list_days_before_start() {
        let client = web_client_with_time(2020, 11, 30, 23, 59, 59);
        assert_eq!(client.days(Year(2020)), None);
    }

    #[test]
    fn list_days_at_start() {
        let client = web_client_with_time(2020, 12, 1, 0, 0, 0);
        assert_eq!(client.days(Year(2020)), Some(vec![Day(1)]));
    }

    #[test]
    fn list_days_in_middle() {
        let client = web_client_with_time(2020, 12, 6, 0, 0, 0);
        assert_eq!(
            client.days(Year(2020)),
            Some(vec![Day(1), Day(2), Day(3), Day(4), Day(5), Day(6)])
        );
    }
}
