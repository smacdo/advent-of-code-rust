use advent_of_code_data::registry::{Result, Solver, SolverError, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(2),
    year: Year(2024),
    part_one: SolverPart {
        func: day_2_1,
        examples: &[],
    },
    part_two: SolverPart {
        func: day_2_2,
        examples: &[],
    },
};

pub fn day_2_1(_input: &str) -> Result<Answer> {
    Err(SolverError::NotFinished)
}

pub fn day_2_2(_input: &str) -> Result<Answer> {
    Err(SolverError::NotFinished)
}
