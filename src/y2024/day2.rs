use advent_of_code_rust::{Answer, Day, Solver, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(2),
    year: Year(2024),
    title: None,
    part_one: day_2_1,
    part_two: day_2_2,
};

pub fn day_2_1(input: &str) -> Answer {
    println!("day 2-1");
    Answer::NotFinished
}

pub fn day_2_2(input: &str) -> Answer {
    println!("day 2-2");
    Answer::NotFinished
}
