use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::Point2;
use linkme::distributed_slice;
use regex::Regex;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(14),
    year: Year(2024),
    part_one: SolverPart {
        func: day_14_1,
        examples: &[(
            Answer::Int(12),
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        )],
    },
    part_two: SolverPart {
        func: day_14_2,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
};

const X_COUNT: usize = 101;
const Y_COUNT: usize = 103;

struct Robot {
    pos: Point2,
    vel: Point2, // TODO: IntVec?
}

fn parse_input(input: &str) -> Vec<Robot> {
    let input_line_re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>\d+),(?<yd>\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = input_line_re.captures(input).unwrap();
            Robot {
                pos: Point2::new(
                    caps["px"].parse::<isize>().unwrap(),
                    caps["py"].parse::<isize>().unwrap(),
                ),
                vel: Point2::new(
                    caps["vx"].parse::<isize>().unwrap(),
                    caps["vy"].parse::<isize>().unwrap(),
                ),
            }
        })
        .collect()
}

fn simulate(robots: &mut [Robot], iterations: usize, x_count: usize, y_count: usize) {
    let x_count = x_count as isize;
    let y_count = y_count as isize;

    for robot in robots {
        for _ in 0..iterations {
            robot.pos += robot.vel;

            while robot.pos.x >= x_count {
                robot.pos.x -= x_count;
            }

            while robot.pos.y >= y_count {
                robot.pos.y -= y_count;
            }
        }
    }
}

pub fn day_14_1(input: &str) -> Result<Answer> {
    let mut robots = parse_input(input);
    simulate(&mut robots, 100, X_COUNT, Y_COUNT);

    // divide into quadrants

    Err(advent_of_code_data::registry::SolverError::NotFinished)
}

pub fn day_14_2(_input: &str) -> Result<Answer> {
    Err(advent_of_code_data::registry::SolverError::NotFinished)
}
