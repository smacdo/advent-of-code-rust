use std::str::FromStr;

use advent_of_code_data as aoc;
use ube::spatial::{Grid, Point2};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;
const START_CHAR: char = 'S';
const SPLITTER_CHAR: char = '^';
const EMPTY_CHAR: char = '.';
const BEAM_CHAR: char = '|';

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_7_1,
        examples: &[yt::Example {
            input: ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............",
            expected: aoc::Answer::Int(21),
        }],
    },
    part_two: yt::SolverPart {
        func: day_7_2,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

pub fn find_start(grid: &Grid<char>) -> Option<Point2> {
    grid.points().find(|&pt| grid[pt] == START_CHAR)
}

pub fn update_beam(grid: &mut Grid<char>, from: Point2) -> usize {
    let mut pos = from;
    let mut splitters_count = 0;

    while grid.is_pos_in_bounds(pos) {
        match grid[pos] {
            EMPTY_CHAR => {
                grid[pos] = BEAM_CHAR;
            }
            SPLITTER_CHAR => {
                let pts = [Point2::WEST, Point2::EAST];
                let mut did_split = false;

                for pt in pts {
                    let pos = pt + pos;
                    if grid.is_pos_in_bounds(pos) && grid[pos] == EMPTY_CHAR {
                        splitters_count += update_beam(grid, pos);
                        did_split = true;
                    }
                }

                if did_split {
                    splitters_count += 1
                }

                break;
            }
            START_CHAR => {
                // ignore!
            }
            BEAM_CHAR => {
                // beam already traveled this path - no need to re-traverse!
                break;
            }
            c => {
                panic!("unexpected character {} at {}", c, pos);
            }
        }

        pos += Point2::SOUTH;
    }

    splitters_count
}

pub fn day_7_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut grid: Grid<char> = Grid::from_str(args.input).unwrap();
    let start = find_start(&grid).unwrap();

    Ok(update_beam(&mut grid, start).into())
}

pub fn day_7_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mini_split_correct() {
        let input = "...S...\n.......\n...^...\n.......\n..^.^..";
        let expected = "...S...\n...|...\n..|^|..\n..|.|..\n.|^|^|.";

        let mut grid: Grid<char> = Grid::from_str(input).unwrap();
        let start = find_start(&grid).unwrap();

        assert_eq!(update_beam(&mut grid, start), 3);

        let actual = format!("{}", grid);
        assert_eq!(actual.trim(), expected);
    }
}
