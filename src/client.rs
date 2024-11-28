use std::{path::PathBuf, str::FromStr};

use crate::{settings::ClientOptions, Answer, Day, Part, Puzzle, Year};

pub trait Client {}

pub struct WebClient {
    config: ClientConfig,
}

impl WebClient {
    pub fn new() -> Self {
        Self::with_options(Default::default())
    }

    pub fn with_options(options: ClientOptions) -> Self {
        Self {
            config: ClientConfig::new(options),
        }
    }

    pub fn years(&self) -> Vec<Year> {
        todo!()
    }

    pub fn days(&self, year: Year) -> Vec<Day> {
        todo!()
    }

    pub fn get_input(&self, day: Day, year: Year) -> String {
        todo!()
    }

    pub fn submit_answer(&mut self, answer: Answer, part: Part, day: Day, year: Year) {
        todo!()
    }

    pub fn get_puzzle(&self, day: Day, year: Year) -> Puzzle {
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

#[derive(Default)]
pub struct ClientConfig {
    pub session_id: String,
    pub cache_dir: PathBuf,
    pub encryption_token: String,
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
        }
    }
}
