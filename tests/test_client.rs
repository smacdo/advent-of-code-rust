use std::{cell::RefCell, path::PathBuf, rc::Rc};

use advent_of_code_data::{
    aoc_service::{ServiceConnector, ServiceError},
    cache::{PuzzleCache, PuzzleFsCache, SessionCache, SessionFsCache},
    client::{Client, ClientError, WebClient},
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

fn write_answers(config: &Config, answers: &Answers, part: Part, day: Day, year: Year) {
    PuzzleFsCache::new(
        config.puzzle_dir.clone(),
        Some(config.passphrase.to_string()),
    )
    .save_answers(answers, part, day, year)
    .unwrap()
}

fn get_cached_input(config: &Config, day: Day, year: Year) -> Option<String> {
    let puzzle_cache = PuzzleFsCache::new(
        config.puzzle_dir.clone(),
        Some(config.passphrase.to_string()),
    );

    puzzle_cache.load_input(day, year).unwrap()
}

fn write_input(config: &Config, input: &str, day: Day, year: Year) {
    PuzzleFsCache::new(
        config.puzzle_dir.clone(),
        Some(config.passphrase.to_string()),
    )
    .save_input(input, day, year)
    .unwrap()
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
fn get_input_calls_endpoint() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Create mock get_input that records parameters.
    struct MockArgs {
        day: Day,
        year: Year,
        session: String,
    }

    let mock_args: Rc<RefCell<Option<MockArgs>>> = Rc::new(RefCell::new(None));
    let mock_args_clone = mock_args.clone();

    let client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(move |day, year, session| -> Result<String, ServiceError> {
                mock_args_clone.replace(Some(MockArgs {
                    day,
                    year,
                    session: session.to_string(),
                }));
                Ok("hello world".to_string())
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    // Get input and validate that the endpoint was called with expected arguments.
    assert!(client.get_input(Day(1), Year(2000)).is_ok());

    let args = mock_args.take().unwrap();

    assert_eq!(args.day, Day(1));
    assert_eq!(args.year, Year(2000));
    assert_eq!(&args.session, &config.session_id.clone().unwrap());

    // Get input with different arguments and check everything is still working as expected.
    assert!(client.get_input(Day(13), Year(2008)).is_ok());

    let args = mock_args.take().unwrap();

    assert_eq!(args.day, Day(13));
    assert_eq!(args.year, Year(2008));
    assert_eq!(&args.session, &config.session_id.unwrap());
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

#[test]
fn get_input_skips_cache_if_answer_in_cache() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);
    write_input(&config, "testing 123", Day(12), Year(1812));

    let client = WebClient::with_custom_impl(
        config,
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    assert_eq!(
        &client.get_input(Day(12), Year(1812)).unwrap(),
        "testing 123"
    );
}

#[test]
fn get_input_writes_to_cache() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    let client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Ok("hello world".to_string())
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    // Check that there is no cache prior to calling get_input.
    assert!(
        get_cached_input(&config, Day(12), Year(1812)).is_none(),
        "there should be no cached input before get_input"
    );

    // Verify that the input is cached after calling get_input.
    assert_eq!(
        &client.get_input(Day(12), Year(1812)).unwrap(),
        "hello world"
    );

    if let Some(input) = get_cached_input(&config, Day(12), Year(1812)) {
        assert_eq!(&input, "hello world");
    } else {
        panic!("expected answer cache to exist after submit_answer");
    }
}

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
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Err(ServiceError::HttpStatusError(400))
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    match client.get_input(Day(1), Year(2000)) {
        Err(ClientError::BadSessionId(session)) => {
            assert_eq!(session, config.session_id.unwrap());
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}

#[test]
fn get_input_not_found_err_if_http_404() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("sssion123"), &temp_dir);

    let client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Err(ServiceError::HttpStatusError(404))
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    match client.get_input(Day(23), Year(1992)) {
        Err(ClientError::PuzzleNotFound(day, year)) => {
            assert_eq!(day, Day(23));
            assert_eq!(year, Year(1992));
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}

#[test]
fn get_input_other_http_err() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("sssion123"), &temp_dir);

    let client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                Err(ServiceError::HttpStatusError(418))
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    match client.get_input(Day(23), Year(1992)) {
        Err(ClientError::ServerHttpError(status_code)) => {
            assert_eq!(status_code, 418);
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}

#[test]
fn submit_answer_calls_endpoint() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Create mock submit_answer that records parameters.
    struct MockArgs {
        answer: Answer,
        part: Part,
        day: Day,
        year: Year,
        session: String,
    }

    let mock_args: Rc<RefCell<Option<MockArgs>>> = Rc::new(RefCell::new(None));
    let mock_args_clone = mock_args.clone();

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(move |answer, part, day, year, session| {
                mock_args_clone.replace(Some(MockArgs {
                    answer: answer.clone(),
                    part,
                    day,
                    year,
                    session: session.to_string(),
                }));

                Ok("That's the right answer! You are one star closer".to_string())
            }),
        }),
    );

    // Submit the answer and validate that the endpoint was called with expected arguments.
    assert!(client
        .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
        .is_ok());

    let args = mock_args.take().unwrap();

    assert_eq!(args.answer, Answer::Int(42));
    assert_eq!(args.part, Part::One);
    assert_eq!(args.day, Day(1));
    assert_eq!(args.year, Year(2000));
    assert_eq!(&args.session, &config.session_id.clone().unwrap());

    // Submit the answer with different arguments and check everything is still working as expected.
    assert!(client
        .submit_answer(
            Answer::String("hello".to_string()),
            Part::Two,
            Day(13),
            Year(2008)
        )
        .is_ok());

    let args = mock_args.take().unwrap();

    assert_eq!(args.answer, Answer::String("hello".to_string()));
    assert_eq!(args.part, Part::Two);
    assert_eq!(args.day, Day(13));
    assert_eq!(args.year, Year(2008));
    assert_eq!(&args.session, &config.session_id.unwrap());
}

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
fn submit_uses_answer_cache() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Write a wrong answer to the cache.
    let mut answers = Answers::new();
    answers.add_wrong_answer(Answer::Int(42));

    write_answers(&config, &answers, Part::One, Day(17), Year(2012));

    // Submit a wrong answer and verify the service backend is never called.
    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(17), Year(2012))
            .unwrap(),
        CheckResult::Wrong
    );
}

