use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_11_1,
        examples: &[yt::Example {
            input: "aaa: you hhh\n you: bbb ccc\n bbb: ddd eee\n ccc: ddd eee fff\n ddd: ggg\n eee: out\n fff: out\n ggg: out\n hhh: ccc fff iii\n iii: out",
            expected: aoc::Answer::Int(0),
        }],
    },
    part_two: yt::SolverPart {
        func: day_11_2,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

#[derive(Debug, PartialEq)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

fn parse_device_outputs(input: &str) -> Vec<Device> {
    input
        .lines()
        .map(|line| {
            let (name_text, outputs_text) = line.split_once(':').unwrap();
            Device {
                name: name_text.trim().to_string(),
                outputs: outputs_text
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn day_11_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

pub fn day_11_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        assert_eq!(
            parse_device_outputs("you: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff"),
            vec![
                Device {
                    name: "you".to_string(),
                    outputs: vec!["bbb".to_string(), "ccc".to_string()]
                },
                Device {
                    name: "bbb".to_string(),
                    outputs: vec!["ddd".to_string(), "eee".to_string()]
                },
                Device {
                    name: "ccc".to_string(),
                    outputs: vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()]
                },
            ]
        );
    }
}
