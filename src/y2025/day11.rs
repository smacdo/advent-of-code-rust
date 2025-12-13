use advent_of_code_data as aoc;
use ube::graph::{is_acyclic, Graph};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

const EXAMPLE_INPUT: &str = "aaa: you hhh\n you: bbb ccc\n bbb: ddd eee\n ccc: ddd eee fff\n ddd: ggg\n eee: out\n fff: out\n ggg: out\n hhh: ccc fff iii\n iii: out";

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_11_1,
        examples: &[yt::Example {
            input: EXAMPLE_INPUT,
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

fn parse_device_outputs(input: &str) -> Graph {
    let elements = input
        .lines()
        .map(|line| {
            let (name_text, outputs_text) = line.split_once(':').unwrap();
            (
                name_text.trim(),
                outputs_text.split_whitespace().collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // TODO: can this be moved into the graph module? maybe as a From<(&'str, Vec<'&str>)>?
    Graph::from_iter(
        elements
            .iter()
            .map(|(name, neighbors)| (*name, neighbors.as_slice())),
    )
}

pub fn day_11_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let device_graph = parse_device_outputs(args.input);
    assert!(is_acyclic(&device_graph));

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
        let mut g = Graph::new();

        g.add_node_with_neighbors("you", &["bbb", "ccc"]);
        g.add_node_with_neighbors("bbb", &["ddd", "eee"]);
        g.add_node_with_neighbors("ccc", &["ddd", "eee", "fff"]);

        assert_eq!(
            parse_device_outputs("you: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff"),
            g
        );
    }

    /*
    #[test]
    fn example_has_no_backflow() {
        let devices_list = DeviceList::new(parse_device_outputs(EXAMPLE_INPUT));
        assert!(!check_back_flow(&devices_list));
    }
    */
}
