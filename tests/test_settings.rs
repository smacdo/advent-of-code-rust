use std::{path::PathBuf, str::FromStr};

use advent_of_code_data::config::read_config_from_env_vars;

#[test]
fn config_with_env_vars() {
    std::env::set_var("AOC_SESSION", "my_session");
    std::env::set_var("AOC_PASSPHRASE", "my_password");
    std::env::set_var("AOC_PUZZLE_DIR", "/tmp/puzzles");
    std::env::set_var("AOC_SESSIONS_DIR", "/tmp/foo/bar");

    let options = read_config_from_env_vars(None);

    assert_eq!(options.session_id, Some("my_session".to_string()));
    assert_eq!(options.passphrase, Some("my_password".to_string()));
    assert_eq!(
        options.puzzle_dir,
        Some(PathBuf::from_str("/tmp/puzzles").unwrap())
    );
    assert_eq!(
        options.sessions_dir,
        Some(PathBuf::from_str("/tmp/foo/bar").unwrap())
    );
}
