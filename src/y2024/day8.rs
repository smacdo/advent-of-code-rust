use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use advent_of_code_data::registry::{Result, Solver, SolverError, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::{Grid, Point2};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(8),
    year: Year(2024),
    part_one: SolverPart {
        func: day_8_1,
        examples: &[(
            Answer::Int(14),
            "............
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
        )],
    },
    part_two: SolverPart {
        func: day_8_2,
        examples: &[
            (
                Answer::Int(9),
                "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
            ),
            (
                Answer::Int(34),
                "............
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
            ),
        ],
    },
};

// TODO: convert to an iterator?
fn pairwise_combinations<T>(items: &[T]) -> Vec<(T, T)>
where
    T: Clone,
{
    assert!(items.len() > 1);
    let mut combinations: Vec<(T, T)> = Vec::new();

    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            combinations.push((items[i].clone(), items[j].clone()));
        }
    }

    combinations
}

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

pub fn day_8_1(input: &str) -> Result<Answer> {
    let map = Grid::<char>::from_str(input).unwrap();
    let mut antennas: HashMap<char, Vec<Point2>> = HashMap::new();
    let mut antinodes: HashSet<Point2> = HashSet::new();

    for pos in map.points() {
        let c = map[pos];

        if c.is_ascii_alphanumeric() {
            antennas.entry(c).or_default().push(pos);
        }
    }

    for (_frequency, positions) in &antennas {
        for (pos_a, pos_b) in pairwise_combinations(positions) {
            // Calculate the manhattan distance between the two points.
            let distance = pos_b - pos_a;

            // The antinodes are located in line at twice the distance, so apply
            // the manhattan distance to each node.
            let antinode_a = pos_a - distance;
            let antinode_b = pos_b + distance;

            antinodes.insert(antinode_a);
            antinodes.insert(antinode_b);
        }
    }

    Ok(antinodes
        .into_iter()
        .filter(|n| map.is_pos_in_bounds(*n))
        .count()
        .try_into()
        .unwrap())
}

pub fn day_8_2(input: &str) -> Result<Answer> {
    let map = Grid::<char>::from_str(input).unwrap();
    let mut antennas: HashMap<char, Vec<Point2>> = HashMap::new();
    let mut antinodes: HashSet<Point2> = HashSet::new();

    for pos in map.points() {
        let c = map[pos];

        if c.is_ascii_alphanumeric() {
            antennas.entry(c).or_default().push(pos);
        }
    }

    for (_frequency, positions) in &antennas {
        for (pos_a, pos_b) in pairwise_combinations(positions) {
            // Calculate the manhattan distance between the two points.
            let distance = pos_b - pos_a;

            // The antinodes are located at every position in line with the
            // mahattan distance between the two points. Record each point until
            // the lines is off the map.
            let mut antinode_a = pos_a - distance;

            while map.is_pos_in_bounds(antinode_a) {
                antinodes.insert(antinode_a);
                antinode_a -= distance;
            }

            let mut antinode_b = pos_b + distance;

            while map.is_pos_in_bounds(antinode_b) {
                antinodes.insert(antinode_b);
                antinode_b += distance;
            }

            // Record the original antenna locations too!
            antinodes.insert(pos_a);
            antinodes.insert(pos_b);
        }
    }

    visualize(&map, &antinodes);

    Ok(antinodes
        .into_iter()
        .filter(|n| map.is_pos_in_bounds(*n))
        .count()
        .try_into()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let values = &[10, 20, 30, 40];

        assert_eq!(
            pairwise_combinations(values),
            vec![(10, 20), (10, 30), (10, 40), (20, 30), (20, 40), (30, 40)]
        );
    }
}
