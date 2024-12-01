mod y2024;

use advent_of_code_data::{
    client::{Client, WebClient},
    registry::{Solver, SolverRegistry},
    runner::SolverRunner,
    Day, Year,
};
use advent_of_code_rust::terminal_output::ConsoleRunnerEventHandler;
use clap::{Parser, Subcommand};
use linkme::distributed_slice;

#[distributed_slice]
pub static SOLVERS: [Solver];

// TODO: validation day, year

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging log output on.
    #[arg(long, action)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs puzzle solvers.
    Run {
        /// Only run solvers on puzzles that do not have a correct answer yet.
        #[arg(short, long, action)]
        unsolved: bool,

        /// Restrict solvers to the given days only.
        #[arg(short, long)]
        day: Option<Vec<usize>>,

        /// Restrict solvers to the given year only.
        #[arg(short, long)]
        year: Option<usize>,
    },
    /// Runs a solver with visualization mode enabled.
    Visualize {
        /// Day of puzzle.
        #[arg(short, long)]
        day: usize,

        /// Year of puzzle.
        #[arg(short, long)]
        year: usize,
    },
    /// Prints the input for a puzzle.
    Input {
        /// Day of puzzle.
        #[arg(short, long)]
        day: usize,

        /// Year of puzzle.
        #[arg(short, long)]
        year: usize,
    },
}

fn main() {
    // Argument parsing.
    let cli = Cli::parse();

    let log_level = match cli.debug {
        true => tracing::Level::DEBUG,
        _ => tracing::Level::INFO,
    };

    println!("LOG LEVEL: {:?}", log_level);

    // Logging setup.
    let subscriber = tracing_subscriber::fmt().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let solver_registry = SolverRegistry::new(&SOLVERS);

    // Create the Advent of Code client.
    let client: WebClient = Default::default();

    // Run solvers?
    match &cli.command {
        Some(Commands::Run {
            unsolved: _unsolved,
            day: days,
            year,
        }) => {
            // TODO: Find the latest year for a default value if year unspecified.
            // TODO: Use latest year with _unsolved_ puzzles or all puzzles.
            let year = Year(year.expect("TODO: implement default fallback"));

            // TODO: All the days if not specified.
            let requested_days = days.as_ref().expect("TODO: implement default fallback");
            let available_days = solver_registry.days(year);
            let mut runner =
                SolverRunner::new(Box::new(client), Box::new(ConsoleRunnerEventHandler {}));

            for requested_day in requested_days {
                let requested_day = Day(*requested_day);

                if available_days.contains(&requested_day) {
                    runner.push(solver_registry.solver(requested_day, year).clone());
                }
            }

            runner.run_all();
        }
        Some(Commands::Input { day, year }) => {
            println!("{}", client.get_input(Day(*day), Year(*year)));
        }
        _ => {
            panic!("command not implemented yet")
        }
    }
}
