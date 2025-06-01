use std::collections::HashMap;

use advent_of_code_data as aoc;
use yuletide as yt;

use advent_of_code_rust::utils;
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_11_1,
        examples: &[yt::Example {
            input: "125 17",
            expected: aoc::Answer::Int(55312),
        }],
    },
    part_two: yt::SolverPart {
        func: day_11_2,
        examples: &[yt::Example {
            input: "125 17",
            expected: aoc::Answer::Int(65601038650482),
        }],
    },
};

fn blink(stones: &HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut new_stones: HashMap<i64, usize> = HashMap::new();

    for (stone, count) in stones {
        // NOTE: ilog10 counts the number of digits
        // https://stackoverflow.com/a/24177168
        let digit_count = stone.checked_ilog10().map(|x| x + 1);

        match digit_count {
            None => {
                *new_stones.entry(1).or_default() += count;
            }
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
                *new_stones.entry(left).or_default() += count;
                *new_stones.entry(right).or_default() += count;
            }
            Some(_) => {
                *new_stones.entry(*stone * 2024).or_default() += count;
            }
        }
    }

    new_stones
}

fn parse_input(input: &str) -> HashMap<i64, usize> {
    let mut new_stones: HashMap<i64, usize> = HashMap::new();

    for stone in utils::find_ints(input) {
        *new_stones.entry(stone).or_default() += 1;
    }

    new_stones
}

pub fn day_11_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut stones = parse_input(args.input);

    for _ in 0..25 {
        stones = blink(&stones);
    }

    Ok(stones.values().sum::<usize>().into())
}

pub fn day_11_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut stones = parse_input(args.input);

    for _ in 0..75 {
        stones = blink(&stones);
        //println!("{i} has {} stones", stones.values().sum::<usize>());
    }

    Ok(stones.values().sum::<usize>().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let stones = parse_input("0 1 10 99 999");
        assert_eq!(blink(&stones), parse_input("1 2024 1 0 9 9 2021976"));

        //assert_eq!(blink(&stones), vec![]);
    }
}
