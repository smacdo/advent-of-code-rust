use advent_of_code_data::{data::CheckResult, registry::{Solver, SolverError}, runner::{RunnerError, RunnerEventHandler}, Answer, Part};


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
        result: &Result<(Answer, CheckResult), RunnerError>,
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
            Err(RunnerError::Solver(SolverError::NotFinished)) => {
                println!("👻 Solver for part {} is not finished", part);
            },
            Err(RunnerError::Solver(SolverError::ExampleFailed { input, expected, actual,..}) ) => {
                println!(
                    "👎 The example output for part {} is `{}` but the solver returned `{}` using input:\n```\n{}\n```",
                    part, 
                    expected,
                    actual,
                    input
                );
            }
            Err(RunnerError::Solver(SolverError::TooSoon)) => {
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