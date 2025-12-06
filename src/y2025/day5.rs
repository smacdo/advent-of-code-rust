use std::ops::RangeInclusive;

use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_5_1,
        examples: &[yt::Example {
            input: "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32",
            expected: aoc::Answer::Int(3),
        }],
    },
    part_two: yt::SolverPart {
        func: day_5_2,
        examples: &[yt::Example {
            input: "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32",
            expected: aoc::Answer::Int(14),
        }],
    },
};

#[derive(Debug, PartialEq)]
struct Database {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

fn parse_range(input: &str) -> (usize, usize) {
    let (first, last) = input.split_once("-").expect("expected -");
    (
        first
            .parse()
            .unwrap_or_else(|_| panic!("invalid number for first `{first}`")),
        last.parse()
            .unwrap_or_else(|_| panic!("invalid number of last `{last}`")),
    )
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by_key(|r| *r.start());
    //eprintln!("RANGES: {ranges:?}");

    let mut merged: Vec<RangeInclusive<usize>> = Vec::new();

    if let Some(first_element) = ranges.first().cloned() {
        merged.push(first_element);

        for i in 1..ranges.len() {
            let current = ranges[i].clone();
            let merged_last = merged.last().unwrap().clone();

            if current.start() <= merged_last.end() {
                let start = merged_last.start();
                let last = if current.end() > merged_last.end() {
                    current.end()
                } else {
                    merged_last.end()
                };

                let last_index = merged.len() - 1;
                merged[last_index] = *start..=*last;
            } else {
                merged.push(current);
            }
        }
    }

    merged
}

fn parse_database(input: &str) -> Database {
    let mut db = Database {
        fresh_ranges: Vec::new(),
        ingredients: Vec::new(),
    };

    let (fresh_range_lines, ingredients_lines) = input.split_once("\n\n").unwrap();

    for line in fresh_range_lines.lines() {
        let (first, last) = parse_range(line);
        db.fresh_ranges.push(first..=last);
    }

    for line in ingredients_lines.lines() {
        db.ingredients.push(line.parse().expect("usize"));
    }

    db
}

pub fn day_5_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let db = parse_database(args.input);
    let mut sum = 0;

    for ingredient in &db.ingredients {
        for r in &db.fresh_ranges {
            if r.contains(ingredient) {
                sum += 1;
                break;
            }
        }
    }

    Ok(sum.into())
}

pub fn day_5_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let db = parse_database(args.input);
    let ranges = merge_ranges(db.fresh_ranges);
    let mut sum: usize = 0;

    for r in ranges {
        //eprintln!("{}-{}", r.start(), r.end());
        sum += r.end() - r.start() + 1;
    }

    Ok(sum.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_to_database() {
        assert_eq!(
            parse_database("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32"),
            Database {
                fresh_ranges: vec![(3..=5), (10..=14), (16..=20), (12..=18)],
                ingredients: vec![1, 5, 8, 11, 17, 32]
            }
        );
    }

    #[test]
    fn merge_ranges_test() {
        assert_eq!(
            merge_ranges(vec![(2..=6), (8..=10), (1..=3), (15..=18)]),
            vec![(1..=6), (8..=10), (15..=18)]
        );
    }
}
