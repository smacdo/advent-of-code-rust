use std::fmt::Display;
use std::iter::repeat;

use advent_of_code_data::registry::{Result, Solver, SolverError, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::utils::find_ints;
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(7),
    year: Year(2024),
    part_one: SolverPart {
        func: day_7_1,
        examples: &[(
            Answer::Int(3749),
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        )],
    },
    part_two: SolverPart {
        func: day_7_2,
        examples: &[],
    },
};

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Mul,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => '+',
                Operator::Mul => '*',
            }
        )
    }
}

#[derive(Debug)]
struct CalibrationEquation {
    pub test_value: i64,
    pub numbers: Vec<i64>,
    pub operators: Vec<Operator>,
}

impl Display for CalibrationEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.numbers[0])?;

        for i in 0..self.operators.len() {
            write!(f, " {} {}", self.operators[i], self.numbers[i + 1])?;
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<CalibrationEquation> {
    input
        .lines()
        .map(|line| {
            let (test_value_str, numbers_str) = line.split_once(':').unwrap();
            let numbers = find_ints(numbers_str);
            let operators: Vec<Operator> = repeat(Operator::Add).take(numbers.len() - 1).collect();

            assert!(numbers.len() > 1);
            assert!(operators.len() == numbers.len() - 1);

            CalibrationEquation {
                test_value: test_value_str.parse::<i64>().unwrap(),
                operators,
                numbers,
            }
        })
        .collect()
}

fn evaluate_all_combinations(equation: &mut CalibrationEquation) -> bool {
    evaluate_all_combinations_itr(equation, 0)
}

fn evaluate_all_combinations_itr(equation: &mut CalibrationEquation, pos: usize) -> bool {
    if pos == equation.operators.len() {
        let evaluation_value = evaluate(&equation.numbers, &equation.operators);

        if evaluation_value == equation.test_value {
            //tracing::debug!("OK  : {equation} => {evaluation_value}");
            true
        } else {
            //tracing::debug!(
            //    "FAIL: {equation} => {evaluation_value} (expected {})",
            //    equation.test_value
            //);
            false
        }
    } else {
        if evaluate_all_combinations_itr(equation, pos + 1) {
            return true;
        }

        equation.operators[pos] = Operator::Mul;
        let did_equal = evaluate_all_combinations_itr(equation, pos + 1);

        equation.operators[pos] = Operator::Add;

        did_equal
    }
}

fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
    assert!(numbers.len() > 1);
    assert!(operators.len() == numbers.len() - 1);

    let mut test_value = numbers[0];

    for i in 0..operators.len() {
        test_value = match operators[i] {
            Operator::Add => test_value + numbers[i + 1],
            Operator::Mul => test_value * numbers[i + 1],
        }
    }

    test_value
}

pub fn day_7_1(input: &str) -> Result<Answer> {
    let equations = parse_input(input);
    let mut total_calibration_result = 0;

    for mut equation in equations {
        if evaluate_all_combinations(&mut equation) {
            total_calibration_result += equation.test_value;
        }
    }

    Ok(total_calibration_result.into())
}

pub fn day_7_2(input: &str) -> Result<Answer> {
    Err(SolverError::NotFinished)
}
