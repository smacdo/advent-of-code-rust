use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;
use regex::Regex;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::Solver = yt::Solver {
    day: aoc::Day(3),
    year: aoc::Year(2024),
    part_one: yt::SolverPart {
        func: day_3_1,
        examples: &[(yt::Example {
            input: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            expected: aoc::Answer::Int(161),
        })],
    },
    part_two: yt::SolverPart {
        func: day_3_2,
        examples: &[(yt::Example {
            input: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            expected: aoc::Answer::Int(48),
        })],
    },
};

pub fn day_3_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul_sum: i64 = 0;

    for (_, [left, right]) in re.captures_iter(args.input).map(|c| c.extract()) {
        mul_sum += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
    }

    Ok(mul_sum.into())
}

pub fn day_3_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let re = Regex::new(
        r"(?<mul>mul\((?<left>\d{1,3}),(?<right>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();

    let mut mul_sum: i64 = 0;
    let mut mul_enabled = true;

    for c in re.captures_iter(args.input) {
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
