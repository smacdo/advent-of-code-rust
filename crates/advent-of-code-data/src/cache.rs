use std::path::{Path, PathBuf};

use crate::{
    data::{Answers, Puzzle, User},
    Day, Part, Year,
};

// TODO: Support encryption and decryption.
#[derive(Debug)]
pub struct PuzzleCache {
    cache_dir: PathBuf,
    _encryption_token: Option<String>,
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
            _encryption_token: encryption_token.map(|x| x.into()),
        }
    }

    pub fn load_input(&self, day: Day, year: Year) -> std::io::Result<String> {
        std::fs::read_to_string(Self::input_file_path(&self.cache_dir, day, year))
    }

    pub fn load_answers(&self, part: Part, day: Day, year: Year) -> std::io::Result<Answers> {
        // TODO: Convert unwraps into Errors.
        Ok(Answers::deserialize_from_str(&std::fs::read_to_string(
            Self::answers_file_path(&self.cache_dir, part, day, year),
        )?))
    }

    pub fn load_puzzle(&self, day: Day, year: Year) -> Puzzle {
        // TODO: Convert unwraps into Errors.
        // TODO: Create default input or answers if the files don't exist.
        Puzzle {
            day,
            year,
            input: self.load_input(day, year).unwrap(),
            part_one_answers: self.load_answers(Part::One, day, year).unwrap(),
            part_two_answers: self.load_answers(Part::Two, day, year).unwrap(),
        }
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
        let input_path = Self::input_file_path(&self.cache_dir, day, year);
        let mut puzzle_dir = input_path.clone();
        puzzle_dir.pop();

        std::fs::create_dir_all(puzzle_dir).unwrap();

        tracing::debug!("saving input for day {day} year {year} to {input_path:?}");
        std::fs::write(input_path, input)
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
