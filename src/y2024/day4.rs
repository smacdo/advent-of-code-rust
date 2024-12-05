use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
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

pub fn is_word(grid: &[Vec<char>], word: &str, x: usize, y: usize, dir: &(i32, i32)) -> bool {
    let mut x = x as i32;
    let mut y = y as i32;

    for i in 0..word.len() {
        if x < 0 || y < 0 || y as usize >= grid.len() || x as usize >= grid[y as usize].len() {
            return false;
        }

        if grid[y as usize][x as usize] != word.chars().nth(i).unwrap() {
            return false;
        }

        x += dir.0;
        y += dir.1;
    }

    true
}

pub fn day_4_1(input: &str) -> Result<Answer> {
    // Build row major grid.
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
        visited.push((0..line.len()).map(|_| false).collect());
    }

    // Search each tile to see if it is a valid XMAS.
    let mut xmas_count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
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

fn is_mas(x1: i32, y1: i32, x2: i32, y2: i32, grid: &[Vec<char>]) -> bool {
    if x1 < 0
        || x2 < 0
        || y1 < 0
        || y2 < 0
        || y1 as usize >= grid.len()
        || y2 as usize >= grid.len()
        || x1 as usize >= grid[y1 as usize].len()
        || x2 as usize >= grid[y2 as usize].len()
    {
        false
    } else {
        let y1 = y1 as usize;
        let y2 = y2 as usize;
        let x1 = x1 as usize;
        let x2 = x2 as usize;

        (grid[y1][x1] == 'M' && grid[y2][x2] == 'S') || (grid[y2][x2] == 'M' && grid[y1][x1] == 'S')
    }
}

pub fn day_4_2(input: &str) -> Result<Answer> {
    // Build row major grid.
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
        visited.push((0..line.len()).map(|_| false).collect());
    }

    let mut xmas_count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let dx = x as i32;
            let dy = y as i32;

            if grid[y][x] == 'A'
                && is_mas(dx - 1, dy - 1, dx + 1, dy + 1, &grid)
                && is_mas(dx - 1, dy + 1, dx + 1, dy - 1, &grid)
            {
                xmas_count += 1
            }
        }
    }

    Ok(xmas_count.into())
}
