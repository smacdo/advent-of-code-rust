use std::collections::HashMap;

use advent_of_code_data as aoc;
use yuletide as yt;

use advent_of_code_rust::spatial::Point2;
use linkme::distributed_slice;
use regex::Regex;

// TODO: Consider using Grid rather than Hashmap

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverRegistration = yt::SolverRegistration {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_14_1,
        examples: &[yt::Example {
            input: "p=0,4 v=3,-3
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
            expected: aoc::Answer::Int(12),
        }],
    },
    part_two: yt::SolverPart {
        func: day_14_2,
        examples: &[
            // TODO: missing example?
        ],
    },
};

const X_COUNT: usize = 101;
const Y_COUNT: usize = 103;

#[derive(Debug)]
struct Robot {
    pos: Point2,
    vel: Point2, // TODO: IntVec?
}

fn parse_input(input: &str) -> Vec<Robot> {
    let input_line_re =
        Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = input_line_re.captures(line).unwrap();
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

// TODO: turn this into a utility function
// TODO: make this generic number type?
// TODO: make a point wrap utility function.
fn wrap(mut a: isize, k: usize) -> isize {
    assert!(k <= isize::MAX as usize);
    let k = k as isize;

    while a >= k {
        a -= k;
    }
    while a < 0 {
        a += k;
    }

    a
}

fn simulate(robots: &mut [Robot], iterations: usize, x_count: usize, y_count: usize) {
    for robot in robots {
        for _ in 0..iterations {
            robot.pos += robot.vel;

            robot.pos.x = wrap(robot.pos.x, x_count);
            robot.pos.y = wrap(robot.pos.y, y_count);

            assert!(robot.pos.x >= 0 && robot.pos.x < x_count as isize);
            assert!(robot.pos.y >= 0 && robot.pos.y < y_count as isize);
        }
    }
}

fn visualize(tiles: &HashMap<Point2, usize>, x_count: usize, y_count: usize) {
    let mut viz_str = String::new();

    for y in 0..y_count {
        for x in 0..x_count {
            if let Some(count) = tiles.get(&Point2::new(x as isize, y as isize)) {
                viz_str.push_str(&format!("{count}"));
            } else {
                viz_str.push('.');
            }
        }

        viz_str.push('\n');
    }

    tracing::debug!("\n{}", viz_str);
}

pub fn day_14_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut robots = parse_input(args.input);

    let is_example_input = args.input == SOLVER.part_one.examples[0].input;
    let x_count = if is_example_input { 11 } else { X_COUNT };
    let y_count = if is_example_input { 7 } else { Y_COUNT };

    // Calculate where robots will end up after 100 steps.
    simulate(&mut robots, 100, x_count, y_count);

    // Merge robots that are on the same tile into a shared count.
    let mut positions: HashMap<Point2, usize> = HashMap::new();

    for r in robots {
        positions
            .entry(r.pos)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    visualize(&positions, x_count, y_count);

    // Calculate safety factor as per puzzle description.
    let x_middle = (x_count / 2) as isize;
    let y_middle = (y_count / 2) as isize;
    let mut quad = [0, 0, 0, 0];

    for (pos, count) in positions {
        match (pos.x.cmp(&x_middle), pos.y.cmp(&y_middle)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quad[0] += count,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quad[2] += count,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quad[1] += count,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quad[3] += count,
            _ => {}
        }
    }

    let safety_factor: usize = quad.into_iter().product();
    Ok(safety_factor.into())
}

pub fn day_14_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    // Search for the first timestep where all the robots have unique positions.
    let mut robots = parse_input(args.input);
    let mut timestep = 0;
    let mut positions: HashMap<Point2, usize> = HashMap::new();

    loop {
        assert!(timestep < 10000);

        simulate(&mut robots, 1, X_COUNT, Y_COUNT);
        timestep += 1;

        positions.clear();
        let mut all_unique = true;

        for r in &robots {
            if let std::collections::hash_map::Entry::Vacant(e) = positions.entry(r.pos) {
                e.insert(1);
            } else {
                all_unique = false;
                break;
            }
        }

        if all_unique {
            break;
        }
    }

    visualize(&positions, X_COUNT, Y_COUNT);
    Ok(timestep.into())
}
