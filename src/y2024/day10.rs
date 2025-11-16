use std::collections::{BinaryHeap, HashSet};

use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;
use noclip::spatial::{Direction4, Grid, Point2};

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_10_1,
        examples: &[yt::Example {
            input: "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
            expected: aoc::Answer::Int(36),
        }],
    },
    part_two: yt::SolverPart {
        func: day_10_2,
        examples: &[yt::Example {
            input: "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
            expected: aoc::Answer::Int(81),
        }],
    },
};

fn count_trailheads(trailhead_pos: Point2, map: &Grid<usize>, allow_multiple: bool) -> usize {
    let mut frontier: BinaryHeap<Point2> = BinaryHeap::new();

    if map[trailhead_pos] == 0 {
        frontier.push(trailhead_pos);
    }

    let mut peaks: HashSet<Point2> = HashSet::new();
    let mut trailhead_count: usize = 0;

    while let Some(next_pos) = frontier.pop() {
        if map[next_pos] == 9 && (allow_multiple || !peaks.contains(&next_pos)) {
            peaks.insert(next_pos);
            trailhead_count += 1;
        }

        for neighbor_dir in Direction4::all() {
            let neighbor_pos = next_pos + neighbor_dir;

            if map.is_pos_in_bounds(neighbor_pos) && map[neighbor_pos] == map[next_pos] + 1 {
                frontier.push(neighbor_pos);
            }
        }
    }

    trailhead_count
}

pub fn day_10_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::parse_str(args.input, |c| c.to_digit(10).unwrap() as usize).unwrap();
    let trailhead_score_sum: usize = map
        .points()
        .map(|pos| count_trailheads(pos, &map, false))
        .sum();

    Ok(trailhead_score_sum.into())
}

pub fn day_10_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::parse_str(args.input, |c| c.to_digit(10).unwrap() as usize).unwrap();
    let trailhead_score_sum: usize = map
        .points()
        .map(|pos| count_trailheads(pos, &map, true))
        .sum();

    Ok(trailhead_score_sum.into())
}
