use std::path::PathBuf;

use advent_of_code_data::{
    cache::{PuzzleCache, PuzzleFsCache, SessionCache, SessionFsCache},
    client::{
        protocol::{ServiceConnector, ServiceError},
        Client, ClientError, WebClient,
    },
    config::{Config, ConfigBuilder},
    data::{Answers, CheckResult, Session},
    Answer, Day, Part, Year,
};
use chrono::Duration;
use tempfile::{tempdir, TempDir};

const WRONG_ANSWER_WAIT_ONE_MINUTE: &str = "That's not the right answer.  If you're stuck, make sure you're using the full input data; there are also some general tips on the <a href=\"/2023/about\">about page</a>, or you can ask for hints on the <a href=\"https://www.reddit.com/r/adventofcode/\" target=\"_blank\">subreddit</a>.  Please wait one minute before trying again.";
const WRONG_ANSWER_WAIT_TWO_MINUTES: &str = "That's not the right answer - please wait 2 minutes";
const WRONG_ANSWER_WAIT_FIVE_M: &str = "That's not the right answer - You have 5m left to wait";
const WRONG_ANSWER_WAIT_SIX_M_TEN_S: &str =
    "That's not the right answer - You have 6m 10s left to wait";

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

fn get_cached_answers(config: &Config, part: Part, day: Day, year: Year) -> Option<Answers> {
    let puzzle_cache = PuzzleFsCache::new(
        config.puzzle_dir.clone(),
        Some(config.passphrase.to_string()),
    );

    puzzle_cache.load_answers(part, day, year).unwrap()
}

fn get_cached_session(config: &Config) -> Option<Session> {
    SessionFsCache::new(config.sessions_dir.clone())
        .try_load(config.session_id.as_ref().unwrap())
        .unwrap()
}

fn write_session(config: &Config, session: &Session) {
    SessionFsCache::new(config.sessions_dir.clone())
        .save(session)
        .unwrap();
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

// TODO: test `submit_answer` service connector called with expected day, year and session.

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

#[test]
fn submit_correct_answer() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Create mock submit_answer
    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok("That's the right answer! You are one star closer".to_string())
            }),
        }),
    );

    // Check there is no cached answers before calling submit_answer.
    assert!(
        get_cached_answers(&config, Part::One, Day(1), Year(2000)).is_none(),
        "there should be no cached answers before submit_answer"
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Correct
    );

    // Check submit_answer added the submitted answer to the cache.
    if let Some(answers) = get_cached_answers(&config, Part::One, Day(1), Year(2000)) {
        assert!(answers
            .correct_answer_ref()
            .as_ref()
            .map(|a| *a == Answer::Int(42))
            .unwrap_or(false))
    } else {
        panic!("expected answer cache to exist after submit_answer");
    }
}

#[test]
fn submit_wrong_answer() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok("That's not the right answer".to_string())
            }),
        }),
    );

    // Check there is no cached answers before calling submit_answer.
    assert!(
        get_cached_answers(&config, Part::One, Day(1), Year(2000)).is_none(),
        "there should be no cached answers before submit_answer"
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Wrong
    );

    // Check submit_answer added the submitted answer to the cache.
    if let Some(answers) = get_cached_answers(&config, Part::One, Day(1), Year(2000)) {
        assert!(answers
            .wrong_answers_ref()
            .first()
            .map(|a| *a == Answer::Int(42))
            .unwrap_or(false))
    } else {
        panic!("expected answer cache to exist after submit_answer");
    }
}

#[test]
fn submit_wrong_answer_too_low() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok("<p>Your answer is too low.</p>\n<p>If you're stuck, ".to_string())
            }),
        }),
    );

    // Check there is no cached answers before calling submit_answer.
    assert!(
        get_cached_answers(&config, Part::One, Day(1), Year(2000)).is_none(),
        "there should be no cached answers before submit_answer"
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::TooLow
    );

    // Check submit_answer added the submitted answer to the cache.
    if let Some(answers) = get_cached_answers(&config, Part::One, Day(1), Year(2000)) {
        assert!(answers.low_bounds_ref().map(|a| a == 42).unwrap_or(false))
    } else {
        panic!("expected answer cache to exist after submit_answer");
    }
}

