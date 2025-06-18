use advent_of_code_data::{
    cache::{CacheError, PuzzleCache, PuzzleFsCache},
    Day, Year,
};
use tempfile::tempdir;

#[test]
fn load_encrypted_input() {
    let cache_dir = tempdir().unwrap();
    let encryption_token = Some("TEST".to_string());

    // Write input data with a temporary client.
    {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
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
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
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
    let encryption_token: Option<String> = None;

    // Write input data without a password.
    {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data without a password.
    let read_cached_input = |day: Day, year: Year| {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
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
    let encryption_token = Some("TEST".to_string());

    // Write input data with a temporary client.
    {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data back with another temporary client.
    let read_cached_input = |day: Day, year: Year| {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
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
    let encryption_token = Some("TEST".to_string());

    let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
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
        let encryption_token = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token);
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data with the wrong encryption password.
    let read_cached_input = |day: Day, year: Year| {
        let encryption_token = Some("wrong password".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token);
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
        let encryption_token = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token);
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data without an encryption password.
    let read_cached_input = |day: Day, year: Year| {
        let encryption_token: Option<String> = None;
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token);
        puzzle_cache.load_input(day, year)
    };

    assert!(matches!(
        read_cached_input(Day(19), Year(2000)),
        Err(CacheError::EncryptionTokenNotSet)
    ));
}

#[test]
fn load_input_err_if_password_provided_but_input_not_encrypted() {
    let cache_dir = tempdir().unwrap();

    // Write unencrypted input data with a temporary client.
    {
        let encryption_token: Option<String> = None;
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token);
        puzzle_cache
            .save_input("foobar", Day(19), Year(2000))
            .unwrap();
    }

    // Read input data with an encryption password.
    let read_cached_input = |day: Day, year: Year| {
        let encryption_token = Some("TEST".to_string());
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token);
        puzzle_cache.load_input(day, year)
    };

    assert!(matches!(
        read_cached_input(Day(19), Year(2000)),
        Err(CacheError::EncryptionTokenNotNeeded)
    ));
}
