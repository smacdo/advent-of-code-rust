use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

// TODO: Switch local directory config name to .advent_of_code_data.toml
// TODO: Also switch the example file.

const DIRS_QUALIFIER: &str = "com";
const DIRS_ORG: &str = "smacdo";
const DIRS_APP: &str = "advent_of_code_data";

const EXAMPLE_CONFIG_TEXT: &str = r#"[client]
# session_id = "REPLACE_ME"
# encryption_token = "REPLACE_ME"
"#;

pub struct ClientOptions {
    pub session_id: Option<String>,
    pub puzzle_dir: Option<PathBuf>,
    pub user_cache_dir: Option<PathBuf>,
    pub encryption_token: Option<String>,
    pub fake_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl ClientOptions {
    pub fn new() -> Self {
        Self {
            session_id: None,
            puzzle_dir: None,
            user_cache_dir: None,
            encryption_token: None,
            fake_time: None,
        }
    }

    pub fn with_config_file<P: AsRef<Path>>(self, path: P) -> Self {
        // TODO: return errors instead of panicing
        // TODO: add tests
        let config_text = fs::read_to_string(&path).expect("config file should exist");
        self.with_toml_config(&config_text)
            .expect("toml parsing failed")
    }

    pub fn with_local_dir_config(mut self) -> Self {
        // TODO: add tests
        let local_config_path = std::env::current_dir()
            .expect("current_dir is expected to work")
            .join("aoc_settings.toml");

        if local_config_path.exists() {
            tracing::debug!("loading config values from: {local_config_path:?}");
            self = self.with_config_file(local_config_path);
        } else {
            tracing::debug!("local dir config not found: {local_config_path:?}")
        }

        self
    }

    pub fn with_user_config(mut self) -> Self {
        // TODO: add tests

        // Read local application configuration.
        if let Some(project_dir) =
            directories::ProjectDirs::from(DIRS_QUALIFIER, DIRS_ORG, DIRS_APP)
        {
            const EXAMPLE_FILE_NAME: &str = "config.example.toml";
            const CONFIG_FILE_NAME: &str = "config.toml";

            let config_dir = project_dir.config_dir();
            let example_config_path = config_dir.join(EXAMPLE_FILE_NAME);

            // Create the application's config dir if its missing.
            if !std::fs::exists(config_dir).unwrap_or(false) {
                std::fs::create_dir_all(config_dir).unwrap_or_else(|e| {
                    tracing::debug!("failed to create app config dir: {e:?}");
                });
            }

            // Create an example config file in the user config dir to help
            // users get started.
            if !std::fs::exists(&example_config_path).unwrap_or(false) {
                tracing::debug!("created example config at {example_config_path:?}");

                std::fs::write(example_config_path, EXAMPLE_CONFIG_TEXT).unwrap_or_else(|e| {
                    tracing::debug!("failed to create example config: {e:?}");
                });
            }

            // Load the user config if it exists.
            let config_path = config_dir.join(CONFIG_FILE_NAME);

            if std::fs::exists(&config_path).unwrap_or(false) {
                tracing::debug!("loading user config at: {config_path:?}");
                self = self.with_config_file(config_path);
            } else {
                tracing::debug!("no user config found at: {config_path:?}");
            }
        } else {
            tracing::debug!("could not calculate user config dir on this machine");
        }

        // Read home directory.
        if let Some(base_dirs) = directories::BaseDirs::new() {
            const HOME_CONFIG_NAME: &str = ".advent_of_code_data.toml";
            let home_config_path = base_dirs.home_dir().join(HOME_CONFIG_NAME);

            if std::fs::exists(&home_config_path).unwrap_or(false) {
                tracing::debug!("loading user config at: {home_config_path:?}");
                self = self.with_config_file(home_config_path);
            } else {
                tracing::debug!("no user config found at: {home_config_path:?}");
            }
        }

        // Read custom configuration path from `AOC_CONFIG_PATH`.
        const CUSTOM_CONFIG_ENV_KEY: &str = "AOC_CONFIG_PATH";

        if let Ok(custom_config_path) = std::env::var(CUSTOM_CONFIG_ENV_KEY) {
            if std::fs::exists(&custom_config_path).unwrap_or(false) {
                tracing::debug!("loading user config at: {custom_config_path:?}");
                self = self.with_config_file(custom_config_path);
            } else {
                tracing::debug!("no user config found at: {custom_config_path:?}");
            }
        } else {
            tracing::debug!(
                "skipping custom user config because env var `{CUSTOM_CONFIG_ENV_KEY}` is not set"
            );
        }

        self
    }

    /// Use the local user's cache directory as the storage location for user
    /// data caching.
    pub fn with_user_cache(self) -> Self {
        // TODO: implement XDG lookup.
        // TODO: check if env variable has changed the user cache dir.
        // TODO: log if file is found/not found
        // TODO: add tests
        self
    }

    pub fn with_env_vars(mut self) -> Self {
        // TODO: add key for custom puzzle cache directory.
        const SESSION_ID_ENV_KEY: &str = "AOC_SESSION";
        const PASSWORD_ENV_KEY: &str = "AOC_PASSWORD";

        fn try_read_env_var<F: FnOnce(String)>(name: &str, setter: F) {
            if let Ok(v) = std::env::var(name) {
                tracing::debug!("found env var `{name}` with value `{v}`");
                setter(v)
            }
        }

        try_read_env_var(SESSION_ID_ENV_KEY, |v| {
            self.session_id = Some(v);
        });

        try_read_env_var(PASSWORD_ENV_KEY, |v| {
            self.encryption_token = Some(v);
        });

        self
    }

    pub fn with_toml_config(mut self, config_text: &str) -> Result<Self, toml::de::Error> {
        const CLIENT_TABLE_NAME: &str = "client";
        const SESSION_ID_KEY: &str = "session_id";
        const PUZZLE_DIR_KEY: &str = "puzzle_dir";
        const ENCRYPTION_TOKEN_KEY: &str = "encryption_token";
        const REPLACE_ME: &str = "REPLACE_ME";

        fn try_read_key<F: FnOnce(&str)>(table: &toml::Table, key: &str, setter: F) {
            match table.get(key).as_ref() {
                Some(toml::Value::String(s)) => {
                    if s == REPLACE_ME {
                        tracing::debug!("ignoring TOML key {key} because value is `{REPLACE_ME}`");
                    } else {
                        tracing::debug!("found TOML key `{key}` with value `{s}`");
                        setter(s)
                    }
                }
                None => {
                    tracing::debug!("TOML key {key} not present, or its value was not a string");
                }
                _ => {
                    tracing::warn!("TOML key {key} must be string value");
                }
            };
        }

        let toml: toml::Table = config_text.parse::<toml::Table>()?;

        match toml.get(CLIENT_TABLE_NAME) {
            Some(toml::Value::Table(client_config)) => {
                try_read_key(client_config, SESSION_ID_KEY, |v| {
                    self.session_id = Some(v.to_string())
                });

                try_read_key(client_config, PUZZLE_DIR_KEY, |v| {
                    self.puzzle_dir = Some(PathBuf::from_str(v).unwrap())
                });

                try_read_key(client_config, ENCRYPTION_TOKEN_KEY, |v| {
                    self.encryption_token = Some(v.to_string())
                });
            }
            _ => {
                tracing::debug!(
                    "TOML table {CLIENT_TABLE_NAME} not found, no config keys will be loaded"
                );
            }
        }

        Ok(self)
    }

    pub fn with_session_id<S: Into<String>>(mut self, session_id: S) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_puzzle_dir<P: Into<PathBuf>>(mut self, puzzle_dir: P) -> Self {
        self.puzzle_dir = Some(puzzle_dir.into());
        self
    }

    pub fn with_encryption_token<S: Into<String>>(mut self, encryption_token: S) -> Self {
        self.encryption_token = Some(encryption_token.into());
        self
    }

    pub fn with_fake_time(mut self, fake_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.fake_time = Some(fake_time);
        self
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self::new()
            .with_user_config()
            .with_user_cache()
            .with_local_dir_config()
            .with_env_vars()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_options_are_none_by_default() {
        let options = ClientOptions::new();
        assert!(options.session_id.is_none());
        assert!(options.puzzle_dir.is_none());
        assert!(options.encryption_token.is_none());
    }

    #[test]
    fn client_can_overwrite_options() {
        let options = ClientOptions::new()
            .with_encryption_token("12345")
            .with_encryption_token("54321");

        assert!(options.session_id.is_none());
        assert!(options.puzzle_dir.is_none());
        assert_eq!(options.encryption_token, Some("54321".to_string()));
    }

    #[test]
    fn set_client_options_with_builder_funcs() {
        let options = ClientOptions::new()
            .with_session_id("MY_SESSION_ID")
            .with_puzzle_dir("MY_CACHE_DIR")
            .with_encryption_token("MY_PASSWORD");

        assert_eq!(options.session_id, Some("MY_SESSION_ID".to_string()));
        assert_eq!(
            options.puzzle_dir,
            Some(PathBuf::from_str("MY_CACHE_DIR").unwrap())
        );
        assert_eq!(options.encryption_token, Some("MY_PASSWORD".to_string()));
    }

    #[test]
    fn set_client_options_from_toml() {
        let config_text = r#"
        [client]
        session_id = "12345"
        puzzle_dir = "path/to/puzzle/dir"
        encryption_token = "foobar"
        "#;

        let options = ClientOptions::new().with_toml_config(config_text).unwrap();

        assert_eq!(options.session_id, Some("12345".to_string()));
        assert_eq!(
            options.puzzle_dir,
            Some(PathBuf::from_str("path/to/puzzle/dir").unwrap())
        );
        assert_eq!(options.encryption_token, Some("foobar".to_string()));
    }

    #[test]
    fn set_client_options_from_toml_ignores_missing_fields() {
        let config_text = r#"
        [client]
        session_id = "12345"
        encryption_token_XXXX = "foobar"
        "#;

        let options = ClientOptions::new().with_toml_config(config_text).unwrap();

        assert_eq!(options.session_id, Some("12345".to_string()));
        assert!(options.puzzle_dir.is_none());
        assert!(options.encryption_token.is_none());
    }

    #[test]
    fn set_client_options_from_toml_ignores_replace_me_values() {
        let config_text = r#"
        [client]
        session_id = "REPLACE_ME"
        puzzle_dir = "path/to/puzzle/dir"
        encryption_token = "REPLACE_ME"
        "#;

        let options = ClientOptions::new().with_toml_config(config_text).unwrap();

        assert!(options.session_id.is_none());
        assert!(options.encryption_token.is_none());
        assert_eq!(
            options.puzzle_dir,
            Some(PathBuf::from_str("path/to/puzzle/dir").unwrap())
        );
    }
}
