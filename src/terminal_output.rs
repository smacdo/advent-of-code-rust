use std::time::Duration;

use advent_of_code_data::{data::CheckResult, Answer, Part};
use tracing::{event, Level};
use yuletide::{
    runner::{RunDetails, RunnerError, RunnerEventHandler},
    {Solver, SolverError},
};

pub struct ConsoleRunnerEventHandler {}

impl ConsoleRunnerEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ConsoleRunnerEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RunnerEventHandler for ConsoleRunnerEventHandler {
    fn on_start_solver(&mut self, solver: &Solver) {
        println!("Solving day {} year {}...", solver.day, solver.year);
    }

    fn on_start_part(&mut self, _solver: &Solver, part: Part) {
        println!("  Running part {part}...",);
    }

    fn on_example_fail(
        &mut self,
        solver: &Solver,
        part: Part,
        duration: Duration,
        example_index: usize,
        result: Result<Answer, RunnerError>,
    ) {
        let expected = &solver.part(part).example(example_index).expected;
        let input = solver.part(part).example(example_index).input;
        let actual = result.map_or_else(|e| e.to_string(), |v| v.to_string());

        event!(
            Level::WARN,
            %solver.year,
            %solver.day,
            %part,
            example_index,
            duration = duration.as_secs_f32(),
            actual,
            %expected,
            input,
            "the answer `{actual}` for example {example_index} did not match the expected value `{expected}`",
        );
    }

    fn on_finish_part_examples(
        &mut self,
        _solver: &Solver,
        _part: Part,
        duration: Duration,
        pass_count: usize,
        fail_count: usize,
    ) {
        if fail_count > 0 {
            println!(
                "    Checked {} examples and {fail_count} failed ❌ [{:.3}s]",
                pass_count + fail_count,
                duration.as_secs_f32()
            )
        } else if pass_count > 0 {
            println!(
                "    Checked {pass_count} examples ✅ [{:.3}s]",
                duration.as_secs_f32()
            )
        }
    }

    fn on_finish_part(
        &mut self,
        solver: &Solver,
        part: Part,
        duration: Duration,
        result: &Result<(Answer, CheckResult), RunnerError>,
    ) {
        // Catch the examples failed condition early, and print it before trying
        // to calculate runtime of the solution which isn't possible because the
        // solution never ran.
        match result {
            Ok((answer, CheckResult::Correct)) => {
                println!(
                    "    Answer is correct 👍 [{:.3}s]: {answer}",
                    duration.as_secs_f32()
                );
            }
            Ok((answer, CheckResult::Wrong)) => {
                println!(
                    "    Answer is wrong 👎 [{:.3}s]: {answer}",
                    duration.as_secs_f32()
                )
            }
            Ok((answer, CheckResult::TooLow)) => {
                println!(
                    "    Answer is too low 📉 [{:.3}s]: {answer}",
                    duration.as_secs_f32()
                )
            }
            Ok((answer, CheckResult::TooHigh)) => {
                println!(
                    "    Answer is too high 📈 [{:.3}s]: {answer}",
                    duration.as_secs_f32()
                )
            }
            Err(RunnerError::Solver(SolverError::NotFinished)) => {
                println!(
                    "    Solution is not finished 👻 [{:.3}s]",
                    duration.as_secs_f32()
                );
            }
            Err(RunnerError::Solver(SolverError::TooSoon)) => {
                event!(
                    Level::ERROR,
                    %solver.year,
                    %solver.day,
                    %part,
                    duration = duration.as_secs_f32(),
                    "solution submission blocked - please wait a bit before trying again",
                );
            }
            Err(error) => {
                event!(
                    Level::ERROR,
                    %solver.year,
                    %solver.day,
                    %part,
                    duration = duration.as_secs_f32(),
                    ?error,
                    "solver returned an error rather than an answer",
                );
            }
        }
    }

    fn on_finish_solver(&mut self, _solver: &Solver, duration: Duration, _details: RunDetails) {
        println!("Solved in {:.3} seconds", duration.as_secs_f32())
    }
}
