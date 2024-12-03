use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
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

pub fn day_3_1(input: &str) -> Result<Answer> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul_sum: i64 = 0;

    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        mul_sum += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
    }

    Ok(mul_sum.into())
}

pub fn day_3_2(input: &str) -> Result<Answer> {
    let re = Regex::new(
        r"(?<mul>mul\((?<left>\d{1,3}),(?<right>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();

    let mut mul_sum: i64 = 0;
    let mut mul_enabled = true;

    for c in re.captures_iter(input) {
        if c.name("mul").is_some() {
            if mul_enabled {
                let left = c.name("left").unwrap().as_str();
                let right = c.name("right").unwrap().as_str();

                mul_sum += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
            }
        } else if c.name("do").is_some() {
            mul_enabled = true;
        } else if c.name("dont").is_some() {
            mul_enabled = false;
        }
    }

    Ok(mul_sum.into())
}
