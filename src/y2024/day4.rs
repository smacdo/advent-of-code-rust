use std::str::FromStr;

use advent_of_code_data as aoc;
use advent_of_code_rust::spatial::{Direction8, Grid, Point2};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_4_1,
        examples: &[yt::Example {
            input: "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
            expected: aoc::Answer::Int(18),
        }],
    },
    part_two: yt::SolverPart {
        func: day_4_2,
        examples: &[yt::Example {
            input: ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
            expected: aoc::Answer::Int(9),
        }],
    },
};

pub fn is_word(grid: &Grid<char>, word: &str, pos: Point2, offset: Point2) -> bool {
    let mut pos = pos;

    for i in 0..word.len() {
        if !grid.is_pos_in_bounds(pos) {
            return false;
        }

        if grid[pos] != word.chars().nth(i).unwrap() {
            return false;
        }

        pos += offset;
    }

    true
}

pub fn day_4_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let grid = Grid::from_str(args.input).unwrap();
    let mut xmas_count = 0;

    for pos in grid.points() {
        for d in Direction8::all() {
            if is_word(&grid, "XMAS", pos, d.into()) {
                xmas_count += 1;
            }
        }
    }

    Ok(xmas_count.into())
}

fn is_mas(grid: &Grid<char>, a: Point2, b: Point2) -> bool {
    grid.is_pos_in_bounds(a)
        && grid.is_pos_in_bounds(b)
        && ((grid[a] == 'M' && grid[b] == 'S') || (grid[b] == 'M' && grid[a] == 'S'))
}

pub fn day_4_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let grid = Grid::from_str(args.input).unwrap();
    let mut xmas_count = 0;

    for p in grid.points() {
        if grid[p] == 'A'
            && is_mas(&grid, p - Point2::one(), p + Point2::one())
            && is_mas(&grid, p + Point2::new(-1, 1), p + Point2::new(1, -1))
        {
            xmas_count += 1
        }
    }

    Ok(xmas_count.into())
}
