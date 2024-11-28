use advent_of_code_rust::registry::{Result, Solver, SolverError};
use advent_of_code_rust::{Answer, Day, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(2),
    year: Year(2024),
    part_one: day_2_1,
    part_two: day_2_2,
};

pub fn day_2_1(_input: &str) -> Result<Answer> {
    println!("day 2-1");
    Err(SolverError::NotFinished)
}

pub fn day_2_2(_input: &str) -> Result<Answer> {
    println!("day 2-2");
    Err(SolverError::NotFinished)
}