#[test]
fn submit_uses_service_if_cache_missing_answer() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("session123"), &temp_dir);

    // Write a correct answer to the cache.
    let mut answers = Answers::new();
    answers.set_correct_answer(Answer::Int(42));

    write_answers(&config, &answers, Part::One, Day(17), Year(2012));

    // Submit a correct answer and verify the service backend is called.
    let was_called: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let was_called_clone = was_called.clone();

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(move |_answer, _part, _day, _year, _session| {
                was_called_clone.replace(true);
                Ok("That's the right answer! You are one star closer".to_string())
            }),
        }),
    );

    assert_eq!(
        client
            .submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000))
            .unwrap(),
        CheckResult::Correct
    );
    assert!(was_called.take());
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

#[test]
fn submit_answer_err_if_no_session() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(None, &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| unimplemented!()),
        }),
    );

    // Check submit_answer returns expected response.
    assert!(matches!(
        client.submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000)),
        Err(ClientError::SessionIdRequired)
    ));
}

#[test]
fn submit_answer_bad_session_err_if_http_400() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("sssion123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Err(ServiceError::HttpStatusError(400))
            }),
        }),
    );

    // Check submit_answer returns expected response.
    match client.submit_answer(Answer::Int(42), Part::One, Day(1), Year(2000)) {
        Err(ClientError::BadSessionId(session)) => {
            assert_eq!(session, config.session_id.unwrap());
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}

#[test]
fn submit_answer_not_found_err_if_http_404() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("sssion123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Err(ServiceError::HttpStatusError(404))
            }),
        }),
    );

    // Check submit_answer returns expected response.
    match client.submit_answer(Answer::Int(42), Part::One, Day(23), Year(1992)) {
        Err(ClientError::PuzzleNotFound(day, year)) => {
            assert_eq!(day, Day(23));
            assert_eq!(year, Year(1992));
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}

#[test]
fn submit_answer_other_http_err() {
    let temp_dir = tempdir().unwrap();
    let config = make_test_config(Some("sssion123"), &temp_dir);

    let mut client = WebClient::with_custom_impl(
        config.clone(),
        Box::new(TestAdventOfCodeService {
            mock_get_input: Box::new(|_day, _year, _session| -> Result<String, ServiceError> {
                unimplemented!()
            }),
            mock_submit_answer: Box::new(|_answer, _part, _day, _year, _session| {
                Err(ServiceError::HttpStatusError(418))
            }),
        }),
    );

    // Check submit_answer returns expected response.
    match client.submit_answer(Answer::Int(42), Part::One, Day(23), Year(1992)) {
        Err(ClientError::ServerHttpError(status_code)) => {
            assert_eq!(status_code, 418);
        }
        x => {
            panic!("incorrect result {:?}", x);
        }
    }
}
