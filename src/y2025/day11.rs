use std::collections::HashMap;

use advent_of_code_data as aoc;
use ube::graph::{bfs, Graph, GraphBuilder, NodeBuilder, NodeKey};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

const EXAMPLE_INPUT_1: &str = "aaa: you hhh\n you: bbb ccc\n bbb: ddd eee\n ccc: ddd eee fff\n ddd: ggg\n eee: out\n fff: out\n ggg: out\n hhh: ccc fff iii\n iii: out";
const EXAMPLE_INPUT_2: &str = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out";

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_11_1,
        examples: &[yt::Example {
            input: EXAMPLE_INPUT_1,
            expected: aoc::Answer::Int(0),
        }],
    },
    part_two: yt::SolverPart {
        func: day_11_2,
        examples: &[yt::Example {
            input: EXAMPLE_INPUT_2,
            expected: aoc::Answer::Int(2),
        }],
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

struct NodeKeys {
    target: NodeKey,
    dac: NodeKey,
    fft: NodeKey,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct CacheKey {
    node: NodeKey,
    dac: bool,
    fft: bool,
}

fn count_paths_recursive(
    cache: &mut HashMap<CacheKey, usize>,
    device_graph: &Graph,
    node: NodeKey,
    keys: &NodeKeys,
    dac: bool,
    fft: bool,
) -> usize {
    let mut cache_key = CacheKey { node, dac, fft };

    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    if node == keys.target {
        return if dac && fft { 1 } else { 0 };
    }

    if node == keys.dac {
        cache_key.dac = true;
    } else if node == keys.fft {
        cache_key.fft = true;
    }

    let count = device_graph
        .node(node)
        .edges()
        .map(|w| count_paths_recursive(cache, device_graph, w, keys, cache_key.dac, cache_key.fft))
        .sum();

    cache.insert(cache_key, count);
    count
}

pub fn day_11_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let (device_graph, name_to_node) = parse_device_outputs(args.input);

    let goal_nk = name_to_node["out"];
    let mut goal_count: usize = 0;

    bfs(&device_graph, name_to_node["you"], |g, nk, mut to_visit| {
        if nk == goal_nk {
            goal_count += 1;
        } else {
            for w_nk in g.node(nk).edges() {
                to_visit.add(w_nk);
            }
        }

        true
    });

    Ok(goal_count.into())
}

pub fn day_11_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let (device_graph, name_to_node) = parse_device_outputs(args.input);
    let mut cache: HashMap<CacheKey, usize> = Default::default();

    Ok(count_paths_recursive(
        &mut cache,
        &device_graph,
        name_to_node["svr"],
        &NodeKeys {
            target: name_to_node["out"],
            dac: name_to_node["dac"],
            fft: name_to_node["fft"],
        },
        false,
        false,
    )
    .into())
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
}
