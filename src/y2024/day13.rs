use std::collections::HashMap;

use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::Point2;
use linkme::distributed_slice;
use regex::Regex;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(13),
    year: Year(2024),
    part_one: SolverPart {
        func: day_13_1,
        examples: &[(
            Answer::Int(480),
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        )],
    },
    part_two: SolverPart {
        func: day_13_2,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
};

const COST_A: isize = 3;
const COST_B: isize = 1;

#[derive(Debug, PartialEq)]
struct Machine {
    pub button_a: Point2,
    pub button_b: Point2,
    pub prize: Point2,
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut button_a = Some(Point2::zero());
    let mut button_b = Some(Point2::zero());
    let mut prize = Some(Point2::zero());
    let mut machines: Vec<Machine> = Vec::new();

    enum ExpectedLine {
        ButtonA,
        ButtonB,
        Prize,
        Newline,
    }

    let mut expected_line = ExpectedLine::ButtonA;
    let button_regex = Regex::new(r"[A-ZA-z ]+: X\+(?<X>\d+), Y\+(?<Y>\d+)").unwrap();
    let prize_regex = Regex::new(r"[A-ZA-z ]+: X=(?<X>\d+), Y=(?<Y>\d+)").unwrap();

    for line in input.lines() {
        match expected_line {
            ExpectedLine::ButtonA => {
                let cap = button_regex.captures(line).unwrap();
                button_a = Some(Point2::new(
                    cap["X"].parse::<isize>().unwrap(),
                    cap["Y"].parse::<isize>().unwrap(),
                ));
                expected_line = ExpectedLine::ButtonB;
            }
            ExpectedLine::ButtonB => {
                let cap = button_regex.captures(line).unwrap();
                button_b = Some(Point2::new(
                    cap["X"].parse::<isize>().unwrap(),
                    cap["Y"].parse::<isize>().unwrap(),
                ));
                expected_line = ExpectedLine::Prize;
            }
            ExpectedLine::Prize => {
                let cap = prize_regex.captures(line).unwrap();
                prize = Some(Point2::new(
                    cap["X"].parse::<isize>().unwrap(),
                    cap["Y"].parse::<isize>().unwrap(),
                ));
                expected_line = ExpectedLine::Newline;
            }
            ExpectedLine::Newline => {
                assert!(line.is_empty());

                machines.push(Machine {
                    button_a: button_a.unwrap(),
                    button_b: button_b.unwrap(),
                    prize: prize.unwrap(),
                });

                button_a = None;
                button_b = None;
                prize = None;

                expected_line = ExpectedLine::ButtonA;
            }
        }
    }

    // handle last entry which does not terminate with a newline
    if let Some(button_a) = button_a {
        machines.push(Machine {
            button_a,
            button_b: button_b.unwrap(),
            prize: prize.unwrap(),
        });
    }

    machines
}

fn cheapest_win(machine: &Machine) -> Option<isize> {
    cheapest_win_itr(Point2::zero(), machine, 0, 0, 0, 1, &mut HashMap::new())
}

fn cheapest_win2(machine: &Machine) -> Option<isize> {
    cheapest_win_itr(
        Point2::zero(),
        machine,
        0,
        0,
        0,
        10000000000000,
        &mut HashMap::new(),
    )
}

fn cheapest_win_itr(
    pos: Point2,
    machine: &Machine,
    tokens: isize,
    step_a: usize,
    step_b: usize,
    step_modifier: usize,
    cache: &mut HashMap<Point2, Option<isize>>,
) -> Option<isize> {
    if cache.contains_key(&pos) {
        return cache[&pos];
    }

    if pos.x > machine.prize.x
        || pos.y > machine.prize.y
        || step_a > 100 * step_modifier
        || step_b > 100 * step_modifier
    {
        None
    } else if pos == machine.prize {
        println!("found prize at {pos}");
        Some(tokens)
    } else {
        let a = cheapest_win_itr(
            pos + machine.button_a,
            machine,
            tokens + COST_A,
            step_a + 1,
            step_b,
            step_modifier,
            cache,
        );

        let b = cheapest_win_itr(
            pos + machine.button_b,
            machine,
            tokens + COST_B,
            step_a,
            step_b + 1,
            step_modifier,
            cache,
        );

        let cost = match (a, b) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (_, Some(b)) => Some(b),
            _ => None,
        };

        cache
            .entry(pos)
            .and_modify(|x| assert!(*x == cost))
            .or_insert(cost);
        cost
    }
}

