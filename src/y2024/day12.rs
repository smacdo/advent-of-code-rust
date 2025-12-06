use std::collections::VecDeque;

use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;
use ube::spatial::{Direction4, Direction8, Grid, Point2};

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_12_1,
        examples: &[yt::Example {
            input: "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
            expected: aoc::Answer::Int(1930),
        }],
    },
    part_two: yt::SolverPart {
        func: day_12_2,
        examples: &[yt::Example {
            input: "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
            expected: aoc::Answer::Int(1206),
        }],
    },
};

#[derive(Debug, Clone)]
struct Tile {
    pub c: char,
    pub visited: bool,
}

struct Region {
    pub area: usize,
    pub edges: usize,
    pub corners: usize,
}

fn edges(pt: Point2, map: &Grid<Tile>) -> usize {
    assert!(map.is_pos_in_bounds(pt));

    let mut differing_edge_count = 0;

    for dir in Direction4::all() {
        let neighbor_pt = pt + dir;
        if !map.is_pos_in_bounds(neighbor_pt) || map[pt].c != map[neighbor_pt].c {
            differing_edge_count += 1;
        }
    }

    differing_edge_count
}

fn corners(pt: Point2, map: &Grid<Tile>) -> usize {
    assert!(map.is_pos_in_bounds(pt));

    let mut differing_edge_count = 0;

    // First two values in tuple are used to check an "external" corner and the
    // third tuple value is used to check for an "internal" corner.
    //
    // External:        Internal:
    //   X A              A B
    //   A B              B B
    for (dir_a, dir_b, dir_c) in &[
        (Direction4::East, Direction4::North, Direction8::Northeast),
        (Direction4::West, Direction4::North, Direction8::Northwest),
        (Direction4::West, Direction4::South, Direction8::Southwest),
        (Direction4::East, Direction4::South, Direction8::Southeast),
    ] {
        let neighbor_pt_a = pt + *dir_a;
        let neighbor_pt_b = pt + *dir_b;
        let neighbor_pt_c = pt + *dir_c;

        let differs_a = !map.is_pos_in_bounds(neighbor_pt_a) || map[pt].c != map[neighbor_pt_a].c;
        let differs_b = !map.is_pos_in_bounds(neighbor_pt_b) || map[pt].c != map[neighbor_pt_b].c;
        let internal_corner_differ = map.is_pos_in_bounds(neighbor_pt_a)
            && map.is_pos_in_bounds(neighbor_pt_b)
            && map.is_pos_in_bounds(neighbor_pt_c)
            && map[pt].c == map[neighbor_pt_a].c
            && map[pt].c == map[neighbor_pt_b].c
            && map[pt].c != map[neighbor_pt_c].c;

        if differs_a && differs_b || internal_corner_differ {
            differing_edge_count += 1;
        }
    }

    //println!("{pt} {differing_edge_count}");
    differing_edge_count
}

fn visit_region(pt: Point2, map: &mut Grid<Tile>) -> Region {
    assert!(!map[pt].visited);

    let mut region = Region {
        area: 0,
        edges: 0,
        corners: 0,
    };

    let mut frontier: VecDeque<Point2> = VecDeque::new();
    frontier.push_back(pt);

    let mut frontier_count = frontier.len();

    while frontier_count > 0 {
        for _ in 0..frontier_count {
            let frontier_pt = frontier.pop_front().unwrap();

            if map[frontier_pt].visited {
                continue;
            }

            map[frontier_pt].visited = true;

            region.area += 1;
            region.edges += edges(frontier_pt, map);
            region.corners += corners(frontier_pt, map);

            for dir in Direction4::all() {
                let neighbor_pt = frontier_pt + dir;

                if map.is_pos_in_bounds(neighbor_pt)
                    && !map[neighbor_pt].visited
                    && map[neighbor_pt].c == map[frontier_pt].c
                {
                    frontier.push_back(neighbor_pt);
                }
            }
        }

        frontier_count = frontier.len();
    }

    region
}

