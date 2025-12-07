use std::str::FromStr;

use advent_of_code_data as aoc;
use ube::spatial::Grid;
use yuletide::{self as yt};

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
        examples: &[yt::Example {
            input: "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
            expected: aoc::Answer::Int(3263827),
        }],
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

    for col in 0..col_count {
        let numbers = lines[0..(lines.len() - 1)]
            .iter()
            .map(|line| line[col].parse().unwrap())
            .collect::<Vec<_>>();

        problems.push(match lines[lines.len() - 1][col] {
            "+" => MathProblem::Add(numbers),
            "*" => MathProblem::Mul(numbers),
            _ => panic!("unknown math op"),
        });
    }

    problems
}

fn parse_math_problems_p2(input: &str) -> Vec<MathProblem> {
    let grid: Grid<char> = Grid::from_str(input).unwrap();
    let mut problems: Vec<MathProblem> = Vec::new();
    let mut values: Vec<usize> = Vec::new();
    let mut skip_next_col = false;

    // TODO: column iterator?
    for x in (0..grid.x_count()).rev() {
        let mut current_value: usize = 0;

        if skip_next_col {
            skip_next_col = false;
            continue;
        }

        // rows 0..(len-1) contain the digits for the number. Ignore the row if
        // it is empty.
        for y in 0..(grid.y_count() - 1) {
            match grid.get(x as isize, y as isize) {
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as usize;
                    current_value = current_value * 10 + digit;
                }
                c if *c == ' ' => {}
                _ => panic!("unrecogonized character when parsing digit"),
            };
        }

        // The last row in the column either an operator meaning this is the last value in the
        // problem, or blank meaning there are more values for the problem.
        // TODO: I'm thinking that grid indices should always be usize. Support negative indices with a higher level class "tilemap" similiar to the idea of suporting tile sizes.
        values.push(current_value);

        match grid.get(x as isize, (grid.y_count() - 1) as isize) {
            ' ' => {}
            '+' => {
                problems.push(MathProblem::Add(values));
                values = Vec::new();
                skip_next_col = true;
            }
            '*' => {
                problems.push(MathProblem::Mul(values));
                values = Vec::new();
                skip_next_col = true;
            }
            _ => panic!("unknown math operator char"),
        };
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

pub fn day_6_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Ok(parse_math_problems_p2(args.input)
        .iter()
        .map(|problem| problem.answer())
        .sum::<usize>()
        .into())
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

    #[test]
    fn parse_problems_p2() {
        assert_eq!(
            parse_math_problems_p2(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            ),
            vec![
                MathProblem::Add(vec![4, 431, 623]),
                MathProblem::Mul(vec![175, 581, 32]),
                MathProblem::Add(vec![8, 248, 369]),
                MathProblem::Mul(vec![356, 24, 1])
            ]
        );
    }
}
