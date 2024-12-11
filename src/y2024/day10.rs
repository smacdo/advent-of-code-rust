use std::collections::{BinaryHeap, HashSet};

use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::{Direction4, Grid, Point2};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(10),
    year: Year(2024),
    part_one: SolverPart {
        func: day_10_1,
        examples: &[(
            Answer::Int(36),
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        )],
    },
    part_two: SolverPart {
        func: day_10_2,
        examples: &[(
            Answer::Int(81),
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        )],
    },
};

fn count_trailheads(trailhead_pos: Point2, map: &Grid<usize>, allow_multiple: bool) -> usize {
    let mut frontier: BinaryHeap<Point2> = BinaryHeap::new();

    if map[trailhead_pos] > 0 {
        frontier.push(trailhead_pos);
    }

    let mut peaks: HashSet<Point2> = HashSet::new();
    let mut trailhead_count: usize = 0;

    while let Some(next_pos) = frontier.pop() {
        if map[next_pos] == 9 && (allow_multiple || !peaks.contains(&next_pos)) {
            peaks.insert(next_pos);
            trailhead_count += 1;
        }

        for neighbor_dir in Direction4::itr() {
            let neighbor_pos = next_pos + neighbor_dir;

            if map.is_pos_in_bounds(neighbor_pos) && map[neighbor_pos] == map[next_pos] + 1 {
                frontier.push(neighbor_pos);
            }
        }
    }

    trailhead_count
}

pub fn day_10_1(input: &str) -> Result<Answer> {
    let map = Grid::parse_str(input, |c| c.to_digit(10).unwrap() as usize).unwrap();
    let trailhead_score_sum: usize = map
        .points()
        .map(|pos| count_trailheads(pos, &map, false))
        .sum();

    Ok(trailhead_score_sum.try_into().unwrap())
}

pub fn day_10_2(input: &str) -> Result<Answer> {
    let map = Grid::parse_str(input, |c| c.to_digit(10).unwrap() as usize).unwrap();
    let trailhead_score_sum: usize = map
        .points()
        .map(|pos| count_trailheads(pos, &map, true))
        .sum();

    Ok(trailhead_score_sum.try_into().unwrap())
}
