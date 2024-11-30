mod y2024;

use advent_of_code_data::{
    client::WebClient, data::CheckResult, registry::{Solver, SolverError, SolverRegistry}, runner::{RunnerEventHandler, SolverRunner}, Answer, Day, Part, Year
};
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
            let mut runner = SolverRunner::new(
                Box::new(client),
                Box::new(ConsoleRunnerEventHandler{}));

            for requested_day in requested_days {
                let requested_day = Day(*requested_day);

                if available_days.contains(&requested_day) {
                    runner.push(solver_registry.solver(requested_day, year).clone());
                }
            }

            runner.run_all();
        }
        _ => {
            panic!("command not implemented yet")
        }
    }
}

// TODO: measure elapsed times.
pub struct ConsoleRunnerEventHandler {}

impl RunnerEventHandler for ConsoleRunnerEventHandler {
    fn on_start_solver(&mut self, _solver: &Solver) {
        // TODO: start measuring execution time for solver.
    }

    fn on_part_examples_pass(&mut self, solver: &Solver, part: Part, count: usize) {
        if count > 0 {
            println!(
                "👍 Tested the examples for part {} day {} year {}",
                part, solver.day, solver.year
            )
        }

        // Running the solver with real input happens immediately after this
        // event, so start the solver timer now.
        // TODO: implement time recording.
    }

    fn on_start_part(&mut self, _solver: &Solver, _part: Part) {}

    fn on_finish_part(
        &mut self,
        _solver: &Solver,
        part: Part,
        result: &Result<(Answer, CheckResult), SolverError>,
    ) {
        // Calculate the time elapsed since the examples completed and this event
        // indicating it finished.
        // elapsed_seconds = time.time() - self.solver_start_times[
        //    solver_metadata
        // ].get_part_start_time(part)


        // Catch the examples failed condition early, and print it before trying
        // to calculate runtime of the solution which isn't possible because the
        // solution never ran.
        match result {
            Ok((answer, CheckResult::Correct)) => {
                println!("✅ part {part}: {answer} [0.0s]");
            },
            Ok((answer, CheckResult::Wrong)) => {
                println!("❌ Wrong answer for part {part}: {answer} [0.0s]")
            },
            Ok((answer, CheckResult::TooLow)) => {
                println!("❌ Wrong answer for part {part}: {answer} is too low [0.0s]")
            },
            Ok((answer, CheckResult::TooHigh)) => {
                println!("❌ Wrong answer for part {part}: {answer} is too high [0.0s]")
            },
            Err(SolverError::NotFinished) => {
                println!("👻 Solver for part {} is not finished", part);
            },
            Err(SolverError::ExampleFailed { input, expected, actual,..})  => {
                println!(
                    "👎 The example output for part {} is `{}` but the solver returned `{}` using input:\n```\n{}\n```",
                    part, 
                    expected,
                    actual,
                    input
                );
            }
            Err(SolverError::TooSoon) => {
                println!("⏱️ Solution for part {part} submitted too soon, please wait a bit before trying again");
            }
            Err(error) => {
                // TODO: Better error reporting.
                // TODO: Better icon
                println!("👎 The solver for part {part} returned an unhandled error: {error:?}");
            }
        }
    }

    fn on_finish_solver(
        &mut self,
        _solver: &Solver,
    ) {
        // TODO: report total execution time for solver.
    }
}
