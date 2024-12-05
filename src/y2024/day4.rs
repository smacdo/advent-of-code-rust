use std::str::FromStr;

use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::{Grid, Point2};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(4),
    year: Year(2024),
    part_one: SolverPart {
        func: day_4_1,
        examples: &[(
            Answer::Int(18),
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        )],
    },
    part_two: SolverPart {
        func: day_4_2,
        examples: &[(
            Answer::Int(9),
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        )],
    },
};

// TODO: use directions.
pub fn is_word(grid: &Grid<char>, word: &str, x: isize, y: isize, dir: &(isize, isize)) -> bool {
    let mut pos = Point2::new(x, y);

    for i in 0..word.len() {
        if !grid.is_pos_in_bounds(pos) {
            return false;
        }

        if grid[pos] != word.chars().nth(i).unwrap() {
            return false;
        }

        pos += Point2::new(dir.0, dir.1);
    }

    true
}

pub fn day_4_1(input: &str) -> Result<Answer> {
    let grid = Grid::from_str(input).unwrap();
    let mut xmas_count = 0;

    for y in 0..(grid.x_count() as isize) {
        for x in 0..(grid.y_count() as isize) {
            let dirs = &[
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ];

            for d in dirs {
                if is_word(&grid, "XMAS", x, y, d) {
                    xmas_count += 1;
                }
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

pub fn day_4_2(input: &str) -> Result<Answer> {
    let grid = Grid::from_str(input).unwrap();
    let mut xmas_count = 0;

    for y in 0..(grid.y_count() as isize) {
        for x in 0..(grid.x_count() as isize) {
            if grid[Point2::new(x, y)] == 'A'
                && is_mas(&grid, Point2::new(x - 1, y - 1), Point2::new(x + 1, y + 1))
                && is_mas(&grid, Point2::new(x - 1, y + 1), Point2::new(x + 1, y - 1))
            {
                xmas_count += 1
            }
        }
    }

    Ok(xmas_count.into())
}
