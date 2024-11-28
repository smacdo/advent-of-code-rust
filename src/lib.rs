use client::WebClient;

pub mod client;
pub mod registry;
pub mod settings;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Day(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Year(pub usize);

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
    todo!();
}

pub fn solve(answer: Answer, part: Part, day: Day, year: Year) {
    let client: WebClient = Default::default();
    todo!();
}
