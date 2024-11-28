mod y2024;

use advent_of_code_rust::{
    registry::{Solver, SolverRegistry},
    Answer,
};
use linkme::distributed_slice;

#[distributed_slice]
pub static SOLVERS: [Solver];

fn main() {
    let var_name = SolverRegistry::new(&SOLVERS);
    let solvers = var_name;
    solvers.run_all();
}
