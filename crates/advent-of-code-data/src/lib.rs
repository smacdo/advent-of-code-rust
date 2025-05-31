use std::str::FromStr;

use client::{Client, ClientError, WebClient};
use data::CheckResult;
use thiserror::Error;

pub mod cache;
pub mod client;
pub mod data;
pub mod settings;
mod utils;

// TODO: Remove string parse errors (no more errors when converting). This check
//       should happen in the cache storage, maybe w/ a TODO to support it.

/// Represents a day in an Advent of Code year. Days are typically in the range
/// [1, 25].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Day(pub usize);

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for Day {
    fn from(value: i32) -> Self {
        assert!(value >= 0);
        Day(value as usize)
    }
}

impl From<u32> for Day {
    fn from(value: u32) -> Self {
        Day(value as usize)
    }
}

/// Represents an Advent of Code year, which is a year in which there was at
/// least one Advent of Code puzzle.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Year(pub usize);

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for Year {
    fn from(value: i32) -> Self {
        assert!(value >= 0);
        Year(value as usize)
    }
}

impl From<Year> for i32 {
    fn from(value: Year) -> Self {
        value.0 as i32
    }
}

/// Advent of Code puzzles are split into two parts - `One` and `Two`. Both
/// parts will take the same input but typically produce different answers.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Part {
    One,
    Two,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Part::One => "One",
                Part::Two => "Two",
            }
        )
    }
}

/// Represents possible errors when parsing puzzle answers.
#[derive(Clone, Debug, PartialEq, Error)]
pub enum AnswerParseError {
    #[error("answers with empty strings or only whitespace chars are not allowed")]
    EmptyNotAllowed,
    #[error("answers with newline characters are not allowed")]
    NewlinesNotAllowed,
}

/// Holds a string or integer answer to an Advent of Code puzzle.
///
/// It is important to note that `Answer` does not denote a _correct_ answer to
/// a puzzle. To determine if a given `Answer` is correct you must submit it and
/// inspect the result.
///
/// ```
/// use advent_of_code_data::Answer;
///
/// // Answers can be created via their enum constructors.
/// let string_answer = Answer::String("hello world".to_string());
/// let int_answer = Answer::Int(42);
///
/// // Answers also support explicit conversion.
/// let string_answer: Answer = "hello world".into().unwrap();
/// let int_answer: Answer = 42.into();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    String(String),
    Int(i128),
}

impl Answer {
    pub fn to_i128(&self) -> Option<i128> {
        match self {
            Answer::String(_) => None,
            Answer::Int(v) => Some(*v),
        }
    }
}

impl FromStr for Answer {
    type Err = AnswerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.chars().all(|c| c.is_whitespace()) {
            return Err(AnswerParseError::EmptyNotAllowed);
        }

        if s.chars().any(|c| c == '\n') {
            return Err(AnswerParseError::NewlinesNotAllowed);
        }

        Ok(s.parse::<i128>()
            .map_or_else(|_| Answer::String(s.to_string()), Answer::Int))
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::String(v) => write!(f, "{}", v),
            Answer::Int(v) => write!(f, "{}", v),
        }
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Self::Int(value as i128)
    }
}

impl From<i64> for Answer {
    fn from(value: i64) -> Self {
        Self::Int(value as i128)
    }
}

impl From<isize> for Answer {
    fn from(value: isize) -> Self {
        Self::Int(value as i128)
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Self::Int(value as i128)
    }
}

/// TODO: Good documentation.
pub fn get_input(day: Day, year: Year) -> Result<String, ClientError> {
    let client: WebClient = Default::default();
    client.get_input(day, year)
}

/// TODO: Good documentation.
pub fn submit_answer(
    answer: Answer,
    part: Part,
    day: Day,
    year: Year,
) -> Result<CheckResult, ClientError> {
    let mut client: WebClient = Default::default();
    client.submit_answer(answer, part, day, year)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_year() {
        let year = Year(2022);
        assert_eq!(&format!("{year}"), "2022");
    }

    #[test]
    fn print_day() {
        let day = Day(22);
        assert_eq!(&format!("{day}"), "22");
    }

    #[test]
    fn print_part() {
        assert_eq!(&format!("{}", Part::One), "One");
        assert_eq!(&format!("{}", Part::Two), "Two");
    }

    #[test]
    fn print_answer() {
        assert_eq!(
            &format!("{}", Answer::String("hello world".to_string())),
            "hello world"
        );

        assert_eq!(&format!("{}", Answer::Int(42)), "42");
    }
}
