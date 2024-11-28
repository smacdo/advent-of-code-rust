mod y2024;

use advent_of_code_rust::{
    registry::{Solver, SolverRegistry},
    Answer, Day, Part, Year,
};
use linkme::distributed_slice;

#[distributed_slice]
pub static SOLVERS: [Solver];

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let var_name = SolverRegistry::new(&SOLVERS);
    let solvers = var_name;
    //println!("{}", get_input(Day(1), Year(2023)));
    //println!(
    //    "SUBMIT: {}",
    //    submit_answer(Answer::Int(42), Part::One, Day(12), Year(2023))
    //);
    solvers.run_all();
}
