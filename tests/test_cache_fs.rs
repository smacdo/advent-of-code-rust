use advent_of_code_data::{
    cache::{CacheError, PuzzleCache, PuzzleFsCache},
    data::{Answers, CheckResult},
    Answer, Day, Part, Year,
};
use tempfile::tempdir;

#[test]
fn load_encrypted_input() {
    let cache_dir = tempdir().unwrap();
    let passphrase = Some("TEST".to_string());

    // Write input data with a temporary client.
    {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
        puzzle_cache
            .save_input("testing\n-123+=", Day(1), Year(1987))
            .unwrap();

        puzzle_cache
            .save_input("this is day 10", Day(10), Year(1987))
            .unwrap();

        puzzle_cache
            .save_input("foobar", Day(3), Year(2000))
            .unwrap();
    }

    // Read input data with the same password.
    let read_cached_input = |day: Day, year: Year| {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
        puzzle_cache
            .load_input(day, year)
            .expect("no errors from load_input are expected")
            .expect("cache file is expected to exist")
    };

    assert_eq!(&read_cached_input(Day(1), Year(1987)), "testing\n-123+=");
    assert_eq!(&read_cached_input(Day(10), Year(1987)), "this is day 10");
    assert_eq!(&read_cached_input(Day(3), Year(2000)), "foobar");
}

#[test]
fn load_unencrypted_input() {
    let cache_dir = tempdir().unwrap();
    let passphrase: Option<String> = None;

    // Write input data without a password.
    {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data without a password.
    let read_cached_input = |day: Day, year: Year| {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
        puzzle_cache
            .load_input(day, year)
            .expect("no errors from load_input are expected")
    };

    assert_eq!(
        read_cached_input(Day(19), Year(2000)),
        Some("foobar".to_string())
    );
}

#[test]
fn load_input_returns_some_if_cache_exists() {
    let cache_dir = tempdir().unwrap();
    let passphrase = Some("TEST".to_string());

    // Write input data with a temporary client.
    {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data back with another temporary client.
    let read_cached_input = |day: Day, year: Year| {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
        puzzle_cache
            .load_input(day, year)
            .expect("no errors from load_input are expected")
    };

    assert_eq!(
        read_cached_input(Day(19), Year(2000)),
        Some("foobar".to_string())
    );
}

#[test]
fn load_input_returns_none_if_cache_not_found() {
    let cache_dir = tempdir().unwrap();
    let passphrase = Some("TEST".to_string());

    let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase.clone());
    let result = puzzle_cache
        .load_input(Day(1), Year(2023))
        .expect("no errors from load_input are expected");

    assert!(result.is_none())
}

#[test]
fn load_input_err_if_wrong_password() {
    let cache_dir = tempdir().unwrap();

    // Write encrypted input data with a temporary client.
    {
        let passphrase = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data with the wrong encryption password.
    let read_cached_input = |day: Day, year: Year| {
        let passphrase = Some("wrong password".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache.load_input(day, year)
    };

    assert!(matches!(
        read_cached_input(Day(19), Year(2000)),
        Err(CacheError::Decryption(_))
    ));
}

#[test]
fn load_input_err_if_password_missing() {
    let cache_dir = tempdir().unwrap();

    // Write encrypted input data with a temporary client.
    {
        let passphrase = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data without an encryption password.
    let read_cached_input = |day: Day, year: Year| {
        let passphrase: Option<String> = None;
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache.load_input(day, year)
    };

    assert!(matches!(
        read_cached_input(Day(19), Year(2000)),
        Err(CacheError::PassphraseRequired)
    ));
}

#[test]
fn load_input_err_if_password_provided_but_input_not_encrypted() {
    let cache_dir = tempdir().unwrap();

    // Write unencrypted input data with a temporary client.
    {
        let passphrase: Option<String> = None;
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data with an encryption password.
    let read_cached_input = |day: Day, year: Year| {
        let passphrase = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache.load_input(day, year)
    };

    assert!(matches!(
        read_cached_input(Day(19), Year(2000)),
        Err(CacheError::PassphraseNotNeeded)
    ));
}

#[test]
fn load_answers_cache_exists() {
    let cache_dir = tempdir().unwrap();

    // Save answers for part one to cache.
    {
        let passphrase: Option<String> = None;
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        let mut answers = Answers::new();

        answers.set_correct_answer(Answer::Int(42));
        answers.add_wrong_answer(Answer::from("nope"));

        puzzle_cache
            .save_answers(&answers, Part::One, Day(19), Year(2000))
            .expect("no errors expected when saving answers to cache")
    }

    // Save answers for part two to cache.
    {
        let passphrase: Option<String> = None;
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        let mut answers = Answers::new();

        answers.set_correct_answer(Answer::from("hello world"));
        answers.set_high_bounds(Answer::Int(100000));
        answers.add_wrong_answer(Answer::Int(12345));

        puzzle_cache
            .save_answers(&answers, Part::Two, Day(19), Year(2000))
            .expect("no errors expected when saving answers to cache")
    }

    // Load answers back from cache.
    let read_cached_input = |part: Part, day: Day, year: Year| {
        let passphrase = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache
            .load_answers(part, day, year)
            .expect("errors not expected when loading answers from cache")
    };

    let answers_part_one = read_cached_input(Part::One, Day(19), Year(2000))
        .expect("answers for part one is expected to be in the cache");

    assert_eq!(
        answers_part_one.check(&Answer::Int(42)),
        Some(CheckResult::Correct)
    );
    assert_eq!(
        answers_part_one.check(&Answer::from("nope")),
        Some(CheckResult::Wrong)
    );

    let answers_part_two = read_cached_input(Part::Two, Day(19), Year(2000))
        .expect("answers for part two is expected to be in the cache");

    assert_eq!(
        answers_part_two.check(&Answer::from("hello world")),
        Some(CheckResult::Correct)
    );
    assert_eq!(
        answers_part_two.check(&Answer::Int(100000)),
        Some(CheckResult::TooHigh)
    );
    assert_eq!(
        answers_part_two.check(&Answer::Int(12345)),
        Some(CheckResult::Wrong)
    );
}

#[test]
fn load_answers_returns_none_if_cache_missing() {
    let cache_dir = tempdir().unwrap();

    // Load answers back from cache.
    let read_cached_input = |part: Part, day: Day, year: Year| {
        let passphrase = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), passphrase);
        puzzle_cache
            .load_answers(part, day, year)
            .expect("errors not expected when loading answers from cache")
    };

    assert!(read_cached_input(Part::One, Day(19), Year(2000)).is_none());
    assert!(read_cached_input(Part::Two, Day(19), Year(2000)).is_none());
}
