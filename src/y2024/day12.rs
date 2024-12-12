use std::collections::VecDeque;

use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::{Direction4, Grid, Point2};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(12),
    year: Year(2024),
    part_one: SolverPart {
        func: day_12_1,
        examples: &[(
            Answer::Int(1930),
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        )],
    },
    part_two: SolverPart {
        func: day_12_2,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
};

#[derive(Debug, Clone)]
struct Tile {
    pub c: char,
    pub visited: bool,
}

struct Region {
    pub area: usize,
    pub perimeter: usize,
}

fn perimeter(pt: Point2, map: &Grid<Tile>) -> usize {
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

fn visit_region(pt: Point2, map: &mut Grid<Tile>) -> Region {
    assert!(!map[pt].visited);

    let mut region = Region {
        area: 0,
        perimeter: 0,
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
            region.perimeter += perimeter(frontier_pt, map);

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

pub fn day_12_1(input: &str) -> Result<Answer> {
    let mut map: Grid<Tile> = Grid::parse_str(input, |c| Tile { c, visited: false }).unwrap();
    let mut total_cost: usize = 0;

    for pt in map.points() {
        if !map[pt].visited {
            let region = visit_region(pt, &mut map);
            total_cost += region.area * region.perimeter;
        }
    }

    Ok(total_cost.into())
}

pub fn day_12_2(_input: &str) -> Result<Answer> {
    Err(advent_of_code_data::registry::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Grid<Tile> {
        Grid::parse_str(input, |c| Tile { c, visited: false }).unwrap()
    }

    #[test]
    fn region_a() {
        let region = visit_region(Point2::new(0, 0), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.perimeter, 10);
    }

    #[test]
    fn region_b() {
        let region = visit_region(Point2::new(1, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.perimeter, 8);
    }

    #[test]
    fn region_c() {
        let region = visit_region(Point2::new(2, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 4);
        assert_eq!(region.perimeter, 10);
    }

    #[test]
    fn region_d() {
        let region = visit_region(Point2::new(3, 1), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 1);
        assert_eq!(region.perimeter, 4);
    }

    #[test]
    fn region_e() {
        let region = visit_region(Point2::new(0, 3), &mut parse("AAAA\nBBCD\nBBCC\nEEEC"));
        assert_eq!(region.area, 3);
        assert_eq!(region.perimeter, 8);
    }
}
