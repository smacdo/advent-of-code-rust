use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use base64::{prelude::BASE64_STANDARD, Engine};
use core::str;
use simple_crypt::{decrypt, encrypt};
use thiserror::Error;

use crate::{
    data::{Answers, Puzzle, User},
    Day, Part, Year,
};

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("AOC encryption token expected but not provided (check your config)")]
    EncryptionTokenNotSet,
    #[error("Cached input file is not encrypted but encryption token was provided")]
    EncryptionTokenNotNeeded,
    #[error("base 64 decoding failed: {}", .0)]
    DecodeBase64(#[from] base64::DecodeError),
    #[error("decryption failed: {}", .0)]
    Decryption(#[source] anyhow::Error),
    #[error("encryption failed: {}", .0)]
    Encryption(#[source] anyhow::Error),
    #[error("decoding utf8 failed: {}", .0)]
    DecodeUtf8(#[from] std::string::FromUtf8Error),
    #[error("serializing or deserializing failed: {}", .0)]
    JsonSerde(#[from] serde_json::Error),
    #[error("a file i/o error occured while reading/writing the cache: {}", .0)]
    Io(#[from] std::io::Error),
}

pub trait PuzzleCache: Debug {
    fn try_load_input(&self, day: Day, year: Year) -> Result<Option<String>, CacheError>;

    fn load_input(&self, day: Day, year: Year) -> Result<String, CacheError>;

    fn load_answers(&self, part: Part, day: Day, year: Year) -> Result<Answers, CacheError>;

    fn load_puzzle(&self, day: Day, year: Year) -> Result<Puzzle, CacheError>;

    fn save(&self, puzzle: Puzzle) -> Result<(), CacheError> {
        self.save_input(&puzzle.input, puzzle.day, puzzle.year)?;
        self.save_answers(&puzzle.part_one_answers, Part::One, puzzle.day, puzzle.year)?;
        self.save_answers(&puzzle.part_two_answers, Part::Two, puzzle.day, puzzle.year)?;

        Ok(())
    }

    fn save_input(&self, input: &str, day: Day, year: Year) -> Result<(), CacheError>;

    fn save_answers(
        &self,
        answers: &Answers,
        part: Part,
        day: Day,
        year: Year,
    ) -> Result<(), CacheError>;
}

pub trait UserDataCache: Debug {
    fn load(&self, session_id: &str) -> Result<User, CacheError>;
    fn save(&self, user: &User) -> Result<(), CacheError>;
}

#[derive(Debug)]
pub struct PuzzleFsCache {
    cache_dir: PathBuf,
    encryption_token: Option<String>,
}

impl PuzzleFsCache {
    const INPUT_FILE_NAME: &'static str = "input.txt";
    const ENCRYPTED_INPUT_FILE_NAME: &'static str = "input.encrypted.txt";
    const PART_ONE_ANSWERS_FILE_NAME: &'static str = "part-1-answers.txt";
    const PART_TWO_ANSWERS_FILE_NAME: &'static str = "part-2-answers.txt";

    pub fn new<P: Into<PathBuf>, S: Into<String>>(
        cache_dir: P,
        encryption_token: Option<S>,
    ) -> Self {
        // TODO: Validate cache_dir is a directory, and is writable.
        Self {
            cache_dir: cache_dir.into(),
            encryption_token: encryption_token.map(|x| x.into()),
        }
    }

    pub fn dir_for_puzzle(cache_dir: &Path, day: Day, year: Year) -> PathBuf {
        cache_dir.join(format!("y{}", year)).join(day.to_string())
    }

    pub fn input_file_path(cache_dir: &Path, day: Day, year: Year, encrypted: bool) -> PathBuf {
        Self::dir_for_puzzle(cache_dir, day, year).join(if encrypted {
            Self::ENCRYPTED_INPUT_FILE_NAME
        } else {
            Self::INPUT_FILE_NAME
        })
    }

    pub fn answers_file_path(cache_dir: &Path, part: Part, day: Day, year: Year) -> PathBuf {
        Self::dir_for_puzzle(cache_dir, day, year).join(match part {
            Part::One => Self::PART_ONE_ANSWERS_FILE_NAME,
            Part::Two => Self::PART_TWO_ANSWERS_FILE_NAME,
        })
    }
}

impl PuzzleCache for PuzzleFsCache {
    fn try_load_input(&self, day: Day, year: Year) -> Result<Option<String>, CacheError> {
        match self.load_input(day, year) {
            Ok(input) => Ok(Some(input)),
            Err(CacheError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn load_input(&self, day: Day, year: Year) -> Result<String, CacheError> {
        // Load the cached input file from disk.
        let using_encryption = self.encryption_token.is_some();
        let input_path = Self::input_file_path(&self.cache_dir, day, year, using_encryption);

        // Check for the existence or non-existence of the input file, and hint
        // to the user if encryption seems to be incorrectly configured.
        if !std::fs::exists(&input_path)? {
            let alt_input_path =
                Self::input_file_path(&self.cache_dir, day, year, !using_encryption);

            match (
                std::fs::exists(&alt_input_path).unwrap_or(false),
                using_encryption,
            ) {
                (true, false) => return Err(CacheError::EncryptionTokenNotSet),
                (true, true) => return Err(CacheError::EncryptionTokenNotNeeded),
                _ => {}
            }
        }

        // Read the input file.
        tracing::debug!("loading input for day {day} year {year} from {input_path:?}");
        let mut input_text = std::fs::read_to_string(input_path)?;

        // Decrypt the input data.
        if let Some(encryption_token) = &self.encryption_token {
            let encrypted_bytes = BASE64_STANDARD
                .decode(input_text.as_bytes())
                .map_err(CacheError::DecodeBase64)?;
            let input_bytes = decrypt(&encrypted_bytes, encryption_token.as_bytes())
                .map_err(CacheError::Decryption)?;
            input_text = String::from_utf8(input_bytes).map_err(CacheError::DecodeUtf8)?;

            tracing::debug!("succesfully decrypted input for puzzle day {day} year {year}")
        }

        Ok(input_text)
    }

    fn load_answers(&self, part: Part, day: Day, year: Year) -> Result<Answers, CacheError> {
        Ok(Answers::deserialize_from_str(&std::fs::read_to_string(
            Self::answers_file_path(&self.cache_dir, part, day, year),
        )?))
    }

    fn load_puzzle(&self, day: Day, year: Year) -> Result<Puzzle, CacheError> {
        // TODO: Create default input or answers if the files don't exist.
        Ok(Puzzle {
            day,
            year,
            input: self.load_input(day, year)?,
            part_one_answers: self.load_answers(Part::One, day, year)?,
            part_two_answers: self.load_answers(Part::Two, day, year)?,
        })
    }

    fn save(&self, puzzle: Puzzle) -> Result<(), CacheError> {
        // Create the puzzle directory in the cache if it doesn't already exist.
        let puzzle_dir = Self::dir_for_puzzle(&self.cache_dir, puzzle.day, puzzle.year);
        std::fs::create_dir_all(puzzle_dir)?;

        self.save_input(&puzzle.input, puzzle.day, puzzle.year)?;
        self.save_answers(&puzzle.part_one_answers, Part::One, puzzle.day, puzzle.year)?;
        self.save_answers(&puzzle.part_two_answers, Part::Two, puzzle.day, puzzle.year)?;

        Ok(())
    }

    fn save_input(&self, input: &str, day: Day, year: Year) -> Result<(), CacheError> {
        // Calculate the path to the puzzle's input file.
        let input_path =
            Self::input_file_path(&self.cache_dir, day, year, self.encryption_token.is_some());

        // Create puzzle directory if it does not already exist.
        let mut puzzle_dir = input_path.clone();
        puzzle_dir.pop();

        std::fs::create_dir_all(puzzle_dir)?;

        // Write the input to disk, and optionally encrypt the input file when
        // stored on disk.
        if let Some(encryption_token) = &self.encryption_token {
            // Encrypt then base64 encode for better version control handling.
            let encrypted_data = encrypt(input.as_bytes(), encryption_token.as_bytes())
                .map_err(CacheError::Encryption)?;
            let b64_encrypted_text = BASE64_STANDARD.encode(encrypted_data);

            tracing::debug!("saving encrypted input for day {day} year {year} to {input_path:?}");
            Ok(std::fs::write(input_path, b64_encrypted_text)?)
        } else {
            // No encryption.
            tracing::debug!("saving unencrypted input for day {day} year {year} to {input_path:?}");
            Ok(std::fs::write(input_path, input)?)
        }
    }

    fn save_answers(
        &self,
        answers: &Answers,
        part: Part,
        day: Day,
        year: Year,
    ) -> Result<(), CacheError> {
        let answers_path = Self::answers_file_path(&self.cache_dir, part, day, year);

        // Create the puzzle directory if it doesn't already exist.
        let mut puzzle_dir = answers_path.clone();
        puzzle_dir.pop();

        std::fs::create_dir_all(puzzle_dir)?;

        tracing::debug!("saving answer for part {part} day {day} year {year} to {answers_path:?}");
        Ok(std::fs::write(answers_path, answers.serialize_to_string())?)
    }
}

#[derive(Debug)]
pub struct UserDataFsCache {
    cache_dir: PathBuf,
}

impl UserDataFsCache {
    pub fn new<P: Into<PathBuf>>(cache_dir: P) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    pub fn path_for_user_data(&self, session_id: &str) -> PathBuf {
        // TODO: Validate session_id is safe for filename.
        self.cache_dir.join(format!("{}.json", session_id))
    }
}

impl UserDataCache for UserDataFsCache {
    fn load(&self, session_id: &str) -> Result<User, CacheError> {
        let user_file_path = self.path_for_user_data(session_id);

        if user_file_path.is_file() {
            tracing::debug!("cached user data for {session_id} at `{user_file_path:?}`");

            let json_text = std::fs::read_to_string(user_file_path)?;
            let user: User = serde_json::from_str(&json_text)?;

            Ok(user)
        } else {
            tracing::debug!("no cached user data for {session_id} at `{user_file_path:?}`, returning new User object");
            Ok(User::new(session_id))
        }
    }

    fn save(&self, user: &User) -> Result<(), CacheError> {
        let user_file_path = self.path_for_user_data(&user.session_id);

        // Create puzzle directory if it does not already exist.
        let mut user_file_dir = user_file_path.clone();
        user_file_dir.pop();

        std::fs::create_dir_all(user_file_dir)?;

        // Write the serialized user data to disk.
        let json_text = serde_json::to_string(&user)?;
        tracing::debug!("saving user data `{user_file_path:?}`");

        std::fs::write(user_file_path, json_text)?;
        Ok(())
    }
}
