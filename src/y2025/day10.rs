use advent_of_code_data as aoc;
use ube::utils::find_ints;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_10_1,
        examples: &[yt::Example {
            input: "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
            expected: aoc::Answer::Int(7),
        }],
    },
    part_two: yt::SolverPart {
        func: day_10_2,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

#[derive(Debug, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn parse_manual(input: &str) -> Vec<Machine> {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    input
        .lines()
        .map(|line| {
            let (lights_text, rest) = line.split_at(line.find('(').unwrap());
            let (buttons_text, joltage_text) = rest.split_at(rest.find('{').unwrap());

            let lights = lights_text
                .chars()
                .filter(|c| *c == '.' || *c == '#')
                .map(|c| c == '#')
                .collect::<Vec<_>>();
            let buttons = buttons_text
                .split('(')
                .filter(|s| !s.is_empty())
                .map(|chunk| find_ints::<usize>(chunk).unwrap())
                .collect::<Vec<_>>();
            let joltages = find_ints::<usize>(joltage_text).unwrap();

            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect::<Vec<_>>()
}

pub fn day_10_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

pub fn day_10_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_examples() {
        assert_eq!(
            parse_manual("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,13) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            vec![Machine {
                lights: vec![false, true, true, false],
                buttons: vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1]
                ],
                joltages: vec![3, 5, 4, 7]
            },Machine {
                lights: vec![false, false, false, true, false],
                buttons: vec![
                    vec![0, 2, 3, 4],
                    vec![2, 13],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                joltages: vec![7, 5, 12, 7, 2]
            }]

            // [...#.] (0,2,3,4) (2,13) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        );
    }
}
