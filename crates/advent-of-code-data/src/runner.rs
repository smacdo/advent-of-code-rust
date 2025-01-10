use std::time::{Duration, Instant};

use thiserror::Error;

use crate::{
    client::{Client, ClientError},
    data::CheckResult,
    registry::{Solver, SolverError},
    Answer, Part,
};

// TODO:
//  - on_start_solver(solver)
//    - on_start_part(solver, part)
//      - on_start_examples(solver, part)
//        - on_start_example(solver, part, example_index)
//        _ on_finish_example(solver, part, example_index, duration, result)
//      - on_finish_examples(solver, part, did_all_pass)
//      - on_start_solver_part(solver, part)
//      - on_finish_solver_part(solver, part, duration, result)
//   - on_finish_part(solver, part, duration, result)
// - on_finish_solver(solver, duration, result)
pub trait RunnerEventHandler {
    fn on_start_solver(&mut self, _solver: &Solver) {}
    fn on_start_part(&mut self, _solver: &Solver, _part: Part) {}
    fn on_start_part_example(&mut self, _solver: &Solver, _part: Part, _example_index: usize) {}
    fn on_example_pass(
        &mut self,
        _solver: &Solver,
        _part: Part,
        _duration: Duration,
        _example_index: usize,
    ) {
    }
    fn on_example_fail(
        &mut self,
        _solver: &Solver,
        _part: Part,
        _duration: Duration,
        _example_index: usize,
        _result: Result<Answer, RunnerError>,
    ) {
    }
    fn on_finish_part_examples(
        &mut self,
        solver: &Solver,
        part: Part,
        duration: Duration,
        pass_count: usize,
        fail_count: usize,
    );
    fn on_finish_part(
        &mut self,
        solver: &Solver,
        part: Part,
        duration: Duration,
        result: &Result<(Answer, CheckResult), RunnerError>,
    );

    fn on_finish_solver(&mut self, solver: &Solver, duration: Duration, details: RunDetails);
}

pub struct SolverRunner {
    solvers_to_run: Vec<Solver>,
    pub client: Box<dyn Client>,
    pub event_handler: Box<dyn RunnerEventHandler>,
}

impl SolverRunner {
    pub fn new(client: Box<dyn Client>, event_handler: Box<dyn RunnerEventHandler>) -> Self {
        Self {
            solvers_to_run: Vec::new(),
            client,
            event_handler,
        }
    }

    /// Add solver to the list of solvers to be run.
    /// Solvers are run in the order that they are pushed to the runner.
    pub fn push(&mut self, solver: Solver) {
        tracing::debug!(
            "add solver year {} day {} to runner",
            solver.day,
            solver.year
        );

        self.solvers_to_run.push(solver);
    }

    pub fn run_all(&mut self) {
        for solver in &self.solvers_to_run {
            Self::run(solver, &mut *self.client, &mut *self.event_handler);
        }
    }

    fn run(solver: &Solver, client: &mut dyn Client, events: &mut dyn RunnerEventHandler) {
        tracing::debug!(
            "start running solver day {} year {}",
            solver.day,
            solver.year
        );

        events.on_start_solver(solver);

        let mut event_details = SolverEventDetails::new();

        for part in [Part::One, Part::Two] {
            let solver_part = solver.part(part);

            events.on_start_part(solver, part);

            // Validate examples listed for the current part prior to running the
            // part on real input. Use all of the examples associated with the
            // solver unless the caller has requested a specific example be run.
            let all_examples_start_time = Instant::now();

            let mut pass_count = 0;
            let mut fail_count = 0;

            let examples = solver_part.examples;

            for (index, example) in examples.iter().enumerate() {
                let example_start_time = Instant::now();
                let result = (solver_part.func)(example.input);
                let example_duration = Instant::now() - example_start_time;

                match result {
                    Ok(answer) => {
                        if answer == example.expected {
                            pass_count += 1;
                            events.on_example_pass(solver, part, example_duration, index);
                        } else {
                            fail_count += 1;
                            events.on_example_fail(
                                solver,
                                part,
                                example_duration,
                                index,
                                Ok(answer),
                            );
                        }
                    }
                    Err(err) => {
                        fail_count += 1;
                        events.on_example_fail(
                            solver,
                            part,
                            example_duration,
                            index,
                            Err(err.into()),
                        );
                    }
                }
            }

            // Notify the event manager that examples have passed, otherwise if
            // any have failed then skip running the part with real input.
            let all_examples_duration = Instant::now() - all_examples_start_time;
            events.on_finish_part_examples(
                solver,
                part,
                all_examples_duration,
                pass_count,
                fail_count,
            );

            if fail_count > 0 {
                continue;
            }

            // Fetch input only after examples have passed, but before we start
            // timing the execution of the solver.
            let input = client.get_input(solver.day, solver.year).unwrap();

            // Run the solver against real puzzle input.
            let solve_start_time = Instant::now();
            let run_solver_result = (solver_part.func)(&input);
            let solve_duration = Instant::now() - solve_start_time;

            let part_result = run_solver_result
                .map_err::<RunnerError, _>(|e| e.into())
                .and_then(|answer| {
                    let check_result = client
                        .submit_answer(answer.clone(), part, solver.day, solver.year)
                        .map_err::<RunnerError, _>(|e| e.into())?;
                    Ok((answer, check_result))
                });

            events.on_finish_part(solver, part, solve_duration, &part_result);
            event_details.record_part(part, solve_duration, part_result);
        }

        let run_details: RunDetails = event_details.into();
        events.on_finish_solver(solver, run_details.duration, run_details);
    }
}

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error(transparent)]
    Solver(#[from] SolverError),
}

#[derive(Debug)]
pub struct RunPartDetails {
    pub answer: Answer,
    pub check_result: CheckResult,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct RunDetails {
    pub part_one_result: Option<Result<RunPartDetails, RunnerError>>,
    pub part_two_result: Option<Result<RunPartDetails, RunnerError>>,
    pub duration: Duration,
}

// TODO: measure example time, solver time and account for input time.
#[derive(Debug)]
struct SolverEventDetails {
    pub start_time: Instant,
    pub finish_time: Option<Instant>,
    pub part_one_result: Option<Result<RunPartDetails, RunnerError>>,
    pub part_two_result: Option<Result<RunPartDetails, RunnerError>>,
}

impl SolverEventDetails {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            finish_time: None,
            part_one_result: None,
            part_two_result: None,
        }
    }

    fn record_part(
        &mut self,
        part: Part,
        duration: Duration,
        result: Result<(Answer, CheckResult), RunnerError>,
    ) {
        let field = match part {
            Part::One => &mut self.part_one_result,
            Part::Two => &mut self.part_two_result,
        };

        *field = Some(result.map(|(answer, check_result)| RunPartDetails {
            answer,
            check_result,
            duration,
        }));
    }
}

impl Default for SolverEventDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SolverEventDetails> for RunDetails {
    fn from(value: SolverEventDetails) -> Self {
        RunDetails {
            part_one_result: value.part_one_result,
            part_two_result: value.part_two_result,
            duration: value.finish_time.unwrap_or(Instant::now()) - value.start_time,
        }
    }
}
