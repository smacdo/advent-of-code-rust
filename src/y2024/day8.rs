use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;
use ube::spatial::{Grid, Point2};
use ube::utils::pairwise_combinations;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_8_1,
        examples: &[yt::Example {
            input: "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
            expected: aoc::Answer::Int(14),
        }],
    },
    part_two: yt::SolverPart {
        func: day_8_2,
        examples: &[
            yt::Example {
                input: "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
                expected: aoc::Answer::Int(9),
            },
            yt::Example {
                input: "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
                expected: aoc::Answer::Int(34),
            },
        ],
    },
};

#[allow(dead_code)]
fn visualize(map: &Grid<char>, antinodes: &HashSet<Point2>) {
    for row in map.rows() {
        for pos in row {
            if antinodes.contains(&pos) {
                print!("#");
            } else {
                print!("{}", map[pos]);
            }
        }
        println!();
    }
}

pub fn day_8_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::<char>::from_str(args.input).unwrap();
    let mut antennas: HashMap<char, Vec<Point2>> = HashMap::new();
    let mut antinodes: HashSet<Point2> = HashSet::new();

    for pos in map.points() {
        let c = map[pos];

        if c.is_ascii_alphanumeric() {
            antennas.entry(c).or_default().push(pos);
        }
    }

    for positions in antennas.values() {
        for (pos_a, pos_b) in pairwise_combinations(positions) {
            // Calculate the manhattan distance between the two points.
            let distance = *pos_b - *pos_a;

            // The antinodes are located in line at twice the distance, so apply
            // the manhattan distance to each node.
            let antinode_a = *pos_a - distance;
            let antinode_b = *pos_b + distance;

            antinodes.insert(antinode_a);
            antinodes.insert(antinode_b);
        }
    }

    Ok(antinodes
        .into_iter()
        .filter(|n| map.is_pos_in_bounds(*n))
        .count()
        .into())
}

pub fn day_8_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let map = Grid::<char>::from_str(args.input).unwrap();
    let mut antennas: HashMap<char, Vec<Point2>> = HashMap::new();
    let mut antinodes: HashSet<Point2> = HashSet::new();

    for pos in map.points() {
        let c = map[pos];

        if c.is_ascii_alphanumeric() {
            antennas.entry(c).or_default().push(pos);
        }
    }

    for positions in antennas.values() {
        for (pos_a, pos_b) in pairwise_combinations(positions) {
            // Calculate the manhattan distance between the two points.
            let distance = *pos_b - *pos_a;

            // The antinodes are located at every position in line with the
            // mahattan distance between the two points. Record each point until
            // the lines is off the map.
            let mut antinode_a = *pos_a - distance;

            while map.is_pos_in_bounds(antinode_a) {
                antinodes.insert(antinode_a);
                antinode_a -= distance;
            }

            let mut antinode_b = *pos_b + distance;

            while map.is_pos_in_bounds(antinode_b) {
                antinodes.insert(antinode_b);
                antinode_b += distance;
            }

            // Record the original antenna locations too!
            antinodes.insert(*pos_a);
            antinodes.insert(*pos_b);
        }
    }

    Ok(antinodes
        .into_iter()
        .filter(|n| map.is_pos_in_bounds(*n))
        .count()
        .into())
}
