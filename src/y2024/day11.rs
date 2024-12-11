use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::utils;
use either::{Left, Right};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(11),
    year: Year(2024),
    part_one: SolverPart {
        func: day_11_1,
        examples: &[(Answer::Int(55312), "125 17")],
    },
    part_two: SolverPart {
        func: day_11_2,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
};

fn blink(stones: &[i64]) -> Vec<i64> {
    stones
        .iter()
        .flat_map(|stone| {
            // NOTE: ilog10 counts the number of digits
            // https://stackoverflow.com/a/24177168
            let digit_count = stone.checked_ilog10().map(|x| x + 1);
            // println!("{stone} digit count is {digit_count:?}");

            match digit_count {
                None => Left(std::iter::once(1)),
                Some(digit_count) if digit_count % 2 == 0 => {
                    // Extract left and right sides.
                    let half_digit_count = digit_count / 2;
                    let mut left = *stone;
                    let mut right = 0;

                    for i in 0..half_digit_count {
                        let digit = left % 10;
                        left /= 10;
                        right += digit * 10_i64.pow(i);
                    }

                    // println!("{stone} splits into {left} and {right}");

                    Right(std::iter::once(left).chain(std::iter::once(right)))
                }
                Some(_) => Left(std::iter::once(*stone * 2024)),
            }
        })
        .collect()
}

pub fn day_11_1(input: &str) -> Result<Answer> {
    let mut stones = utils::find_ints(input);

    for _ in 0..25 {
        stones = blink(&stones);
    }

    Ok(stones.len().into())
}

pub fn day_11_2(input: &str) -> Result<Answer> {
    let mut stones = utils::find_ints(input);

    for i in 0..75 {
        stones = blink(&stones);
        println!("{i} has {} stones", stones.len());
    }

    Ok(stones.len().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let stones = utils::find_ints("0 1 10 99 999");
        assert_eq!(blink(&stones), vec![1, 2024, 1, 0, 9, 9, 2021976]);

        //assert_eq!(blink(&stones), vec![]);
    }
}
