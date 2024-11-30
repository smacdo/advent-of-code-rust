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

    pub fn run_all(&mut self) {}
}
