use std::collections::{HashMap, VecDeque};

use advent_of_code_data as aoc;
use ube::graph::{self, is_acyclic, Graph, GraphBuilder, NodeBuilder, NodeKey};
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

fn parse_device_outputs(input: &str) -> (Graph, HashMap<String, NodeKey>) {
    let mut g_builder = GraphBuilder::new();

    input.lines().for_each(|line| {
        let (name_text, outputs_text) = line.split_once(':').unwrap();

        let mut n_builder = NodeBuilder::new();
        n_builder.set_name(name_text);

        for neighbor_name in outputs_text.split_whitespace() {
            n_builder.add_edge(neighbor_name);
        }

        g_builder.add_node(n_builder);
    });

    g_builder.build()
}

pub fn count_paths(g: &Graph, start: NodeKey, end: NodeKey) -> usize {
    let mut goal_count = 0;

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    while !to_visit.is_empty() {
        let visit_count = to_visit.len();

        for _ in 0..visit_count {
            let nk = to_visit.pop_front().unwrap();

            if nk == end {
                goal_count += 1;
            } else {
                for w_nk in g.node(nk).edges() {
                    to_visit.push_back(w_nk);
                }
            }
        }
    }

    goal_count
}

pub fn day_11_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let (device_graph, name_to_node) = parse_device_outputs(args.input);
    assert!(is_acyclic(&device_graph));

    Ok(count_paths(&device_graph, name_to_node["you"], name_to_node["out"]).into())
}

pub fn day_11_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let g = GraphBuilder::new()
            .with_node(|n| n.with_name("you").with_edge("bbb").with_edge("ccc"))
            .with_node(|n| n.with_name("bbb").with_edge("ddd").with_edge("eee"))
            .with_node(|n| {
                n.with_name("ccc")
                    .with_edge("ddd")
                    .with_edge("eee")
                    .with_edge("fff")
            })
            .with_node(|n| n.with_name("ddd"))
            .with_node(|n| n.with_name("eee"))
            .with_node(|n| n.with_name("fff"))
            .build();

        assert_eq!(
            parse_device_outputs("you: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff"),
            g
        );
    }

    #[test]
    fn example_has_no_backflow() {
        let (device_graph, _name_to_node) = parse_device_outputs(EXAMPLE_INPUT);
        assert!(is_acyclic(&device_graph));
    }
}
