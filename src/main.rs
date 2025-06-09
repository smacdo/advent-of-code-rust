mod y2024;

use advent_of_code_data::{
    client::{Client, WebClient},
    settings::ClientOptions,
    Day, Part, Year,
};
use clap::{Parser, Subcommand};
use linkme::distributed_slice;
use thiserror::Error;
use yuletide::terminal_output::ConsoleRunnerEventHandler;
use yuletide::{runner::SolverRunner, SolverAutoRegister, SolverRegistry};

#[distributed_slice]
pub static SOLVERS: [SolverAutoRegister];

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

// TODO: Merge all of the no puzzle errors into one, and then provided a better
// custom formatter to print descriptive errors.
#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("no puzzle solver found for year {} day {}", .0, .1)]
    SolverNotFound(Year, Day),
    #[error("no puzzle solvers were found for year {}", .0)]
    NoSolversForYear(Year),
    #[error("no puzzle solvers were found")]
    NoSolversFound,
    #[error("an error happened while communicating with the Advent of Code service: {}", .0)]
    ClientError(#[from] advent_of_code_data::client::ClientError),
}

fn main() -> Result<(), AppError> {
    // Argument parsing.
    let cli = Cli::parse();

    let log_level = match cli.debug {
        true => tracing::Level::DEBUG,
        _ => tracing::Level::INFO,
    };

    // Logging setup.
    let subscriber = tracing_subscriber::fmt().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let solver_registry = SolverRegistry::compiled_from(&SOLVERS);

    // Create the Advent of Code client.
    let client = WebClient::with_options(
        ClientOptions::new()
            .with_cache()
            .with_user_config()
            .with_local_dir_config()
            .with_env_vars(),
    );

    match &cli.command {
        Some(Commands::Run { days, year }) => {
            run_solver_command(&solver_registry, client, days, year)
        }
        Some(Commands::Check { days, year }) => {
            run_check_command(&solver_registry, client, days, year)
        }
        Some(Commands::Input { day, year }) => {
            let puzzle_input = client.get_input(Day(*day), Year(*year))?;
            println!("{puzzle_input}");

            Ok(())
        }
        _ => {
            panic!("command not implemented")
        }
    }
}

fn run_solver_command(
    solver_registry: &SolverRegistry,
    client: WebClient,
    days: &Option<Vec<usize>>,
    year: &Option<usize>,
) -> Result<(), AppError> {
    // Use the puzzle year given on the command line, or if not specified find the most
    // recent year in the solver registry.
    let year = year.map_or_else(
        || {
            solver_registry
                .years()
                .into_iter()
                .max()
                .ok_or(AppError::NoSolversFound)
        },
        |year| Ok(Year(year)),
    )?;

    // Use the puzzle day given on the command line, or if not specified find the most
    // recent day in the registry for the selected year.
    let requested_days = days.as_ref().map_or_else(
        || -> Result<_, AppError> {
            Ok(vec![solver_registry
                .days(year)
                .ok_or(AppError::NoSolversForYear(year))?
                .into_iter()
                .max()
                .expect(
                    "expected only years with at least one solver from SolverRegistry::years()",
                )])
        },
        |days| Ok(days.iter().map(|d| Day(*d)).collect()),
    )?;

    let mut runner =
        SolverRunner::new(Box::new(client), Box::new(ConsoleRunnerEventHandler::new()));

    let available_days = solver_registry
        .days(year)
        .ok_or(AppError::NoSolversForYear(year))?;

    // Error out if any of the requested days do not have a solver.
    if let Some(missing_day) = requested_days
        .iter()
        .find(|day| !available_days.contains(day))
    {
        return Err(AppError::SolverNotFound(year, *missing_day));
    }

    // Run a solver for each requested day.
    for requested_day in requested_days {
        if available_days.contains(&requested_day) {
            runner.push(
                solver_registry
                    .solver(year, requested_day)
                    .ok_or(AppError::SolverNotFound(year, requested_day))?
                    .clone(),
            );
        }
    }

    runner.run_all(); // TODO: This should return Result and be changed to ?.
    Ok(())
}

fn run_check_command(
    solver_registry: &SolverRegistry,
    client: WebClient,
    days: &Option<Vec<usize>>,
    year: &Option<usize>,
) -> Result<(), AppError> {
    // Iterate all the year(r) and day(s) from the arguments, and save a list of puzzles that have
    // at least one part with a correct answer in the puzzle cache.
    //
    // If no years are given, assume the caller wants to run a check on all years with at least one
    // solver.
    let mut puzzles: Vec<(Year, Day)> = Vec::new();

    for year in year.map(|y| vec![Year(y)]).unwrap_or_else(|| {
        // When the caller does not provide a year, default to every year that
        // has at least one solver.
        solver_registry.years()
    }) {
        for day in days
            .as_ref()
            .map(|days| Ok(days.iter().map(|d| Day(*d)).collect()))
            .unwrap_or_else(|| {
                // When the daller does not provide a list of days, default to
                // every day in the year that has a solver.
                solver_registry
                    .days(year)
                    .ok_or(AppError::NoSolversForYear(year))
            })?
        {
            for part in [Part::One, Part::Two] {
                if let Ok(answers) = client.puzzle_cache.load_answers(part, day, year) {
                    if answers.correct_answer_ref().is_some() {
                        puzzles.push((year, day));
                    }
                }
            }
        }
    }

    // Start puzzles in ascending calendar order.
    puzzles.sort();

    // Run selected puzzle days.
    // TODO: Specify runner should not submit any answers.
    let mut runner =
        SolverRunner::new(Box::new(client), Box::new(ConsoleRunnerEventHandler::new()));

    for (year, day) in puzzles {
        runner.push(
            solver_registry
                .solver(year, day)
                .expect("puzzles array is exepcted to contain only valid year/day values")
                .clone(),
        );
    }

    runner.run_all(); // TODO: This should return Result and be changed to ?.
    Ok(())
}
