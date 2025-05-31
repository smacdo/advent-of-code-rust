use std::collections::HashMap;

use thiserror::Error;

use crate::{Answer, Day, Part, Year};

/// Represents an error that can happen when running an Advent of Code solver.
#[derive(Error, Debug)]
pub enum SolverError {
    #[error("there is not answer because the solver is not finished")]
    NotFinished,
    #[error("the answer was submitted too soon, please wait before trying again")]
    TooSoon,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// The result of running an Advent of Code solver.
pub type Result<T> = core::result::Result<T, SolverError>;

/// A function that solves an Advent of Code puzzle part.
pub type SolverPartFn = fn(&str) -> Result<Answer>;

/// A `Solver` is runnable Advent of Code puzzle solution for a given year and
/// day.
#[derive(Clone, Debug)]
pub struct Solver {
    /// Advent of Code puzzle day.
    pub day: Day,
    /// Advent of Code puzzle year.
    pub year: Year,
    /// A function that solves part one.
    pub part_one: SolverPart,
    /// A function that solves part two.
    pub part_two: SolverPart,
}

impl Solver {
    /// Get a function that solves for the requested puzzle part.
    pub fn part(&self, part: Part) -> &SolverPart {
        match part {
            Part::One => &self.part_one,
            Part::Two => &self.part_two,
        }
    }
}

/// Holds the function that solves for an Advent of Code puzzle part, and any
/// additional metadata.
#[derive(Clone, Debug)]
pub struct SolverPart {
    pub func: SolverPartFn,
    pub examples: &'static [Example],
}

impl SolverPart {
    /// Get a specific example stored in the solver part's metadata.
    pub fn example(&self, index: usize) -> &Example {
        &self.examples[index]
    }
}

/// Defines the exepcted answer for a given input. Examples are specific to an
/// Advent of Calendar puzzle.
#[derive(Clone, Debug)]
pub struct Example {
    /// The puzzle input.
    pub input: &'static str,
    /// The expected answer.
    pub expected: Answer,
}

/// Holds a collection of puzzle solvers that can be looked up by year and day.
pub struct SolverRegistry {
    solvers: HashMap<Year, HashMap<Day, Solver>>,
}

impl SolverRegistry {
    /// Create a new `SolverRegistry` from a collection of existing solvers.
    pub fn compiled_from(all_solvers: &[Solver]) -> Self {
        // TODO: Support multiple solvers for the same year + day (e.g., alternative solutions).
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

    /// Get a list of the years for which there is a `Solver` in the registry.
    pub fn years(&self) -> Vec<Year> {
        let mut y: Vec<_> = self.solvers.keys().cloned().collect();
        y.sort();
        y
    }

    /// Get a list of the days for the provided `year` for which there is a
    /// `Solver` in the registry.
    pub fn days(&self, year: Year) -> Option<Vec<Day>> {
        let mut d: Vec<_> = self.solvers.get(&year)?.values().map(|s| s.day).collect();
        d.sort();
        Some(d)
    }

    /// Get the solver for the requested `day` and `year`.
    pub fn solver(&self, year: Year, day: Day) -> Option<&Solver> {
        self.solvers
            .get(&year)
            .and_then(|solvers_for_year| solvers_for_year.get(&day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part(_input: &str) -> Result<Answer> {
        Err(SolverError::NotFinished)
    }

    fn create_solver(year: Year, day: Day) -> Solver {
        Solver {
            day,
            year,
            part_one: SolverPart {
                func: test_part,
                examples: &[],
            },
            part_two: SolverPart {
                func: test_part,
                examples: &[],
            },
        }
    }

    #[test]
    fn years_empty_if_no_solvers() {
        let registry = SolverRegistry::compiled_from(&[]);
        assert!(registry.years().is_empty());
    }

    #[test]
    fn years_are_from_solvers_in_registry() {
        // Include duplicate years, and years that are out of order.
        let registry = SolverRegistry::compiled_from(&[
            create_solver(Year(2024), Day(1)),
            create_solver(Year(2024), Day(1)),
            create_solver(Year(2024), Day(5)),
            create_solver(Year(2025), Day(1)),
            create_solver(Year(1999), Day(1)),
        ]);
        assert_eq!(registry.years(), vec![Year(1999), Year(2024), Year(2025)]);
    }

    #[test]
    fn days_empty_if_no_solvers() {
        let registry = SolverRegistry::compiled_from(&[create_solver(Year(2024), Day(1))]);
        assert!(registry.days(Year(2023)).is_none());
        assert!(registry.days(Year(2024)).is_some());
    }

    #[test]
    fn days_are_only_for_the_requested_year() {
        // Include duplicate days, and days that are out of order.
        let registry = SolverRegistry::compiled_from(&[
            create_solver(Year(2024), Day(1)),
            create_solver(Year(2024), Day(1)),
            create_solver(Year(2024), Day(7)),
            create_solver(Year(2024), Day(5)),
            create_solver(Year(2025), Day(1)),
            create_solver(Year(1999), Day(19)),
            create_solver(Year(1999), Day(15)),
        ]);

        assert_eq!(registry.days(Year(1999)), Some(vec![Day(15), Day(19)]));
        assert_eq!(
            registry.days(Year(2024)),
            Some(vec![Day(1), Day(5), Day(7)])
        );
        assert_eq!(registry.days(Year(2025)), Some(vec![Day(1)]));
    }

    #[test]
    fn get_solver_empty_if_year_or_day_do_not_exist() {
        let registry = SolverRegistry::compiled_from(&[create_solver(Year(2024), Day(1))]);
        assert!(registry.solver(Year(2025), Day(1)).is_none());
        assert!(registry.solver(Year(2024), Day(2)).is_none());
        assert!(registry.solver(Year(2024), Day(1)).is_some());
    }

    #[test]
    fn get_solver_matches_day_and_year() {
        let registry = SolverRegistry::compiled_from(&[
            create_solver(Year(2024), Day(1)),
            create_solver(Year(2024), Day(5)),
            create_solver(Year(2025), Day(2)),
        ]);

        assert_eq!(
            registry.solver(Year(2024), Day(1)).unwrap().year,
            Year(2024)
        );
        assert_eq!(registry.solver(Year(2024), Day(1)).unwrap().day, Day(1));

        assert_eq!(
            registry.solver(Year(2024), Day(5)).unwrap().year,
            Year(2024)
        );
        assert_eq!(registry.solver(Year(2024), Day(5)).unwrap().day, Day(5));

        assert_eq!(
            registry.solver(Year(2025), Day(2)).unwrap().year,
            Year(2025)
        );
        assert_eq!(registry.solver(Year(2025), Day(2)).unwrap().day, Day(2));
    }
}
