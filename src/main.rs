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
    /// Runs one or more solvers and checks if the result is correct/incorrect.
    Run {
        /// Puzzle day (defaults to the most recent day with a solver).
        #[arg(short, long)]
        days: Option<Vec<usize>>,

        /// Puzzle year (defaults to the most recent year with a solver).
        #[arg(short, long)]
        year: Option<usize>,
    },
    /// Runs all puzzle solvers that have a solution, and reports which solvers
    /// are broken because they don't match the known answer.
    Check {
        /// Puzzle day (defaults to all if not specified).
        #[arg(short, long)]
        days: Option<Vec<usize>>,

        /// Puzzle year (defaults to all if not specified).
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

    // Logging setup.
    let subscriber = tracing_subscriber::fmt().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let solver_registry = SolverRegistry::new(&SOLVERS);

    // Create the Advent of Code client.
    let client: WebClient = Default::default();

    match &cli.command {
        Some(Commands::Run { days, year }) => {
            // Use the puzzle year given on the command line, or if not specified find the most
            // recent year in the solver registry.
            let year = year.map_or_else(
                || {
                    solver_registry
                        .years()
                        .into_iter()
                        .max()
                        .expect("TODO: handle when there are no solvers")
                },
                Year,
            );

            // Use the puzzle day given on the command line, or if not specified find the most
            // recent day in the registry for the selected year.
            let requested_days = days.as_ref().map_or_else(
                || {
                    vec![solver_registry
                        .days(year)
                        .into_iter()
                        .max()
                        .expect("TODO: handle when there are no solvers for the year")]
                },
                |days| days.iter().map(|d| Day(*d)).collect(),
            );

            let mut runner =
                SolverRunner::new(Box::new(client), Box::new(ConsoleRunnerEventHandler::new()));
            let available_days = solver_registry.days(year);

            for requested_day in requested_days {
                if available_days.contains(&requested_day) {
                    runner.push(solver_registry.solver(requested_day, year).clone());
                }
            }

            runner.run_all();
        }
        Some(Commands::Check { days, year }) => {
            let mut runner =
                SolverRunner::new(Box::new(client), Box::new(ConsoleRunnerEventHandler::new()));
            // TODO: Tell runner to not check answers against WWW, or to write new answers.

            for year in year
                .map(|y| vec![Year(y)])
                .unwrap_or_else(|| solver_registry.years())
            {
                for day in days
                    .as_ref()
                    .map(|days| days.iter().map(|d| Day(*d)).collect())
                    .unwrap_or_else(|| solver_registry.days(year))
                {
                    // TODO: Check if answer cache is available.
                    // TODO: Check if part 1 or part part 2 has a correct answer.
                    runner.push(solver_registry.solver(day, year).clone());
                }
            }

            runner.run_all();
        }
        Some(Commands::Input { day, year }) => {
            println!("{}", client.get_input(Day(*day), Year(*year)).unwrap());
        }
        _ => {
            panic!("command not implemented")
        }
    }
}
