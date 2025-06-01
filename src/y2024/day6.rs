use std::collections::HashSet;
use std::str::FromStr;

use advent_of_code_data as aoc;
use yuletide as yt;

use advent_of_code_rust::spatial::{Direction4, Grid, Point2};
use linkme::distributed_slice;
use thiserror::Error;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::Solver = yt::Solver {
    day: aoc::Day(6),
    year: aoc::Year(2024),
    part_one: yt::SolverPart {
        func: day_6_1,
        examples: &[yt::Example {
            input: "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
            expected: aoc::Answer::Int(41),
        }],
    },
    part_two: yt::SolverPart {
        func: day_6_2,
        examples: &[yt::Example {
            input: "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
            expected: aoc::Answer::Int(6),
        }],
    },
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Guard {
    pub pos: Point2,
    pub dir: Direction4,
}

impl Guard {
    pub fn new(pos: Point2, dir: Direction4) -> Self {
        Self { pos, dir }
    }
}

fn find_guard(map: &Grid<char>) -> Option<Guard> {
    for pos in map.points() {
        match map[pos] {
            '>' => return Some(Guard::new(pos, Direction4::East)),
            '^' => return Some(Guard::new(pos, Direction4::North)),
            '<' => return Some(Guard::new(pos, Direction4::West)),
            'v' => return Some(Guard::new(pos, Direction4::South)),
            _ => {}
        };
    }

    None
}

fn visualize(map: &Grid<char>, path: &[Point2]) {
    for row in map.rows() {
        for pos in row {
            let c = match map[pos] {
                '.' if path.contains(&pos) => 'X',
                c => c,
            };

            print!("{}", c);
        }

        println!();
    }
}

#[derive(Debug, Error)]
#[error("There was an infinite loop detected in the guards path")]
struct PathLoopError;

// NOTE: Path is not temporally ordered, eg the first entry is not always the
//       guard's starting position.
fn find_guard_path(map: &Grid<char>) -> std::result::Result<Vec<Point2>, PathLoopError> {
    Ok(simulate_guard_walk(map, true)?.unwrap())
}

fn does_guard_walk_loop(map: &Grid<char>) -> bool {
    simulate_guard_walk(map, false).is_err()
}

fn simulate_guard_walk(
    map: &Grid<char>,
    needs_path: bool,
) -> std::result::Result<Option<Vec<Point2>>, PathLoopError> {
    let mut guard = find_guard(map).unwrap();
    let mut tiles_visited: HashSet<Guard> = HashSet::with_capacity(10000);

    while map.is_pos_in_bounds(guard.pos) {
        // Mark each tile that was visited (along with direction) to find any
        // infinite loops.
        if tiles_visited.contains(&guard) {
            return Err(PathLoopError);
        }

        tiles_visited.insert(guard.clone());

        // Walk the guard one tile forward from their current heading. If the
        // tile is an obstruction then the guard should turn 90 degrees to the
        // right.
        let next_pos = guard.pos + guard.dir;

        if map.is_pos_in_bounds(next_pos) && map[next_pos] == '#' {
            guard.dir = guard.dir.rotated_90_cw();
        } else {
            guard.pos = next_pos;
        }
    }

    // Return with the number of uniquely visited tiles (if requested).
    if needs_path {
        let mut tiles_visited: Vec<Point2> = tiles_visited.iter().map(|t| t.pos).collect();

        tiles_visited.sort();
        tiles_visited.dedup();

        Ok(tiles_visited.into())
    } else {
        Ok(None)
    }
}

pub fn day_6_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::<char>::from_str(args.input).unwrap();
    let path = find_guard_path(&map).unwrap();

    visualize(&map, &path);

    Ok(path.len().into())
}

pub fn day_6_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::<char>::from_str(args.input).unwrap();
    let guard = find_guard(&map).unwrap();

    // Try every possible variation of adding one obstruction to the map, and
    // check how many maps have a loop in them when walked.
    let mut number_of_loop_paths = 0;

    for row in map.rows() {
        for pos in row {
            if map[pos] == '#' || pos == guard.pos {
                // obstruction already here, and part 1 proves this input doesn't
                // have a loop.
                continue;
            }

            let mut map = map.clone();
            map[pos] = '#';

            if does_guard_walk_loop(&map) {
                number_of_loop_paths += 1;
            }
        }
    }

    Ok(number_of_loop_paths.into())
}
