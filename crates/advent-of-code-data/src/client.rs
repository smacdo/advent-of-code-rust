mod protocol;

use std::{path::PathBuf, str::FromStr};

use chrono::Datelike;
use protocol::{AdventOfCodeHttpProtocol, AdventOfCodeProtocol};
use thiserror::Error;

use crate::{
    cache::{CacheError, PuzzleCache, PuzzleFsCache, UserDataCache, UserDataFsCache},
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
    #[error("please wait {} before submitting another answer to the Advent of Code service", .0)]
    SubmitTimeOut(chrono::Duration),
    #[error("a correct answer has already been submitted for day {} year {}", .0, .1)]
    AlreadySubmittedAnswer(Day, Year),
    #[error("an unexpected HTTP {} error was returned by the Advent of Code service", .0)]
    Http(reqwest::StatusCode),
    #[error("an unexpected error {} error happened when reading cached data", .0)]
    CacheError(CacheError),
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
    pub config: ClientConfig,
    protocol: Box<dyn AdventOfCodeProtocol>,
    pub puzzle_cache: Box<dyn PuzzleCache>,
    pub user_cache: Box<dyn UserDataCache>,
}

impl WebClient {
    pub fn new() -> Self {
        Self::with_options(Default::default())
    }

    pub fn with_options(options: ClientOptions) -> Self {
        let config = ClientConfig::new(options);
        let advent_protocol = Box::new(AdventOfCodeHttpProtocol::new(&config));
        Self::with_custom_impl(config, advent_protocol)
    }

    pub fn with_custom_impl(
        config: ClientConfig,
        advent_protocol: Box<dyn AdventOfCodeProtocol>,
    ) -> Self {
        // Convert client options into a actual configuration values.
        // TODO: validate config settings are sane.
        let puzzle_dir = config.puzzle_dir.clone();
        let user_data_dir = config.user_cache_dir.clone();
        let encryption_token = config.encryption_token.clone();

        // Print configuration settings to debug log.
        tracing::debug!("client puzzle dir: {puzzle_dir:?}");
        tracing::debug!("client user data dir: {user_data_dir:?}");
        tracing::debug!("client puzzles using encryption: {}", true);

        // TODO: lets callers specify the user data cache.
        // TODO: create a default user data cache.
        Self {
            config,
            protocol: advent_protocol,
            puzzle_cache: Box::new(PuzzleFsCache::new(puzzle_dir, Some(encryption_token))),
            user_cache: Box::new(UserDataFsCache::new(user_data_dir)),
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

        match (
            eastern_start_time.year().cmp(&requested_year),
            eastern_start_time.month() == 12,
        ) {
            (std::cmp::Ordering::Equal, true) => Some(
                (1..(eastern_start_time.day() + 1))
                    .map(|d| d.into())
                    .collect(),
            ),
            (std::cmp::Ordering::Greater, _) => Some((0..25).map(|d| d.into()).collect()),
            _ => None,
        }
    }

    fn get_input(&self, day: Day, year: Year) -> Result<String, ClientError> {
        tracing::trace!("get_input(day=`{day}`, year=`{year}`)",);

        // TODO: Convert expects and unwraps into errors.

        // Check if the input for this puzzle is cached locally before fetching
        // it from the Advent of Code service.
        if let Some(input) = self
            .puzzle_cache
            .load_input(day, year)
            .map_err(ClientError::CacheError)?
        {
            return Ok(input);
        }

        // Fetch the puzzle input from the Advent of Code service.
        let input = self.protocol.get_input(day, year)?;

        // Cache the puzzle input on disk before returning to avoid repeatedly
        // fetching input from the Advent of Code service.
        self.puzzle_cache.save_input(&input, day, year).unwrap();
        Ok(input)
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
        let mut user = self.user_cache.load(&self.config.session_id).unwrap();

        if let Some(submit_wait_until) = user.submit_wait_until {
            if self.config.start_time <= submit_wait_until {
                tracing::warn!("user cannot submit an answer until {submit_wait_until}");
                return Err(ClientError::SubmitTimeOut(
                    submit_wait_until - self.config.start_time,
                ));
            } else {
                // TODO: remove the timeout and save.
                tracing::debug!("user submission timeout has expired, ignoring");
            }
        }

        // Submit to the answer to Advent of Code service.
        let (check_result, time_to_wait) = self.protocol.submit_answer(&answer, part, day, year)?;

        // Write back the amount of time to wait to avoid hitting the server
        // on future submissions.
        if let Some(time_to_wait) = time_to_wait {
            let wait_until = chrono::Utc::now() + time_to_wait;
            tracing::debug!("setting time to wait ({time_to_wait}) to be {wait_until}");

            user.submit_wait_until = Some(wait_until);
            self.user_cache.save(&user).unwrap();
        }

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

impl Default for WebClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug)]
pub struct ClientConfig {
    pub session_id: String,
    pub puzzle_dir: PathBuf,
    pub user_cache_dir: PathBuf,
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
            user_cache_dir: options
                .user_cache_dir
                .unwrap_or(PathBuf::from_str(".advent-of-code-data-cache").unwrap()),
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
