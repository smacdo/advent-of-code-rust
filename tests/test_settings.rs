use std::{io::Write, path::PathBuf, str::FromStr};

use advent_of_code_data::config::{read_config_from_env_vars, read_config_from_file};
use tempfile::NamedTempFile;

#[test]
fn config_with_env_vars() {
    std::env::set_var("AOC_SESSION", "my_session");
    std::env::set_var("AOC_PASSPHRASE", "my_password");
    std::env::set_var("AOC_PUZZLE_DIR", "/tmp/puzzles");
    std::env::set_var("AOC_SESSIONS_DIR", "/tmp/foo/bar");

    let config = read_config_from_env_vars(None);

    assert_eq!(config.session_id, Some("my_session".to_string()));
    assert_eq!(config.passphrase, Some("my_password".to_string()));
    assert_eq!(
        config.puzzle_dir,
        Some(PathBuf::from_str("/tmp/puzzles").unwrap())
    );
    assert_eq!(
        config.sessions_dir,
        Some(PathBuf::from_str("/tmp/foo/bar").unwrap())
    );
}

#[test]
fn config_from_toml_file() {
    let config_text = r#"
        [client]
        session_id = "12345"
        puzzle_dir = "path/to/puzzle/dir"
        sessions_dir = "another/path/to/blah"
        passphrase = "foobar"
        "#;

    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", config_text).unwrap();

    let config = read_config_from_file(None, tmpfile.path()).unwrap();

    assert_eq!(config.session_id, Some("12345".to_string()));
    assert_eq!(
        config.puzzle_dir,
        Some(PathBuf::from_str("path/to/puzzle/dir").unwrap())
    );
    assert_eq!(
        config.sessions_dir,
        Some(PathBuf::from_str("another/path/to/blah").unwrap())
    );
    assert_eq!(config.passphrase, Some("foobar".to_string()));
}
