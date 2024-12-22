use std::collections::HashMap;

use thiserror::Error;

use crate::{Answer, Day, Part, Year};

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("there is not answer because the solver is not finished")]
    NotFinished,
    #[error("an example input did not match the expected output")]
    ExampleFailed {
        part: Part,
        input: String,
        expected: Answer,
        actual: Answer,
    },
    #[error("the answer was submitted too soon, please wait before trying again")]
    TooSoon,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = core::result::Result<T, SolverError>;
pub type SolverFn = fn(&str) -> Result<Answer>;

#[derive(Clone, Debug)]
pub struct Solver {
    pub day: Day,
    pub year: Year,
    pub part_one: SolverPart,
    pub part_two: SolverPart,
}

impl Solver {
    pub fn part(&self, part: Part) -> &SolverPart {
        match part {
            Part::One => &self.part_one,
            Part::Two => &self.part_two,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SolverPart {
    pub func: SolverFn,
    pub examples: &'static [(Answer, &'static str)],
}

pub struct SolverRegistry {
    solvers: HashMap<Year, HashMap<Day, Solver>>,
}

impl SolverRegistry {
    pub fn new(all_solvers: &[Solver]) -> Self {
        // TODO: Merge duplicate entries to allow for split part 1 / part 2 registration.
        // TODO: Sort so order is stable.
        let mut solvers: HashMap<Year, HashMap<Day, Solver>> = Default::default();

        for s in all_solvers.iter() {
            match solvers.get_mut(&s.year) {
                Some(solvers_for_year) => {
                    solvers_for_year.insert(s.day, s.clone());
                }
                None => {
                    solvers.insert(s.year, HashMap::from([(s.day, s.clone())]));
                }
            }
        }

        Self { solvers }
    }

    pub fn years(&self) -> Vec<Year> {
        // TODO: return iterator to avoid Vec allocation.
        self.solvers.keys().cloned().collect()
    }

    pub fn days(&self, year: Year) -> Vec<Day> {
        self.solvers
            .get(&year)
            .expect("TODO: handle if year is empty")
            .values()
            .map(|s| s.day)
            .collect()
    }

    pub fn solver(&self, day: Day, year: Year) -> &Solver {
        &self.solvers[&year][&day]
    }
}

// TODO: Unit tests.
