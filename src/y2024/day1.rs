use advent_of_code_data::registry::{Result, Solver, SolverError};
use advent_of_code_data::{Answer, Day, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(1),
    year: Year(2024),
    part_one: day_1_1,
    part_two: day_1_2,
};

pub fn day_1_1(_input: &str) -> Result<Answer> {
    println!("day 1-1");
    Err(SolverError::NotFinished)
}

pub fn day_1_2(_input: &str) -> Result<Answer> {
    println!("day 1-2");
    Err(SolverError::NotFinished)
}