fn solve_linear_equation(machine: &Machine) -> Option<isize> {
    // There are two equations that need to be solved:
    // p.x = a.x * a + b.x * b
    // p.y = a.y * a + b.y * b
    //
    // TODO: Use Z3 to solve the equations.
    // ```
    // from z3 import Int, Optimize, sat
    // X = Int('X')
    // Y = Int('Y')
    // opt = Optimize()
    // opt.add(a_x * X + b_x * Y == p_x)
    // opt.add(a_y * X + b_y * Y == p_y)
    // opt.minimize(3 * X + Y)
    // if opt.check() == sat:
    //   model = opt.model()
    //   solution_x = model[X].as_long
    //   solution_y = model[Y].as_long
    // ```
    let a = machine.button_a;
    let b = machine.button_b;
    let p = machine.prize;

    let b_moves = (p.y * a.x - p.x * a.y) / (b.y * a.x - b.x * a.y);
    let a_moves = (p.x - b_moves * b.x) / a.x;

    if (a.x * a_moves + b.x * b_moves, a.y * a_moves + b.y * b_moves) != (p.x, p.y) {
        return None;
    }

    Some(a_moves * COST_A + b_moves * COST_B)
}

pub fn day_13_1(input: &str) -> Result<Answer> {
    let machines = parse_input(input);
    let fewest_tokens: isize = machines
        .into_iter()
        .filter_map(|machine| solve_linear_equation(&machine))
        .sum();

    Ok(fewest_tokens.into())
}

pub fn day_13_2(input: &str) -> Result<Answer> {
    let machines = parse_input(input);
    let fewest_tokens: isize = machines
        .into_iter()
        .map(|mut machine| {
            machine.prize += Point2::new(10000000000000, 10000000000000);
            machine
        })
        .filter_map(|machine| solve_linear_equation(&machine))
        .sum();

    Ok(fewest_tokens.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input() {
        assert_eq!(
            parse_input(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
            ),
            vec![
                Machine {
                    button_a: Point2::new(94, 34),
                    button_b: Point2::new(22, 67),
                    prize: Point2::new(8400, 5400)
                },
                Machine {
                    button_a: Point2::new(26, 66),
                    button_b: Point2::new(67, 21),
                    prize: Point2::new(12748, 12176)
                },
                Machine {
                    button_a: Point2::new(17, 86),
                    button_b: Point2::new(84, 37),
                    prize: Point2::new(7870, 6450)
                },
                Machine {
                    button_a: Point2::new(69, 23),
                    button_b: Point2::new(27, 71),
                    prize: Point2::new(18641, 10279)
                }
            ]
        );
    }

    #[test]
    fn part_1_example_1() {
        assert_eq!(
            cheapest_win(&Machine {
                button_a: Point2::new(94, 34),
                button_b: Point2::new(22, 67),
                prize: Point2::new(8400, 5400)
            },),
            Some(280)
        );
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(
            cheapest_win(&Machine {
                button_a: Point2::new(26, 66),
                button_b: Point2::new(67, 21),
                prize: Point2::new(12748, 12176)
            },),
            None
        );
    }

    #[test]
    fn part_1_example_3() {
        assert_eq!(
            cheapest_win(&Machine {
                button_a: Point2::new(17, 86),
                button_b: Point2::new(84, 37),
                prize: Point2::new(7870, 6450)
            },),
            Some(200)
        );
    }

    #[test]
    fn part_1_example_4() {
        assert_eq!(
            cheapest_win(&Machine {
                button_a: Point2::new(69, 23),
                button_b: Point2::new(27, 71),
                prize: Point2::new(18641, 10279)
            },),
            None
        );
    }
}
