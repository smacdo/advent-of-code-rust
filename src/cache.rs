use std::path::{Path, PathBuf};

use crate::{
    data::{Answers, Puzzle},
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
        PuzzleCache {
            cache_dir: cache_dir.into(),
            _encryption_token: encryption_token.map(|x| x.into()),
        }
    }

    pub fn try_load_input(&self, day: Day, year: Year) -> Option<String> {
        let input_path = Self::input_file_path(&self.cache_dir, day, year);

        if input_path.exists() {
            Some(
                std::fs::read_to_string(Self::input_file_path(&self.cache_dir, day, year)).unwrap(),
            )
        } else {
            None
        }
    }

    pub fn load(&self, day: Day, year: Year) -> Puzzle {
        // TODO: Convert unwraps into Errors.
        Puzzle {
            day,
            year,
            input: std::fs::read_to_string(Self::input_file_path(&self.cache_dir, day, year))
                .unwrap(),
            part_one_answers: Answers::deserialize_from_str(
                &std::fs::read_to_string(Self::answers_file_path(
                    &self.cache_dir,
                    Part::One,
                    day,
                    year,
                ))
                .unwrap(),
            ),
            part_two_answers: Answers::deserialize_from_str(
                &std::fs::read_to_string(Self::answers_file_path(
                    &self.cache_dir,
                    Part::Two,
                    day,
                    year,
                ))
                .unwrap(),
            ),
        }
    }

    pub fn save(&self, puzzle: Puzzle) {
        // TODO: Convert unwraps into Errors.
        // Create the puzzle directory in the cache if it doesn't already exist.
        let puzzle_dir = Self::dir_for_puzzle(&self.cache_dir, puzzle.day, puzzle.year);
        std::fs::create_dir_all(puzzle_dir).unwrap();

        std::fs::write(
            Self::input_file_path(&self.cache_dir, puzzle.day, puzzle.year),
            puzzle.input,
        )
        .unwrap();

        std::fs::write(
            Self::answers_file_path(&self.cache_dir, Part::One, puzzle.day, puzzle.year),
            puzzle.part_one_answers.serialize_to_string(),
        )
        .unwrap();

        std::fs::write(
            Self::answers_file_path(&self.cache_dir, Part::Two, puzzle.day, puzzle.year),
            puzzle.part_two_answers.serialize_to_string(),
        )
        .unwrap();
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
