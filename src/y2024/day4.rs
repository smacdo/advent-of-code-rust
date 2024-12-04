use advent_of_code_data::registry::{Result, Solver, SolverError, SolverPart};
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
        examples: &[],
    },
};

pub fn clear_visited(visited: &mut Vec<Vec<bool>>) {
    for y in 0..visited.len() {
        for x in 0..visited[y].len() {
            visited[y][x] = false;
        }
    }
}

pub fn is_word(grid: &Vec<Vec<char>>, word: &str, x: usize, y: usize, dir: &(i32, i32)) -> bool {
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

/*
pub fn find_word(
    word: &str,   // XMAS
    index: usize, // 0
    x: usize,     // 0
    y: usize,     // 1
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    mut current_count: usize,
) -> usize {
    assert!(!word.is_empty());
    assert!(index < word.len());
    assert!(y < grid.len() && x < grid[y].len());

    let next_c = word.chars().nth(index).unwrap();

    if next_c == grid[y][x] {
        if index == word.len() - 1 {
            return current_count + 1;
        }
    } else {
        return current_count;
    }

    let directions = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for dir_offset in directions {
        let new_x = (x as i32) + dir_offset.0;
        let new_y = (y as i32) + dir_offset.1;

        // reject out of bounds
        if new_x < 0
            || new_y < 0
            || new_y as usize >= grid.len()
            || new_x as usize >= grid[new_y as usize].len()
        {
            continue;
        }

        // reject if visited
        if visited[new_y as usize][new_x as usize] {
            continue;
        }

        // visit
        visited[new_y as usize][new_x as usize] = true;
        current_count = find_word(
            word,
            index + 1,
            new_x as usize,
            new_y as usize,
            grid,
            visited,
            current_count,
        );
    }

    current_count
}
    */

pub fn day_4_1(input: &str) -> Result<Answer> {
    // Build row major grid.
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
        visited.push((0..line.len()).into_iter().map(|_| false).collect());
    }

    // Search each tile to see if it is a valid XMAS.
    let mut xmas_count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            /*
            clear_visited(&mut visited);
            let count = find_word("XMAS", 0, x, y, &grid, &mut visited, 0);

            if (count > 0) {
                println!("FOUND at {x}, {y}. New count is {}", count + xmas_count)
            }
            */

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

            //xmas_count += count;
        }
    }

    Ok(xmas_count.try_into().unwrap())
}

pub fn day_4_2(input: &str) -> Result<Answer> {
    Err(SolverError::NotFinished)
}
