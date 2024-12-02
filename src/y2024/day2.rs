use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::utils;
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(2),
    year: Year(2024),
    part_one: SolverPart {
        func: day_2_1,
        examples: &[(
            Answer::Int(2),
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        )],
    },
    part_two: SolverPart {
        func: day_2_2,
        examples: &[],
    },
};

fn check_is_safe(levels: &[i64]) -> bool {
    levels
        .windows(2) // Iterate over [n0, n1], [n1, n2], ...
        .map(|ab| ab[1] - ab[0]) // Calculate distance between [n, n+1]
        .map(|delta| {
            // Marker distances larger than allowed as unsafe.
            if delta.abs() >= 1 && delta.abs() <= 3 {
                Some(delta)
            } else {
                None
            }
        })
        .reduce(|acc, e| match (acc, e) {
            // Reject any safe values that switch from increasing to decreasing
            // or vice versa.
            (Some(prev), Some(next)) => {
                if (prev >= 0) == (next >= 0) {
                    Some(next)
                } else {
                    None
                }
            }
            _ => None,
        })
        .expect("input is expected to have at least two levels")
        .is_some()
}

pub fn day_2_1(input: &str) -> Result<Answer> {
    let mut safe_reports_count = 0;

    for report in input.lines() {
        let levels = utils::find_ints(report);

        if check_is_safe(&levels) {
            safe_reports_count += 1
        }
    }

    Ok(safe_reports_count.into())
}

pub fn day_2_2(input: &str) -> Result<Answer> {
    let mut safe_reports_count = 0;

    for report in input.lines() {
        let levels = utils::find_ints(report);

        if check_is_safe(&levels) {
            safe_reports_count += 1
        } else {
            // This is SUPER brute force and gross but hey it's worth a first
            // attempt. I can always return later when I've thought of a nice
            // way to do this.
            for index_to_delete in 0..levels.len() {
                let dont_worry_about_it: Vec<i64> = levels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index_to_delete)
                    .map(|(_, x)| *x)
                    .collect();

                if check_is_safe(&dont_worry_about_it) {
                    safe_reports_count += 1;
                    break;
                }
            }
        }
    }

    Ok(safe_reports_count.into())
}
