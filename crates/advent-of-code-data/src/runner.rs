use crate::{
    client::Client,
    data::CheckResult,
    registry::{Solver, SolverError},
    Answer, Part,
};

pub trait RunnerEventHandler {
    fn on_start_solver(&mut self, solver: &Solver);
    fn on_part_examples_pass(&mut self, solver: &Solver, part: Part, count: usize);
    fn on_start_part(&mut self, solver: &Solver, part: Part);
    fn on_finish_part(
        &mut self,
        solver: &Solver,
        part: Part,
        result: &Result<(Answer, CheckResult), SolverError>,
    );
    fn on_finish_solver(&mut self, solver: &Solver);
}

pub struct SolverRunner {
    solvers_to_run: Vec<Solver>,
    client: Box<dyn Client>,
    event_handler: Box<dyn RunnerEventHandler>,
}

impl SolverRunner {
    pub fn new(client: Box<dyn Client>, event_handler: Box<dyn RunnerEventHandler>) -> Self {
        Self {
            solvers_to_run: Vec::new(),
            client,
            event_handler,
        }
    }

    pub fn push(&mut self, solver: Solver) {
        self.solvers_to_run.push(solver);
    }

    pub fn run_all(&mut self) {
        for solver in &self.solvers_to_run {
            Self::run(solver, &mut *self.client, &mut *self.event_handler);
        }
    }

    fn run(solver: &Solver, client: &mut dyn Client, events: &mut dyn RunnerEventHandler) {
        let parts = [Part::One, Part::Two];

        events.on_start_solver(solver);

        for part in parts {
            let solver_part = solver.part(part);

            events.on_start_part(solver, part);

            // Validate examples listed for the current part prior to running the
            // part on real input. Use all of the examples associated with the
            // solver unless the caller has requested a specific example be run.
            let mut examples_pass = true;
            let examples = solver_part.examples;

            for example in examples {
                let result = (solver_part.func)(example.1);

                match result {
                    Ok(answer) => {
                        if answer != example.0 {
                            // Example failed - set the result for this part as
                            // "example failed". Stop testing examples for this part.
                            events.on_finish_part(
                                solver,
                                part,
                                &Err(SolverError::ExampleFailed {
                                    part,
                                    input: example.1.to_string(),
                                    expected: example.0.clone(),
                                    actual: answer,
                                }),
                            );

                            examples_pass = false;
                            break;
                        }
                    }
                    Err(err) => {
                        panic!("TODO: handle errors during examples: {err:?}")
                    }
                }
            }

            // Notify the event manager that examples have passed, otherwise if
            // any have failed then skip running the part with real input.
            if examples_pass {
                events.on_part_examples_pass(solver, part, examples.len())
            } else {
                continue;
            }

            // Run the solver against real puzzle input.
            let input = client.get_input(solver.day, solver.year);
            let solver_result = (solver_part.func)(&input);

            let final_result = match solver_result {
                Ok(answer) => {
                    let check_result =
                        client.submit_answer(answer.clone(), part, solver.day, solver.year);

                    Ok((answer, check_result))
                }
                Err(err) => Err(err),
            };

            events.on_finish_part(solver, part, &final_result);
        }

        events.on_finish_solver(solver);
    }
}
