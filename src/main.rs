mod y2024;

use advent_of_code_rust::{Answer, Solver, SolverRegistry};
use linkme::distributed_slice;

#[distributed_slice]
pub static SOLVERS: [Solver];

fn main() {
    let solvers = SolverRegistry::new(&SOLVERS);
    solvers.run_all();
}
