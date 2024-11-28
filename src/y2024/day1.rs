use advent_of_code_rust::registry::Solver;
use advent_of_code_rust::{Answer, Day, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(1),
    year: Year(2024),
    part_one: day_1_1,
    part_two: day_1_2,
};

pub fn day_1_1(_input: &str) -> Answer {
    println!("day 1-1");
    Answer::NotFinished
}

pub fn day_1_2(_input: &str) -> Answer {
    println!("day 1-2");
    Answer::NotFinished
}
