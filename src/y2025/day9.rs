use advent_of_code_data as aoc;
use ube::{spatial::Point2, utils::pairwise_combinations};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_9_1,
        examples: &[yt::Example {
            input: "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3",
            expected: aoc::Answer::Int(50),
        }],
    },
    part_two: yt::SolverPart {
        func: day_9_2,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

fn parse_tile_locations(input: &str) -> Vec<Point2> {
    input
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(",").unwrap();
            Point2 {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn find_largest_rectangle(points: Vec<Point2>) -> usize {
    pairwise_combinations(&points)
        .map(|(a, b)| {
            let w = (b.x - a.x).abs() + 1;
            let h = (b.y - a.y).abs() + 1;
            (w * h) as usize
        })
        .max()
        .unwrap()
}

pub fn day_9_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Ok(find_largest_rectangle(parse_tile_locations(args.input)).into())
}

pub fn day_9_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}
