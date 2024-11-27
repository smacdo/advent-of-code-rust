#[derive(Clone, Debug)]
pub struct Day(pub usize);

#[derive(Clone, Debug)]
pub struct Year(pub usize);

#[derive(Clone, Debug)]
pub enum Answer {
    NotFinished,
    String(String),
    Int(isize),
}

pub type SolverFn = fn(&str) -> Answer;

#[derive(Clone, Debug)]
pub struct Solver {
    pub day: Day,
    pub year: Year,
    pub title: Option<String>,
    pub part_one: SolverFn,
    pub part_two: SolverFn,
}

pub struct SolverRegistry {
    solvers: Vec<Solver>,
}

impl SolverRegistry {
    pub fn new(solvers: &[Solver]) -> Self {
        Self {
            solvers: solvers.into(),
        }
    }

    pub fn run_all(&self) {
        for s in &self.solvers {
            let a1 = (s.part_one)("");
            let a2 = (s.part_two)("");

            println!(
                "day {}, year {}: part 1 = `{:?}`, part 2 = `{:?}`",
                s.day.0, s.year.0, a1, a2
            );
        }
    }
}
