use std::{collections::HashMap, time::Instant};

use advent_of_code_data::{data::CheckResult, registry::{Solver, SolverError}, runner::{RunnerError, RunnerEventHandler}, Answer, Day, Part, Year};

struct SolverTimes {
    pub part_one_start: Option<Instant>,
    pub part_two_start: Option<Instant>,
}

impl SolverTimes {
    pub fn new() -> Self {
        Self {
            part_one_start: None,
            part_two_start: None,
        }
    }
}

impl Default for SolverTimes {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: measure elapsed times.
pub struct ConsoleRunnerEventHandler {
    times: HashMap<(Day, Year), SolverTimes>,
}

impl ConsoleRunnerEventHandler {
    pub fn new() -> Self {
        Self {
            times: HashMap::new()
        }
    }
}

impl Default for ConsoleRunnerEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RunnerEventHandler for ConsoleRunnerEventHandler {
    fn on_start_solver(&mut self, solver: &Solver) {
        self.times.insert((solver.day, solver.year), SolverTimes::new());
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
        match part {
            Part::One => {
                self.times.get_mut(&(solver.day, solver.year)).unwrap().part_one_start = Some(Instant::now());
            } ,
            Part::Two => {
                self.times.get_mut(&(solver.day, solver.year)).unwrap().part_two_start = Some(Instant::now());
            },
        }
    }

    fn on_start_part(&mut self, solver: &Solver, part: Part) {
        match part {
            Part::One => {
                self.times.get_mut(&(solver.day, solver.year)).unwrap().part_one_start = Some(Instant::now());
            } ,
            Part::Two => {
                self.times.get_mut(&(solver.day, solver.year)).unwrap().part_two_start = Some(Instant::now());
            },
        }
    }

    fn on_finish_part(
        &mut self,
        solver: &Solver,
        part: Part,
        result: &Result<(Answer, CheckResult), RunnerError>,
    ) {
        // Calculate the time elapsed since the examples completed and this event
        // indicating it finished.
        // elapsed_seconds = time.time() - self.solver_start_times[
        //    solver_metadata
        // ].get_part_start_time(part)
        let start_time = match part {
            Part::One => {
                self.times.get(&(solver.day, solver.year)).unwrap().part_one_start.unwrap()
            } ,
            Part::Two => {
                self.times.get(&(solver.day, solver.year)).unwrap().part_two_start.unwrap()
            },
        };
        let elapsed_time = Instant::now() - start_time;

        // Catch the examples failed condition early, and print it before trying
        // to calculate runtime of the solution which isn't possible because the
        // solution never ran.
        match result {
            Ok((answer, CheckResult::Correct)) => {
                println!("✅ part {part}: {answer} [{:.3}s]", elapsed_time.as_secs_f32());
            },
            Ok((answer, CheckResult::Wrong)) => {
                println!("❌ Wrong answer for part {part}: {answer} [{:.3}s]", elapsed_time.as_secs_f32())
            },
            Ok((answer, CheckResult::TooLow)) => {
                println!("❌ Wrong answer for part {part}: {answer} is too low [{:.3}s]", elapsed_time.as_secs_f32())
            },
            Ok((answer, CheckResult::TooHigh)) => {
                println!("❌ Wrong answer for part {part}: {answer} is too high [{:.3}s]", elapsed_time.as_secs_f32())
            },
            Err(RunnerError::Solver(SolverError::NotFinished)) => {
                println!("👻 Solver for part {} is not finished [{:.3}s]", part, elapsed_time.as_secs_f32());
            },
            Err(RunnerError::Solver(SolverError::ExampleFailed { input, expected, actual,..}) ) => {
                println!(
                    "👎 The example output for part {} is `{}` but the solver returned `{}` using input [{:.3}s]:\n```\n{}\n```",
                    part, 
                    expected,
                    actual,
                    elapsed_time.as_secs_f32(),
                    input
                );
            }
            Err(RunnerError::Solver(SolverError::TooSoon)) => {
                println!("⏱️ Solution for part {part} submitted too soon, please wait a bit before trying again");
            }
            Err(error) => {
                // TODO: Better error reporting.
                // TODO: Better icon
                println!("👎 The solver for part {part} returned an unhandled error [{:.3}s]: {error:?}", elapsed_time.as_secs_f32());
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