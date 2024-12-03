use advent_of_code_data::registry::{Result, Solver, SolverError, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::utils;
use linkme::distributed_slice;
use regex::Regex;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(3),
    year: Year(2024),
    part_one: SolverPart {
        func: day_3_1,
        examples: &[(
            Answer::Int(161),
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        )],
    },
    part_two: SolverPart {
        func: day_3_2,
        examples: &[(
            Answer::Int(48),
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )],
    },
};

// TODO: no need to iterate on lines

pub fn day_3_1(input: &str) -> Result<Answer> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul_sum: i64 = 0;

    for line in input.lines() {
        for (_, [left, right]) in re.captures_iter(line).map(|c| c.extract()) {
            mul_sum += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
        }
    }

    Ok(mul_sum.into())
}

pub fn day_3_2(input: &str) -> Result<Answer> {
    let re =
        Regex::new(r"(?<cmd>(mul)|(do)|(don't))\(((?<left>\d{1,3}),(?<right>\d{1,3}))?\)").unwrap();
    let mut mul_sum: i64 = 0;
    let mut mul_enabled = true;

    for line in input.lines() {
        for c in re.captures_iter(line) {
            match c.name("cmd").unwrap().as_str() {
                "mul" => {
                    match (c.name("left"), c.name("right")) {
                        (Some(left), Some(right)) => {
                            if mul_enabled {
                                mul_sum += left.as_str().parse::<i64>().unwrap()
                                    * right.as_str().parse::<i64>().unwrap();
                            }
                        }
                        _ => {
                            // invalid - mul is missing required left/right args. Ignore!
                        }
                    }
                }
                "do" => {
                    if c.name("left").is_none() && c.name("right").is_none() {
                        mul_enabled = true;
                    } else {
                        // invalid - do cannot have args. Ignore!
                    }
                }
                "don't" => {
                    if c.name("left").is_none() && c.name("right").is_none() {
                        mul_enabled = false;
                    } else {
                        // invalid - do cannot have args. Ignore!
                    }
                }
                _ => {
                    // unknown command, ignore!
                }
            }
        }
    }

    Ok(mul_sum.into())
}
