use std::collections::HashMap;

use regex::Regex;
use thiserror::Error;

use advent_of_code_data::{Answer, Day, Part, Year};

use crate::SolverArgs;

/// Represents an error that can happen when running an Advent of Code solver.
#[derive(Error, Debug)]
pub enum SolverError {
    #[error("this solver is not finished")]
    NotFinished,
    #[error("the answer was submitted too soon after an incorrect answer, please wait before trying again")]
    TooSoon,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// The result of running an Advent of Code solver.
pub type Result<T> = core::result::Result<T, SolverError>;

/// A function that solves an Advent of Code puzzle part.
pub type SolverPartFn = fn(&SolverArgs) -> Result<Answer>;

/// Contains registration information for a puzzle solver, and is used at start
/// up to create a directory of puzzle solvers.
#[derive(Clone, Debug)]
pub struct SolverAutoRegister {
    /// Module path containing the solver.
    ///
    /// The module path must end in the form `yYYYY::dayD` where `YYYY` is the
    /// puzzle year, and `D` is the day.
    ///
    /// Example: `my_aoc_solutions::y2024::day15`
    pub modpath: &'static str,
    /// A function that solves part one of the puzzle.
    pub part_one: SolverPart,
    /// A function that solves part two of the puzzle.
    pub part_two: SolverPart,
}

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
    /// Create a `SolverRegistry` from a set of partial solver registration
    /// entries.
    pub fn compiled_from(all_solvers: &[SolverAutoRegister]) -> Self {
        // TODO: Support multiple solvers for the same year + day (e.g., alternative solutions).
        let mut solvers: HashMap<Year, HashMap<Day, Solver>> = Default::default();
        let re = Regex::new(r"::y(?<year>\d{4,4})::day(?<day>(\d+))$").unwrap();

        for registration in all_solvers.iter() {
            // Parse the puzzle year and day from the module path.
            let captures = re
                .captures(registration.modpath)
                .expect("module path must be in expected form ::yYYYY::dayD");

            let year = Year(
                captures
                    .name("year")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            );

            let day = Day(captures
                .name("day")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap());

            let solver = Solver {
                year,
                day,
                part_one: registration.part_one.clone(),
                part_two: registration.part_two.clone(),
            };

            match solvers.get_mut(&year) {
                Some(solvers_for_year) => {
                    solvers_for_year.insert(day, solver);
                }
                None => {
                    solvers.insert(year, HashMap::from([(day, solver)]));
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

    fn test_part(_args: &SolverArgs) -> Result<Answer> {
        Err(SolverError::NotFinished)
    }

    fn create_solver(modpath: &'static str) -> SolverAutoRegister {
        SolverAutoRegister {
            modpath,
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
            create_solver("testcrate::y2024::day1"),
            create_solver("testcrate::y2024::day1"),
            create_solver("testcrate::y2024::day5"),
            create_solver("testcrate::y2025::day1"),
            create_solver("testcrate::y1999::day1"),
        ]);
        assert_eq!(registry.years(), vec![Year(1999), Year(2024), Year(2025)]);
    }

    #[test]
    fn days_empty_if_no_solvers() {
        let registry = SolverRegistry::compiled_from(&[create_solver("testcrate::y2024::day1")]);
        assert!(registry.days(Year(2023)).is_none());
        assert!(registry.days(Year(2024)).is_some());
    }

    #[test]
    fn days_are_only_for_the_requested_year() {
        // Include duplicate days, and days that are out of order.
        let registry = SolverRegistry::compiled_from(&[
            create_solver("testcrate::y2024::day1"),
            create_solver("testcrate::y2024::day1"),
            create_solver("testcrate::y2024::day7"),
            create_solver("testcrate::y2024::day5"),
            create_solver("testcrate::y2025::day1"),
            create_solver("testcrate::y1999::day19"),
            create_solver("testcrate::y1999::day15"),
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
        let registry = SolverRegistry::compiled_from(&[create_solver("testcrate::y2024::day1")]);
        assert!(registry.solver(Year(2025), Day(1)).is_none());
        assert!(registry.solver(Year(2024), Day(2)).is_none());
        assert!(registry.solver(Year(2024), Day(1)).is_some());
    }

    #[test]
    fn get_solver_matches_day_and_year() {
        let registry = SolverRegistry::compiled_from(&[
            create_solver("testcrate::y2024::day1"),
            create_solver("testcrate::y2024::day5"),
            create_solver("testcrate::y2025::day2"),
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
