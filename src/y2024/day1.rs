use std::collections::HashMap;

use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::utils::find_ints;
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(1),
    year: Year(2024),
    part_one: SolverPart {
        func: day_1_1,
        examples: &[(
            Answer::Int(11),
            "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n",
        )],
    },
    part_two: SolverPart {
        func: day_1_2,
        examples: &[(
            Answer::Int(31),
            "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n",
        )],
    },
};

pub fn day_1_1(input: &str) -> Result<Answer> {
    // Read input into two lists - left side numbers and right side numbers.
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();

    for line in input.lines() {
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

    Ok(Answer::Int(total_distance))
    //Err(SolverError::NotFinished)
}

pub fn day_1_2(input: &str) -> Result<Answer> {
    // Read input into two lists - left side numbers and right side numbers.
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();

    for line in input.lines() {
        let numbers = find_ints(line);

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    // Calculate how many times each number appears in the two lists.
    fn count_occurences(nums: &[i64]) -> HashMap<i64, i64> {
        let mut counts: HashMap<i64, i64> = HashMap::new();

        for n in nums {
            counts.entry(*n).and_modify(|e| *e += 1).or_insert(1);
        }

        counts
    }

    let counts = count_occurences(&right);
    let mut total_similarity = 0;

    for n in left {
        let similarity = n * counts.get(&n).unwrap_or(&0);
        total_similarity += similarity;
    }

    Ok(Answer::Int(total_similarity))
}
