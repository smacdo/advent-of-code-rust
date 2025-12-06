use std::str::FromStr;

use advent_of_code_data as aoc;
use ube::spatial::{Direction8, Grid, Point2};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_4_1,
        examples: &[yt::Example {
            input: "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.",
            expected: aoc::Answer::Int(13),
        }],
    },
    part_two: yt::SolverPart {
        func: day_4_2,
        examples: &[yt::Example {
            input: "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.",
            expected: aoc::Answer::Int(43),
        }],
    },
};

fn neighbor_count(grid: &Grid<char>, pt: Point2) -> usize {
    Direction8::all()
        .filter(|dir| {
            let n_pos = pt + *dir;
            grid.is_pos_in_bounds(n_pos) && grid[n_pos] == '@'
        })
        .count()
}

pub fn day_4_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::<char>::from_str(args.input).unwrap();

    Ok(map
        .points()
        .filter(|pt| map[*pt] == '@' && neighbor_count(&map, *pt) < 4)
        .count()
        .into())
}

pub fn day_4_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut map = Grid::<char>::from_str(args.input).unwrap();
    let mut sum = 0;

    loop {
        let removable = map
            .points()
            .filter(|pt| map[*pt] == '@' && neighbor_count(&map, *pt) < 4)
            .collect::<Vec<_>>();

        if removable.is_empty() {
            break;
        }

        sum += removable.len();

        for pt in removable {
            map[pt] = '.';
        }
    }

    Ok(sum.into())
}
