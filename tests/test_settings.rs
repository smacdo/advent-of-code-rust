use advent_of_code_data::config::read_config_from_env_vars;

#[test]
fn config_with_env_vars() {
    std::env::set_var("AOC_SESSION", "my_session");
    std::env::set_var("AOC_PASSPHRASE", "my_password");

    let options = read_config_from_env_vars(None);

    assert_eq!(options.session_id, Some("my_session".to_string()));
    assert_eq!(options.passphrase, Some("my_password".to_string()));
}
