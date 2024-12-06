use std::str::FromStr;

use advent_of_code_data::registry::{Result, Solver, SolverError, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::{Direction4, Grid, Point2};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(6),
    year: Year(2024),
    part_one: SolverPart {
        func: day_6_1,
        examples: &[(
            Answer::Int(41),
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        )],
    },
    part_two: SolverPart {
        func: day_6_2,
        examples: &[],
    },
};

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
    for y in 0..map.y_count() {
        for x in 0..map.x_count() {
            let pos = Point2::new(x as isize, y as isize);

            match map[pos] {
                '>' => return Some(Guard::new(pos, Direction4::East)),
                '^' => return Some(Guard::new(pos, Direction4::North)),
                '<' => return Some(Guard::new(pos, Direction4::West)),
                'v' => return Some(Guard::new(pos, Direction4::South)),
                _ => {}
            };
        }
    }

    None
}

fn visualize(map: &Grid<char>, walked: &Grid<bool>) {
    for y in 0..(map.y_count() as isize) {
        for x in 0..(map.x_count() as isize) {
            let pos = Point2::new(x, y);

            let c = match map[pos] {
                '.' if walked[pos] => 'X',
                c => c,
            };

            print!("{}", c);
        }

        println!();
    }
}

pub fn day_6_1(input: &str) -> Result<Answer> {
    let map = Grid::<char>::from_str(input).unwrap();
    let mut walked: Grid<bool> = Grid::new(map.x_count(), map.y_count(), false);
    let mut guard = find_guard(&map).unwrap();

    let mut break_glass_counter = 10000;

    while map.is_pos_in_bounds(guard.pos) {
        // Mark the current space as "walked".
        walked[guard.pos] = true;

        // Walk the guard one tile forward from their current heading. If the
        // tile is an obstruction then the guard should turn 90 degrees to the
        // right.
        let next_pos = guard.pos + guard.dir;

        if map.is_pos_in_bounds(next_pos) && map[next_pos] == '#' {
            guard.dir = guard.dir.rotated_90_cw();
        } else {
            guard.pos = next_pos;
        }

        if break_glass_counter == 0 {
            panic!("BREAK GLASS");
        } else {
            break_glass_counter -= 1;
        }
    }

    visualize(&map, &walked);

    // Count the number of tiles that the guard walked.
    let tiles_walked = walked.into_iter().filter(|x| *x).count();
    Ok(tiles_walked.try_into().unwrap())
}

pub fn day_6_2(_input: &str) -> Result<Answer> {
    Err(SolverError::NotFinished)
}
