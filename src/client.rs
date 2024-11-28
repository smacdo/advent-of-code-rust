use std::{path::PathBuf, str::FromStr};

use chrono::Datelike;

use crate::{
    settings::ClientOptions, utils::get_puzzle_unlock_time, Answer, Day, Part, Puzzle, Year,
};

pub trait Client {}

#[derive(Debug)]
pub struct WebClient {
    config: ClientConfig,
    http_client: reqwest::blocking::Client,
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

        Self {
            config,
            http_client,
        }
    }

    pub fn years(&self) -> Vec<Year> {
        let start_time = self.config.start_time;
        let unlock_time = get_puzzle_unlock_time(start_time.year().into());

        let mut end_year = start_time.year();

        if start_time < unlock_time {
            end_year -= 1;
        }

        (2015..(end_year + 1)).map(|y| y.into()).collect()
    }

    pub fn days(&self, year: Year) -> Option<Vec<Day>> {
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

    pub fn get_input(&self, day: Day, year: Year) -> String {
        // TODO: convert execute expect into errors.

        // Format the URL to fetch puzzle input.
        let url = format!(
            "{}/{}/day/{}/input",
            Self::ADVENT_OF_CODE_URL,
            year.0,
            day.0
        );

        tracing::debug!(
            "creating url to get puzzle input for day {} year {} with url = `{}`",
            day.0,
            year.0,
            url
        );

        // Download the puzzle input to a string.
        let request = self
            .http_client
            .get(url)
            .build()
            .expect("unexpected error when building HTTP GET request for `get_input`");

        self.http_client
            .execute(request)
            .expect("unexpected error when HTTP GET for `get_input`")
            .text()
            .expect("unexpected error don't know what")

        // TODO: Check for "Puzzle inputs differ by user.  Please log in to get your puzzle input."
        // TODO: ^^^ above text comes with HTTP 400
        // TODO: If the session id is set when this happens its either bad or timed out.
    }

    pub fn submit_answer(&mut self, _answer: Answer, _part: Part, _day: Day, _year: Year) {
        todo!()
    }

    pub fn get_puzzle(&self, _day: Day, _year: Year) -> Puzzle {
        todo!()
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
    pub cache_dir: PathBuf,
    pub encryption_token: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

impl ClientConfig {
    pub fn new(options: ClientOptions) -> Self {
        // TODO: convert panics into Errors
        // TODO: verify directory exists
        Self {
            session_id: options.session_id.expect("session id must be set"),
            cache_dir: options
                .cache_dir
                .unwrap_or(PathBuf::from_str(".aoc_client_cache").unwrap()),
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
                .with_cache_dir("DO_NOT_USE")
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
