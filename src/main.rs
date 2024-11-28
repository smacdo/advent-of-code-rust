mod y2024;

use advent_of_code_rust::{
    get_input,
    registry::{Solver, SolverRegistry},
    Day, Year,
};
use linkme::distributed_slice;

#[distributed_slice]
pub static SOLVERS: [Solver];

fn main() {
    let var_name = SolverRegistry::new(&SOLVERS);
    let solvers = var_name;
    println!("{}", get_input(Day(1), Year(2023)));
    solvers.run_all();
}
