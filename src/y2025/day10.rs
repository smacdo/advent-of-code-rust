use advent_of_code_data as aoc;
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

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

/// `[...]`
fn parse_lights(input: &str) -> Vec<bool> {
    let input = input.trim();

    input[1..(input.len() - 1)]
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            c => panic!("unrecogonized char {c}"),
        })
        .collect::<Vec<_>>()
}

fn parse_manual(input: &str) -> Vec<Machine> {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    for line in input.lines() {
        let (lights_text, rest) = line.split_at(line.find('(').unwrap());
        let (buttons_text, joltage_text) = rest.split_at(rest.find('{').unwrap());
    }
    todo!()
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
    fn parse_lights_str() {
        assert_eq!(parse_lights("[.##.] "), vec![false, true, true, false]);
    }
}
