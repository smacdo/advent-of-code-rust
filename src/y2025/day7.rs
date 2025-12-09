use std::{collections::HashMap, str::FromStr};

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
        examples: &[yt::Example {
            input: ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............",
            expected: aoc::Answer::Int(40),
        }],
    },
};

pub fn find_start(grid: &Grid<char>) -> Option<Point2> {
    grid.points().find(|&pt| grid[pt] == START_CHAR)
}

pub fn count_beam_splits(grid: &mut Grid<char>, start: Point2) -> usize {
    let mut pos = start;
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
                        splitters_count += count_beam_splits(grid, pos);
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

pub fn count_timelines(
    grid: &mut Grid<char>,
    start: Point2,
    cache: &mut HashMap<Point2, usize>,
) -> usize {
    let mut pos = start;
    let mut timeline_count = 0;

    while grid.is_pos_in_bounds(pos) {
        match grid[pos] {
            EMPTY_CHAR | BEAM_CHAR => {
                grid[pos] = BEAM_CHAR;

                // Count this as a new timeline each time the beam reaches the bottom of the grid.
                if pos.y as usize == grid.y_count() - 1 {
                    timeline_count += 1;
                }
            }
            SPLITTER_CHAR => {
                let pts = [Point2::WEST, Point2::EAST];

                for pt in pts {
                    let pos = pt + pos;
                    if grid.is_pos_in_bounds(pos) && grid[pos] != SPLITTER_CHAR {
                        if !cache.contains_key(&pos) {
                            let c = count_timelines(grid, pos, cache);
                            cache.insert(pos, c);
                        }

                        timeline_count += cache[&pos];
                    }
                }

                break;
            }
            START_CHAR => {
                // ignore!
            }
            c => {
                panic!("unexpected character {} at {}", c, pos);
            }
        }

        pos += Point2::SOUTH;
    }

    timeline_count
}

pub fn day_7_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut grid: Grid<char> = Grid::from_str(args.input).unwrap();
    let start = find_start(&grid).unwrap();

    Ok(count_beam_splits(&mut grid, start).into())
}

pub fn day_7_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut grid: Grid<char> = Grid::from_str(args.input).unwrap();
    let start = find_start(&grid).unwrap();

    let mut cache: HashMap<Point2, usize> = HashMap::new();

    Ok(count_timelines(&mut grid, start, &mut cache).into())
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

        assert_eq!(count_beam_splits(&mut grid, start), 3);

        let actual = format!("{}", grid);
        assert_eq!(actual.trim(), expected);
    }
}
