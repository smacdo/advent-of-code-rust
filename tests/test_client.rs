use std::path::PathBuf;

use advent_of_code_data::{
    client::{
        protocol::{ServiceConnector, ServiceError},
        Client, ClientError, WebClient,
    },
    config::{Config, ConfigBuilder},
    Answer, Day, Part, Year,
};
use tempfile::{tempdir, TempDir};

fn make_test_config(session: Option<&str>, temp_dir: &TempDir) -> Config {
    let mut builder = ConfigBuilder::new()
        .with_passphrase("1234")
        .with_puzzle_dir(PathBuf::from(temp_dir.path()))
        .with_sessions_dir(PathBuf::from(temp_dir.path()));

    if let Some(session) = session {
        builder = builder.with_session_id(session);
    }

    builder.build().unwrap()
}

struct TestAdventOfCodeService {
    mock_get_input: Box<dyn Fn(Day, Year, &str) -> Result<String, ServiceError>>,
    mock_submit_answer: Box<dyn Fn(&Answer, Part, Day, Year, &str) -> Result<String, ServiceError>>,
}

impl ServiceConnector for TestAdventOfCodeService {
    fn get_input(&self, day: Day, year: Year, session: &str) -> Result<String, ServiceError> {
        (self.mock_get_input)(day, year, session)
    }

    fn submit_answer(
        &self,
        answer: &Answer,
        part: Part,
        day: Day,
        year: Year,
        session: &str,
    ) -> Result<String, ServiceError> {
        (self.mock_submit_answer)(answer, part, day, year, session)
    }
}

#[test]
fn get_input_ok() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let client = WebClient::with_custom_impl(
        config,
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Ok("hello world".to_string())
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    assert_eq!(
        &client.get_input(Day(1), Year(2000)).unwrap(),
        "hello world"
    );
}

// TODO: test `get_input` service connector called with expected day, year and session.
// TODO: test `get_input` reads results from the cache and skips service connector.
// TODO: test `get_input` writes input to the cache.

#[test]
fn get_input_missing_session_error() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(None, &temp_dir);

    let client = WebClient::with_custom_impl(
        config,
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Err(ServiceError::HttpStatusError(400))
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    assert!(matches!(
        client.get_input(Day(1), Year(2000)),
        Err(ClientError::SessionIdRequired)
    ));
}

#[test]
fn get_input_invalid_session_error() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let client = WebClient::with_custom_impl(
        config,
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Err(ServiceError::HttpStatusError(400))
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    assert!(matches!(
        client.get_input(Day(1), Year(2000)),
        Err(ClientError::BadSessionId(_))
    ));
}

// TODO: test `get_input` arbitrary HTTP status error returned.

// TODO: test `submit_answer` returns CheckResult::Correct for correct answer
// TODO: test `submit_answer` returns CheckResult::Correct for wrong answer
// TODO: test `submit_answer` returns CheckResult::Correct for low bounds wrong answer
// TODO: test `submit_answer` returns CheckResult::Correct for high bounds wrong answer

// TODO: test `submit_answer` service connector called with expected day, year and session.

// TODO: test `submit_answer` checks the cache and returns early if cache OK.
// TODO: test `submit_answer` errors if session id not provided.
// TODO: test `submit_answer` errors if there is a submission timeout not past.
// TODO: test `submit_answer` continues if the submission timeout is set but in the past.
// TODO: test `submit_answer` writes answers to the cache.
// TODO: test `submit_answer` errors if bad submission id (400).
// TODO: test `submit_answer` errors if invalid puzzle (404).
// TODO: test `submit_answer` for part 2 before part 1.
// TODO: test `submit_answer` arbitrary HTTP status error returned.
// TODO: test `submit_answer` test that server sends "one minute" timeout and timeout is set.
// TODO: test `submit_answer` test that server sends timeout text variant 1 and timeout is set.
// TODO: test `submit_answer` test that server sends timeout text variant 2 and timeout is set.
