use std::path::{Path, PathBuf};

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
    #[error("base 64 decoding failed: {}", .0)]
    DecodeBase64(#[from] base64::DecodeError),
    #[error("decryption failed: {}", .0)]
    Decryption(#[from] anyhow::Error),
    #[error("decoding utf8 failed: {}", .0)]
    DecodeUtf8(#[from] std::string::FromUtf8Error),
    #[error("a file i/o error occured while reading/writing the cache: {}", .0)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct PuzzleCache {
    cache_dir: PathBuf,
    encryption_token: Option<String>,
}

impl PuzzleCache {
    const INPUT_FILE_NAME: &'static str = "input.txt";
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

    pub fn try_load_input(&self, day: Day, year: Year) -> Result<Option<String>, CacheError> {
        match self.load_input(day, year) {
            Ok(input) => Ok(Some(input)),
            Err(CacheError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn load_input(&self, day: Day, year: Year) -> Result<String, CacheError> {
        // Load the cached input file from disk.
        let input_path = Self::input_file_path(&self.cache_dir, day, year);
        tracing::debug!("loading input for day {day} year {year} from {input_path:?}");

        let mut input_text = std::fs::read_to_string(input_path)?;

        // Decrypt the input data.
        // TODO: Add error check to see if input file is encrypted and the password
        //       token is `None` leading to encrypted input passed to puzzle.
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

    pub fn load_answers(&self, part: Part, day: Day, year: Year) -> Result<Answers, CacheError> {
        Ok(Answers::deserialize_from_str(&std::fs::read_to_string(
            Self::answers_file_path(&self.cache_dir, part, day, year),
        )?))
    }

    pub fn load_puzzle(&self, day: Day, year: Year) -> Result<Puzzle, CacheError> {
        // TODO: Create default input or answers if the files don't exist.
        Ok(Puzzle {
            day,
            year,
            input: self.load_input(day, year)?,
            part_one_answers: self.load_answers(Part::One, day, year)?,
            part_two_answers: self.load_answers(Part::Two, day, year)?,
        })
    }

    pub fn save(&self, puzzle: Puzzle) {
        // TODO: Convert unwraps into Errors.
        // Create the puzzle directory in the cache if it doesn't already exist.
        let puzzle_dir = Self::dir_for_puzzle(&self.cache_dir, puzzle.day, puzzle.year);
        std::fs::create_dir_all(puzzle_dir).unwrap();

        self.save_input(&puzzle.input, puzzle.day, puzzle.year)
            .unwrap();

        self.save_answers(&puzzle.part_one_answers, Part::One, puzzle.day, puzzle.year)
            .unwrap();

        self.save_answers(&puzzle.part_two_answers, Part::Two, puzzle.day, puzzle.year)
            .unwrap();
    }

    pub fn save_input(&self, input: &str, day: Day, year: Year) -> std::io::Result<()> {
        // TODO: Convert unwraps into Errors.
        // Calculate the path to the puzzle's input file.
        let input_path = Self::input_file_path(&self.cache_dir, day, year);

        // Create puzzle directory if it does not already exist.
        let mut puzzle_dir = input_path.clone();
        puzzle_dir.pop();

        std::fs::create_dir_all(puzzle_dir).unwrap();

        // Write the input to disk, and optionally encrypt the input file when
        // stored on disk.
        if let Some(encryption_token) = &self.encryption_token {
            // Encrypt then base64 encode for better version control handling.
            let encrypted_data =
                encrypt(input.as_bytes(), encryption_token.as_bytes()).expect("failed to encrypt");
            let b64_encrypted_text = BASE64_STANDARD.encode(encrypted_data);

            tracing::debug!("saving encrypted input for day {day} year {year} to {input_path:?}");
            std::fs::write(input_path, b64_encrypted_text)
        } else {
            // No encryption.
            tracing::debug!("saving unencrypted input for day {day} year {year} to {input_path:?}");
            std::fs::write(input_path, input)
        }
    }

    pub fn save_answers(
        &self,
        answers: &Answers,
        part: Part,
        day: Day,
        year: Year,
    ) -> std::io::Result<()> {
        let answers_path = Self::answers_file_path(&self.cache_dir, part, day, year);
        let mut puzzle_dir = answers_path.clone();
        puzzle_dir.pop();

        std::fs::create_dir_all(puzzle_dir).unwrap();

        tracing::debug!("saving answer for part {part} day {day} year {year} to {answers_path:?}");
        std::fs::write(answers_path, answers.serialize_to_string())
    }

    pub fn dir_for_puzzle(cache_dir: &Path, day: Day, year: Year) -> PathBuf {
        cache_dir.join(format!("y{}", year)).join(day.to_string())
    }

    pub fn input_file_path(cache_dir: &Path, day: Day, year: Year) -> PathBuf {
        Self::dir_for_puzzle(cache_dir, day, year).join(Self::INPUT_FILE_NAME)
    }

    pub fn answers_file_path(cache_dir: &Path, part: Part, day: Day, year: Year) -> PathBuf {
        Self::dir_for_puzzle(cache_dir, day, year).join(match part {
            Part::One => Self::PART_ONE_ANSWERS_FILE_NAME,
            Part::Two => Self::PART_TWO_ANSWERS_FILE_NAME,
        })
    }
}

#[derive(Debug)]
pub struct UserDataCache {
    cache_dir: PathBuf,
}

impl UserDataCache {
    pub fn new<P: Into<PathBuf>>(cache_dir: P) -> Self {
        // TODO: Validate cache_dir is a directory, and is writable.
        // TODO: Create dir if not exists.

        Self {
            cache_dir: cache_dir.into(),
        }
    }

    pub fn load(&self, session_id: &str) -> User {
        // TODO: Validate session_id is safe for filename.
        // TODO: Figure out a better cache dir layout.
        // TODO: Replace unwraps with Errors.
        let user_file_path = self.cache_dir.join(format!("{session_id}.json"));

        if user_file_path.is_file() {
            tracing::debug!("found cached user data for {session_id} at `{user_file_path:?}`");

            let json_text = std::fs::read_to_string(user_file_path).unwrap();
            let user: User = serde_json::from_str(&json_text).unwrap();

            user
        } else {
            tracing::debug!("no cached user data for {session_id} at `{user_file_path:?}`, returning new User object");
            User::new(session_id)
        }
    }

    pub fn save(&self, user: &User) {
        // TODO: Validate session_id is safe for filename.
        // TODO: Figure out a better cache dir layout.
        // TODO: Replace unwraps with Errors.

        let user_file_path = self.cache_dir.join(format!("{}.json", user.session_id));
        let json_text = serde_json::to_string(&user).unwrap();

        tracing::debug!(
            "saving user data for {} at `{user_file_path:?}`",
            user.session_id
        );
        std::fs::write(user_file_path, json_text).unwrap();
    }
}
