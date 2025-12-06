use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_6_1,
        examples: &[yt::Example {
            input: "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
            expected: aoc::Answer::Int(4277556),
        }],
    },
    part_two: yt::SolverPart {
        func: day_6_2,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

#[derive(Debug, PartialEq)]
enum MathProblem {
    Add(Vec<usize>),
    Mul(Vec<usize>),
}

impl MathProblem {
    pub fn answer(&self) -> usize {
        match self {
            MathProblem::Add(numbers) => numbers.iter().sum(),
            MathProblem::Mul(numbers) => numbers.iter().product(),
        }
    }
}

fn parse_math_problems_p1(input: &str) -> Vec<MathProblem> {
    // Convert input into a row major matrix of numbers, where numbers are separated by space.
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Count the number of columns, and verify it is consistent across rows.
    let col_count = lines[0].len();

    for line in &lines {
        assert!(line.len() == col_count);
    }

    // Convert each column into a math problem.
    let mut problems: Vec<MathProblem> = Vec::new();

    for j in 0..col_count {
        let numbers = lines[0..(lines.len() - 1)]
            .iter()
            .map(|line| line[j].parse().unwrap())
            .collect::<Vec<_>>();

        problems.push(match lines[lines.len() - 1][j] {
            "+" => MathProblem::Add(numbers),
            "*" => MathProblem::Mul(numbers),
            _ => panic!("unknown math op"),
        });
    }

    problems
}

pub fn day_6_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Ok(parse_math_problems_p1(args.input)
        .iter()
        .map(|problem| problem.answer())
        .sum::<usize>()
        .into())
}

pub fn day_6_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_problems_input() {
        assert_eq!(
            parse_math_problems_p1("19  130\n110 3\n+    *"),
            vec![
                MathProblem::Add(vec![19, 110]),
                MathProblem::Mul(vec![130, 3])
            ]
        );
    }
}
