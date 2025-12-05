use std::path::PathBuf;

use thiserror::Error;

use crate::{Answer, Day, Part, Year};

// TODO: Use "service" rather than protocol.

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("HTTP {}", .0)]
    HttpStatusError(u16),
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
}

/// Abstraction of the communication protocol used to communicate with the
/// Advent of Code backend web service enabling test mocks.
pub trait ServiceConnector {
    fn get_input(&self, day: Day, year: Year, session: &str) -> Result<String, ServiceError>;
    fn submit_answer(
        &self,
        answer: &Answer,
        part: Part,
        day: Day,
        year: Year,
        session: &str,
    ) -> Result<String, ServiceError>;
}

#[derive(Debug)]
pub struct AdventOfCodeService {
    pub log_dir: Option<PathBuf>,
}

impl AdventOfCodeService {
    const ADVENT_OF_CODE_DOMAIN: &'static str = "adventofcode.com";
    const ADVENT_OF_CODE_URL: &'static str = "https://adventofcode.com";

    fn create_http_client(
        &self,
        session: Option<&str>,
    ) -> Result<reqwest::blocking::Client, ServiceError> {
        // Create an HTTP client for interacting with the Advent of Code website.
        // TODO: verify dev@smacdo.com email OK
        let cookies: reqwest::cookie::Jar = Default::default();

        if let Some(session) = session {
            let cookie_data = format!(
                "session={}; Domain={}",
                session,
                Self::ADVENT_OF_CODE_DOMAIN
            );

            tracing::debug!("adding session id `{}` to cookie jar", cookie_data);

            cookies.add_cookie_str(
                &cookie_data,
                &Self::ADVENT_OF_CODE_URL.parse::<reqwest::Url>().unwrap(),
            );
        }

        Ok(reqwest::blocking::ClientBuilder::new()
            .cookie_provider(cookies.into())
            .user_agent("github.com/smacdo/advent-of-code-rust [email: dev@smacdo.com]")
            .build()?)
    }

    /// Write a server response to the given directory for debugging purposes.
    pub fn log_server_response(&self, status: u16, text: &str) {
        if let Some(log_dir) = &self.log_dir {
            // Create log directory if it doesn't exist before writing.
            if !log_dir.exists() {
                std::fs::create_dir_all(log_dir).unwrap_or_else(|e| {
                    tracing::error!("error create server response log dir: {e}")
                });
            }

            // Write to a file that has a unique name taken from the current time.
            // Also add the HTTP status code as a suffix to the file name.
            let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
            let log_filename = format!("{}_{}.html", now.format("%Y_%m_%d-%H_%M_%S"), status);

            tracing::debug!(
                "logging server response to: {}",
                log_dir.join(&log_filename).to_string_lossy()
            );

            std::fs::write(log_dir.join(log_filename), text)
                .unwrap_or_else(|e| tracing::error!("error when logging server response: {e}"));
        }
    }
}

impl ServiceConnector for AdventOfCodeService {
    fn get_input(&self, day: Day, year: Year, session: &str) -> Result<String, ServiceError> {
        let url = format!("{}/{}/day/{}/input", Self::ADVENT_OF_CODE_URL, year, day);

        tracing::debug!(
            "url to get puzzle input for day {} year {} is `{}`",
            day,
            year,
            url
        );

        let response = self.create_http_client(Some(session))?.get(url).send()?;
        tracing::debug!("server responed with HTTP {}", response.status());

        if response.status() == reqwest::StatusCode::OK {
            Ok(response.text().unwrap()) // TODO: Handle this!
        } else {
            Err(ServiceError::HttpStatusError(response.status().as_u16()))
        }
    }

    fn submit_answer(
        &self,
        answer: &Answer,
        part: Part,
        day: Day,
        year: Year,
        session: &str,
    ) -> Result<String, ServiceError> {
        // TODO: Convert expects and unwraps into errors.
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
            .create_http_client(Some(session))?
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
            .send()?;

        let status = response.status();
        let text = response.text().unwrap_or_else(|e| {
            tracing::error!("Error when decoding server response text: {e}");
            String::new()
        });

        tracing::debug!("server responed with HTTP {}", status);
        self.log_server_response(status.as_u16(), &text);

        if status == reqwest::StatusCode::OK {
            Ok(text)
        } else {
            Err(ServiceError::HttpStatusError(status.as_u16()))
        }
    }
}

impl AdventOfCodeService {}