pub fn day_12_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut map: Grid<Tile> = Grid::parse_str(args.input, |c| Tile { c, visited: false }).unwrap();
    let mut total_cost: usize = 0;

    for pt in map.points() {
        if !map[pt].visited {
            let region = visit_region(pt, &mut map);
            total_cost += region.area * region.edges;
        }
    }

    Ok(total_cost.into())
}

pub fn day_12_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut map: Grid<Tile> = Grid::parse_str(args.input, |c| Tile { c, visited: false }).unwrap();
    let mut total_cost: usize = 0;

    for pt in map.points() {
        if !map[pt].visited {
            let region = visit_region(pt, &mut map);
            total_cost += region.area * region.corners;
        }
    }

    Ok(total_cost.into())
}

#[cfg(test)]
mod tests {
    use yuletide::SolverArgs;

    use super::*;

    fn parse(input: &str) -> Grid<Tile> {
        Grid::parse_str(input, |c| Tile { c, visited: false }).unwrap()
    }

    #[test]
    fn perimiter_a() {
        let region = visit_region(Point2::new(0, 0), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.edges, 10);
    }

    #[test]
    fn perimiter_b() {
        let region = visit_region(Point2::new(1, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.edges, 8);
    }

    #[test]
    fn perimiter_c() {
        let region = visit_region(Point2::new(2, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.edges, 10);
    }

    #[test]
    fn perimiter_d() {
        let region = visit_region(Point2::new(3, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 1);
        assert_eq!(region.edges, 4);
    }

    #[test]
    fn perimiter_e() {
        let region = visit_region(Point2::new(0, 3), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 3);
        assert_eq!(region.edges, 8);
    }

    #[test]
    fn corners_a() {
        let region = visit_region(Point2::new(0, 0), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.corners, 4);
    }

    #[test]
    fn corners_b() {
        let region = visit_region(Point2::new(1, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.corners, 4);
    }

    #[test]
    fn corners_c() {
        let region = visit_region(Point2::new(2, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.corners, 8);
    }

    #[test]
    fn custom_corners_t() {
        let region = visit_region(Point2::new(1, 1), &mut parse("ABA\nBBB"));
        assert_eq!(region.area, 4);
        assert_eq!(region.corners, 8);
    }

    #[test]
    fn custom_corners_internal_ne() {
        let region = visit_region(Point2::new(1, 1), &mut parse("AB\nAA"));
        assert_eq!(region.area, 3);
        assert_eq!(region.corners, 6);
    }

    #[test]
    fn internal_corners() {
        let region = visit_region(Point2::new(0, 0), &mut parse("AB\nBA\n"));
        assert_eq!(region.area, 1);
        assert_eq!(region.corners, 4);

        assert_eq!(
            4,
            visit_region(Point2::new(0, 1), &mut parse("AB\nBA\n")).corners
        );
        assert_eq!(
            4,
            visit_region(Point2::new(1, 0), &mut parse("AB\nBA\n")).corners
        );
        assert_eq!(
            4,
            visit_region(Point2::new(1, 1), &mut parse("AB\nBA\n")).corners
        );
    }

    #[test]
    fn corner_diagonals() {
        let region = visit_region(Point2::new(0, 0), &mut parse("AB\nBA\n"));
        assert_eq!(region.area, 1);
        assert_eq!(region.corners, 4);

        assert_eq!(
            4,
            visit_region(Point2::new(0, 1), &mut parse("AB\nBA\n")).corners
        );
        assert_eq!(
            4,
            visit_region(Point2::new(1, 0), &mut parse("AB\nBA\n")).corners
        );
        assert_eq!(
            4,
            visit_region(Point2::new(1, 1), &mut parse("AB\nBA\n")).corners
        );
    }

    #[test]
    fn e_shaped_region_corners() {
        let s = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(17, visit_region(Point2::new(0, 0), &mut parse(s)).area);
        assert_eq!(12, visit_region(Point2::new(0, 0), &mut parse(s)).corners);
        assert_eq!(
            aoc::Answer::Int(236),
            day_12_2(&SolverArgs { input: s }).unwrap()
        );
    }

    #[test]
    fn part_two_example() {
        let s = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(
            aoc::Answer::Int(368),
            day_12_2(&SolverArgs { input: s }).unwrap()
        );
    }
}
