use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

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
    assert_eq!(map[trailhead_pos], 0);

    let mut frontier: BinaryHeap<Point2> = BinaryHeap::new();
    frontier.push(trailhead_pos);

    let mut peaks: HashSet<Point2> = HashSet::new();
    let mut trailhead_count: usize = 0;

    while let Some(next_pos) = frontier.pop() {
        if map[next_pos] == 9 && (allow_multiple || !peaks.contains(&next_pos)) {
            peaks.insert(next_pos);
            trailhead_count += 1;
        }

        for neighbor_dir in Direction4::itr() {
            let neighbor_pos = next_pos + neighbor_dir;

            if map.is_pos_in_bounds(neighbor_pos)
                //&& !peaks.contains(&neighbor_pos)
                && map[neighbor_pos] == map[next_pos] + 1
            {
                frontier.push(neighbor_pos);
            }
        }
    }

    trailhead_count
}

pub fn day_10_1(input: &str) -> Result<Answer> {
    let map_chars: Grid<char> = Grid::<_>::from_str(input).unwrap();
    let map: Grid<usize> = Grid::with_values(
        map_chars.x_count(),
        map_chars.y_count(),
        map_chars
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as usize),
    )
    .unwrap();

    let trailhead_score_sum: usize = map
        .points()
        .filter(|pos| map[*pos] == 0)
        .map(|pos| count_trailheads(pos, &map, false))
        .inspect(|score| {
            if *score > 0 {
                //tracing::debug!("score: {score}")
            }
        })
        .sum();
    Ok(trailhead_score_sum.try_into().unwrap())
}

pub fn day_10_2(input: &str) -> Result<Answer> {
    let map_chars: Grid<char> = Grid::<_>::from_str(input).unwrap();
    let map: Grid<usize> = Grid::with_values(
        map_chars.x_count(),
        map_chars.y_count(),
        map_chars
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as usize),
    )
    .unwrap();

    let trailhead_score_sum: usize = map
        .points()
        .filter(|pos| map[*pos] == 0)
        .map(|pos| count_trailheads(pos, &map, true))
        .inspect(|score| {
            if *score > 0 {
                //tracing::debug!("score: {score}")
            }
        })
        .sum();
    Ok(trailhead_score_sum.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reachable_example_1() {
        assert_eq!(
            day_10_1(
                "7770777
7771777
1112111
6543456
7111117
8111118
9111119"
            )
            .unwrap(),
            Answer::Int(2)
        );
    }
}
