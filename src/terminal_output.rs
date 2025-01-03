use std::time::Duration;

use advent_of_code_data::{data::CheckResult, registry::{Solver, SolverError}, runner::{RunDetails, RunPartDetails, RunnerError, RunnerEventHandler}, Part};

// TODO: measure elapsed times.
pub struct ConsoleRunnerEventHandler {
}

impl ConsoleRunnerEventHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Default for ConsoleRunnerEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RunnerEventHandler for ConsoleRunnerEventHandler {
    fn on_start_solver(&mut self, _solver: &Solver) {
    }

    fn on_part_examples_pass(&mut self, solver: &Solver, part: Part, duration: Duration, count: usize) {
        if count > 0 {
            println!(
                "👍 Tested the examples for part {} day {} year {} [{:.3}s]",
                part, solver.day, solver.year, duration.as_secs_f32()
            )
        }
    }

    fn on_start_part(&mut self, _solver: &Solver, _part: Part) {
    }

    fn on_finish_part(
        &mut self,
        _solver: &Solver,
        part: Part,
        duration: Duration,
        result: &Result<RunPartDetails, RunnerError>,
    ) {
        // Catch the examples failed condition early, and print it before trying
        // to calculate runtime of the solution which isn't possible because the
        // solution never ran.
        match result {
            Ok(RunPartDetails{answer, check_result: CheckResult::Correct, ..}) => {
                println!("✅ part {part}: {answer} [{:.3}s]", duration.as_secs_f32());
            },
            Ok(RunPartDetails{answer, check_result: CheckResult::Wrong, ..}) => {
                println!("❌ Wrong answer for part {part}: {answer} [{:.3}s]", duration.as_secs_f32())
            },
            Ok(RunPartDetails{answer, check_result: CheckResult::TooLow, ..}) => {
                println!("❌ Wrong answer for part {part}: {answer} is too low [{:.3}s]", duration.as_secs_f32())
            },
            Ok(RunPartDetails{answer, check_result: CheckResult::TooHigh, ..}) => {
                println!("❌ Wrong answer for part {part}: {answer} is too high [{:.3}s]", duration.as_secs_f32())
            },
            Err(RunnerError::Solver(SolverError::NotFinished)) => {
                println!("👻 Solver for part {} is not finished  [{:.3}s]", part, duration.as_secs_f32());
            },
            Err(RunnerError::Solver(SolverError::ExampleFailed { input, expected, actual,..}) ) => {
                println!(
                    "Example output for part {} is `{}` but the solver returned `{}` using input:\n```\n{}\n```",
                    part, 
                    expected,
                    actual,
                    input
                );
                println!(
                    "👎 The solver for part {} returned `{}` but the example output is `{}`",
                    part, 
                    actual,
                    expected,
                );
            }
            Err(RunnerError::Solver(SolverError::TooSoon)) => {
                println!("⏱️ Solution for part {part} submitted too soon, please wait a bit before trying again  [{:.3}s]", duration.as_secs_f32());
            }
            Err(error) => {
                // TODO: Better error reporting.
                // TODO: Better icon
                println!("👎 The solver for part {part} returned an unhandled error [{:.3}s]: {error:?}", duration.as_secs_f32());
            }
        }
    }

    fn on_finish_solver(
        &mut self,
        _solver: &Solver,
        _duration: Duration,
        _details: RunDetails,
    ) {
    }
}