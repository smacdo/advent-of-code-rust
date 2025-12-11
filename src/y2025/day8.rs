use advent_of_code_data as aoc;
use ube::{spatial::Point3, union_find::UnionFind, utils::pairwise_combinations};
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

const EXAMPLE_INPUT: &str =  "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_8_1,
        examples: &[yt::Example {
            input: EXAMPLE_INPUT,
            expected: aoc::Answer::Int(40),
        }],
    },
    part_two: yt::SolverPart {
        func: day_8_2,
        examples: &[yt::Example {
            input: EXAMPLE_INPUT,
            expected: aoc::Answer::Int(25272),
        }],
    },
};

fn parse_junction_box_positions(input: &str) -> Vec<Point3> {
    input
        .lines()
        .map(|line| {
            let mut splitter = line.splitn(3, ',');
            let x = splitter.next().unwrap().parse().unwrap();
            let y = splitter.next().unwrap().parse().unwrap();
            let z = splitter.next().unwrap().parse().unwrap();

            Point3 { x, y, z }
        })
        .collect::<Vec<_>>()
}

fn find_pairs_closest(positions: &[Point3]) -> Vec<(f64, Point3, Point3)> {
    let mut pairs = pairwise_combinations(positions)
        .map(|(a, b)| (Point3::distance(a, b), *a, *b))
        .collect::<Vec<_>>();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    pairs
}

pub fn day_8_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let positions = parse_junction_box_positions(args.input);
    let pairs = find_pairs_closest(&positions);
    let mut uf: UnionFind<Point3> = UnionFind::from_iter(positions);

    // XXX(scott): The example for this problem uses 10 iterations rather than the puzzle's full
    // 1000.
    let iteration_count = if args.input == EXAMPLE_INPUT {
        10
    } else {
        1000
    };

    for (_dist, a, b) in pairs.into_iter().take(iteration_count) {
        uf.union(&a, &b);
    }

    Ok(uf
        .sets()
        .iter()
        .take(3)
        .map(|(_set, size)| size)
        .product::<usize>()
        .into())
}

pub fn day_8_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let positions = parse_junction_box_positions(args.input);
    let pairs = find_pairs_closest(&positions);
    let mut uf: UnionFind<Point3> = UnionFind::from_iter(positions);

    let mut last = 0;

    for (_dist, a, b) in pairs.into_iter() {
        if uf.union(&a, &b).is_some() {
            last = a.x * b.x;
        }
    }

    Ok(last.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_closest_pairs_using_example() {
        let positions = parse_junction_box_positions(EXAMPLE_INPUT);
        let pairs = find_pairs_closest(&positions)
            .into_iter()
            .map(|(_dist, a, b)| (a, b))
            .collect::<Vec<_>>();

        assert_eq!(
            pairs[0],
            (
                Point3 {
                    x: 162,
                    y: 817,
                    z: 812
                },
                Point3 {
                    x: 425,
                    y: 690,
                    z: 689
                }
            )
        );

        assert_eq!(
            pairs[1],
            (
                Point3 {
                    x: 162,
                    y: 817,
                    z: 812
                },
                Point3 {
                    x: 431,
                    y: 825,
                    z: 988
                }
            )
        );

        assert_eq!(
            pairs[2],
            (
                Point3 {
                    x: 906,
                    y: 360,
                    z: 560
                },
                Point3 {
                    x: 805,
                    y: 96,
                    z: 715
                }
            )
        );

        assert_eq!(
            pairs[3],
            (
                Point3 {
                    x: 431,
                    y: 825,
                    z: 988
                },
                Point3 {
                    x: 425,
                    y: 690,
                    z: 689
                }
            )
        );
    }
}
