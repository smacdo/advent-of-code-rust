use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(10),
    year: Year(2024),
    part_one: SolverPart {
        func: day_10_1,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
    part_two: SolverPart {
        func: day_10_2,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
};

pub fn day_10_1(input: &str) -> Result<Answer> {
    Err(advent_of_code_data::registry::SolverError::NotFinished)
}

pub fn day_10_2(_input: &str) -> Result<Answer> {
    Err(advent_of_code_data::registry::SolverError::NotFinished)
}
