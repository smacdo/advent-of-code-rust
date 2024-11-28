use client::WebClient;

pub mod client;
pub mod data;
pub mod registry;
pub mod settings;
pub mod utils;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Day(pub usize);

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Year(pub usize);

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Part {
    One,
    Two,
}

#[derive(Clone, Debug)]
pub enum Answer {
    String(String),
    Int(i64),
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i64> for Answer {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Self::Int(value as i64)
    }
}

impl From<isize> for Answer {
    fn from(value: isize) -> Self {
        Self::Int(value as i64)
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        assert!(value as u64 <= i64::MAX as u64);
        Self::Int(value as i64)
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match self {
            Answer::String(v) => v.to_string(),
            Answer::Int(v) => v.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub day: Day,
    pub year: Year,
    pub title: Option<String>,
}

pub fn get_input(day: Day, year: Year) -> String {
    let client: WebClient = Default::default();
    client.get_input(day, year)
}

pub fn submit_answer(answer: Answer, part: Part, day: Day, year: Year) -> String {
    let mut client: WebClient = Default::default();
    client.submit_answer(answer, part, day, year)
}