#[test]
fn submit_wrong_answer_too_high() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok("<p>Your answer is too high.</p>\n<p>If you're stuck, ".to_string())
            }),
        }),
    );

    // Check there is no cached answers before calling submit_answer.
    assert!(
        get_cached_answers(&config, Part::One, Day(1), Year(2000)).is_none(),
        "there should be no cached answers before submit_answer"
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::TooHigh
    );

    // Check submit_answer added the submitted answer to the cache.
    if let Some(answers) = get_cached_answers(&config, Part::One, Day(1), Year(2000)) {
        assert!(answers.high_bounds_ref().map(|a| a == 42).unwrap_or(false))
    } else {
        panic!("expected answer cache to exist after submit_answer");
    }
}

#[test]
fn submit_answer_does_not_call_backend_if_timeout_set() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Create a session with timeout in the future.
    let mut session = Session::new(config.session_id.clone().unwrap());
    session.submit_wait_until = Some(config.start_time + Duration::minutes(1));

    write_session(&config, &session);

    // Make sure the client returns an error with the submission timeout inside, and verify that the
    // service endpoint is never contacted.
    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    match client.submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000)) {
        Err(ClientError::SubmitTimeOut(timeout)) => {
            assert_eq!(
                timeout,
                session.submit_wait_until.unwrap() - config.start_time
            );
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}

#[test]
fn submit_answer_proceeds_if_timeout_is_in_the_past() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Create a session with timeout in the future.
    let mut session = Session::new(config.session_id.clone().unwrap());
    session.submit_wait_until = Some(config.start_time - Duration::minutes(1));

    write_session(&config, &session);

    // Submit an answer and verify the backend was called.
    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok("That's not the right answer".to_string())
            }),
        }),
    );

    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Wrong
    );
}

#[test]
fn submit_answer_one_minute_timeout() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok(WRONG_ANSWER_WAIT_ONE_MINUTE.to_string())
            }),
        }),
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Wrong
    );

    // Look for a timeout in the user's session data.
    let actual_time = get_cached_session(&config)
        .expect("session must be written after a submit with a timeout")
        .submit_wait_until
        .expect("wait until time must be written after a submit with a timeout");

    let expected_time = config.start_time + Duration::minutes(1);

    assert!(
        actual_time >= expected_time - Duration::seconds(1)
            && actual_time <= expected_time + Duration::seconds(1)
    );
}

#[test]
fn submit_answer_multiple_minute_timeout() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok(WRONG_ANSWER_WAIT_TWO_MINUTES.to_string())
            }),
        }),
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Wrong
    );

    // Look for a timeout in the user's session data.
    let actual_time = get_cached_session(&config)
        .expect("session must be written after a submit with a timeout")
        .submit_wait_until
        .expect("wait until time must be written after a submit with a timeout");

    let expected_time = config.start_time + Duration::minutes(2);

    assert!(
        actual_time >= expected_time - Duration::seconds(1)
            && actual_time <= expected_time + Duration::seconds(1)
    );
}

#[test]
fn submit_answer_wait_five_m() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok(WRONG_ANSWER_WAIT_FIVE_M.to_string())
            }),
        }),
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Wrong
    );

    // Look for a timeout in the user's session data.
    let actual_time = get_cached_session(&config)
        .expect("session must be written after a submit with a timeout")
        .submit_wait_until
        .expect("wait until time must be written after a submit with a timeout");

    let expected_time = config.start_time + Duration::minutes(5);

    assert!(
        actual_time >= expected_time - Duration::seconds(1)
            && actual_time <= expected_time + Duration::seconds(1)
    );
}

#[test]
fn submit_answer_wait_six_m() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Ok(WRONG_ANSWER_WAIT_SIX_M_TEN_S.to_string())
            }),
        }),
    );

    // Check submit_answer returns expected response.
    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Wrong
    );

    // Look for a timeout in the user's session data.
    let actual_time = get_cached_session(&config)
        .expect("session must be written after a submit with a timeout")
        .submit_wait_until
        .expect("wait until time must be written after a submit with a timeout");

    let expected_time = config.start_time + Duration::minutes(6) + Duration::seconds(10);

    assert!(
        actual_time >= expected_time - Duration::seconds(1)
            && actual_time <= expected_time + Duration::seconds(1)
    );
}

// TODO: test `submit_answer` errors if session id not provided.
// TODO: test `submit_answer` errors if bad submission id (400).
// TODO: test `submit_answer` errors if invalid puzzle (404).
// TODO: test `submit_answer` for part 2 before part 1.
// TODO: test `submit_answer` arbitrary HTTP status error returned.
