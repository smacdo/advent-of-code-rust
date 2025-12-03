use std::collections::HashMap;

use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;
use noclip::utils::find_ints;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_1_1,
        examples: &[yt::Example {
            input: "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n",
            expected: aoc::Answer::Int(11),
        }],
    },
    part_two: yt::SolverPart {
        func: day_1_2,
        examples: &[yt::Example {
            input: "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n",
            expected: aoc::Answer::Int(31),
        }],
    },
};

pub fn day_1_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    // Read input into two lists - left side numbers and right side numbers.
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();

    for line in args.input.lines() {
        let numbers = find_ints(line);

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    // Sort both lists in ascending order.
    left.sort();
    right.sort();

    // Sum the distance between each pair.
    let mut total_distance = 0;

    for (a, b) in left.into_iter().zip(right.into_iter()) {
        let distance = (a - b).abs();
        total_distance += distance;
    }

    Ok(total_distance.into())
}

pub fn day_1_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    // Read input into two lists - left side numbers and right side numbers.
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();

    for line in args.input.lines() {
        let numbers = find_ints(line);

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    // Calculate how many times each number occurs in the right list.
    let mut counts: HashMap<i64, i64> = HashMap::new();

    for n in right {
        *counts.entry(n).or_default() += 1;
    }

    // Calculate total similarity of the two lists by summing the product of
    // each left value times the number of times it appears in the right.
    let mut total_similarity = 0;

    for n in left {
        let similarity = n * counts.get(&n).unwrap_or(&0);
        total_similarity += similarity;
    }

    Ok(total_similarity.into())
}
