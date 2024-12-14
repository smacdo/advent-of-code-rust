use advent_of_code_data::{
    cache::{CacheError, PuzzleCache, PuzzleFsCache},
    Day, Year,
};
use tempfile::tempdir;

#[test]
fn persist_puzzle_input() {
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

    // Read input data back with another temporary client.
    let read_cached_input = |day: Day, year: Year| {
        let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
        puzzle_cache.load_input(day, year).unwrap()
    };

    assert_eq!(&read_cached_input(Day(1), Year(1987)), "testing\n-123+=");
    assert_eq!(&read_cached_input(Day(10), Year(1987)), "this is day 10");
    assert_eq!(&read_cached_input(Day(3), Year(2000)), "foobar");
}

#[test]
fn try_load_input_exists() {
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
        puzzle_cache.try_load_input(day, year).unwrap()
    };

    assert_eq!(
        read_cached_input(Day(19), Year(2000)),
        Some("foobar".to_string())
    );
}

#[test]
fn load_missing_puzzle_input() {
    let cache_dir = tempdir().unwrap();
    let encryption_token = Some("TEST".to_string());

    let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
    let result = puzzle_cache.load_input(Day(1), Year(2023));

    assert!(result.is_err());
    assert!(matches!(result, Err(CacheError::Io(_))));
}

#[test]
fn try_load_missing_puzzle_input() {
    let cache_dir = tempdir().unwrap();
    let encryption_token = Some("TEST".to_string());

    let puzzle_cache = PuzzleFsCache::new(cache_dir.path(), encryption_token.clone());
    let result = puzzle_cache.try_load_input(Day(1), Year(2023)).unwrap();

    assert!(result.is_none())
}
