use client::WebClient;

pub mod client;
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
    Twp,
}

#[derive(Clone, Debug)]
pub enum Answer {
    NotFinished,
    String(String),
    Int(isize),
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

pub fn submit_answer(answer: Answer, part: Part, day: Day, year: Year) {
    let mut client: WebClient = Default::default();
    client.submit_answer(answer, part, day, year)
}
