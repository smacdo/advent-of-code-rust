use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_1_1,
        examples: &[yt::Example {
            input: "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82",
            expected: aoc::Answer::Int(3),
        }],
    },
    part_two: yt::SolverPart {
        func: day_1_2,
        examples: &[yt::Example {
            input: "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82",
            expected: aoc::Answer::Int(6),
        }],
    },
};

struct Dial {
    value: isize,
    count_zero_after_rotation: isize,
    count_zero_during_rotation: isize,
}

impl Dial {
    const RANGE: isize = 100;
    pub fn new(start: isize) -> Self {
        Self {
            value: start,
            count_zero_after_rotation: 0,
            count_zero_during_rotation: 0,
        }
    }

    pub fn turn(&mut self, rotation: Rotation) -> &Self {
        match rotation {
            Rotation::Left(distance) => {
                let i = self.value - distance as isize;

                if i >= 0 {
                    self.value = i;
                } else {
                    let zero_passes = i.div_euclid(Self::RANGE).abs();
                    self.count_zero_during_rotation += zero_passes;

                    if self.value == 0 {
                        self.count_zero_during_rotation -= 1;
                    }

                    self.value = i.rem_euclid(Self::RANGE);
                }
            }
            Rotation::Right(distance) => {
                let i = self.value + distance as isize;

                if i < Self::RANGE {
                    self.value = i;
                } else {
                    let zero_passes: isize = i.div_euclid(Self::RANGE);

                    self.count_zero_during_rotation += zero_passes;
                    self.value = i % Self::RANGE;

                    if self.value == 0 {
                        self.count_zero_during_rotation -= 1;
                    }
                }
            }
        }

        if self.value == 0 {
            self.count_zero_after_rotation += 1
        }

        self
    }
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    let mut rotations: Vec<Rotation> = Vec::new();

    for line in input.lines() {
        let (r, dist) = line.split_at(1);
        rotations.push(match r {
            "L" => Rotation::Left(dist.parse().expect("int")),
            "R" => Rotation::Right(dist.parse().expect("int")),
            _ => panic!("unknown direction char"),
        });
    }

    rotations
}

#[derive(Debug, PartialEq)]
pub enum Rotation {
    Left(usize),
    Right(usize),
}

pub fn day_1_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut dial = Dial::new(50);

    for r in parse_rotations(args.input) {
        dial.turn(r);
    }

    Ok(dial.count_zero_after_rotation.into())
}

pub fn day_1_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut dial = Dial::new(50);

    for r in parse_rotations(args.input) {
        dial.turn(r);
    }

    Ok((dial.count_zero_after_rotation + dial.count_zero_during_rotation).into())
}

#[cfg(test)]
mod tests {
    use either::Either::Left;

    use super::*;

    #[test]
    fn parse_str_to_rotations_vec() {
        assert_eq!(
            parse_rotations("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"),
            vec![
                Rotation::Left(68),
                Rotation::Left(30),
                Rotation::Right(48),
                Rotation::Left(5),
                Rotation::Right(60),
                Rotation::Left(55),
                Rotation::Left(1),
                Rotation::Left(99),
                Rotation::Right(14),
                Rotation::Left(82)
            ]
        );

        assert_eq!(
            parse_rotations("L0\nL100\nR99999"),
            vec![
                Rotation::Left(0),
                Rotation::Left(100),
                Rotation::Right(99999),
            ]
        );
    }

    #[test]
    fn large_rotation() {
        let mut d = Dial::new(50);
        assert_eq!(d.turn(Rotation::Right(1000)).value, 50);
        assert_eq!(d.count_zero_during_rotation, 10);
        assert_eq!(d.count_zero_after_rotation, 0);
    }

    #[test]
    fn left_ending_on_zero() {
        let mut d = Dial::new(50);
        assert_eq!(d.turn(Rotation::Left(50)).value, 0);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Right(50)).value, 50);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Left(50)).value, 0);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 2);

        assert_eq!(d.turn(Rotation::Left(50)).value, 50);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 2);
    }

    #[test]
    fn right_ending_on_zero() {
        let mut d = Dial::new(50);
        assert_eq!(d.turn(Rotation::Right(50)).value, 0);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Left(50)).value, 50);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Right(50)).value, 0);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 2);

        assert_eq!(d.turn(Rotation::Right(50)).value, 50);

        assert_eq!(d.count_zero_during_rotation, 0);
        assert_eq!(d.count_zero_after_rotation, 2);
    }

    #[test]
    fn extra_pass_left_landing_on_zero() {
        let mut d = Dial::new(50);
        assert_eq!(d.turn(Rotation::Left(150)).value, 0);
        assert_eq!(d.turn(Rotation::Left(50)).value, 50);

        assert_eq!(d.count_zero_during_rotation, 1);
        assert_eq!(d.count_zero_after_rotation, 1);
    }

    #[test]
    fn check_counts_per_step() {
        let mut d = Dial::new(50);
        assert_eq!(d.value, 50);

        assert_eq!(d.turn(Rotation::Left(68)).value, 82);
        assert_eq!(d.count_zero_during_rotation, 1);
        assert_eq!(d.count_zero_after_rotation, 0);

        assert_eq!(d.turn(Rotation::Left(30)).value, 52);
        assert_eq!(d.count_zero_during_rotation, 1);
        assert_eq!(d.count_zero_after_rotation, 0);

        assert_eq!(d.turn(Rotation::Right(48)).value, 0);
        assert_eq!(d.count_zero_during_rotation, 1);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Left(5)).value, 95);
        assert_eq!(d.count_zero_during_rotation, 1);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Right(60)).value, 55);
        assert_eq!(d.count_zero_during_rotation, 2);
        assert_eq!(d.count_zero_after_rotation, 1);

        assert_eq!(d.turn(Rotation::Left(55)).value, 0);
        assert_eq!(d.count_zero_during_rotation, 2);
        assert_eq!(d.count_zero_after_rotation, 2);

        assert_eq!(d.turn(Rotation::Left(1)).value, 99);
        assert_eq!(d.count_zero_during_rotation, 2);
        assert_eq!(d.count_zero_after_rotation, 2);

        assert_eq!(d.turn(Rotation::Left(99)).value, 0);
        assert_eq!(d.count_zero_during_rotation, 2);
        assert_eq!(d.count_zero_after_rotation, 3);

        assert_eq!(d.turn(Rotation::Right(14)).value, 14);
        assert_eq!(d.count_zero_during_rotation, 2);
        assert_eq!(d.count_zero_after_rotation, 3);

        assert_eq!(d.turn(Rotation::Left(82)).value, 32);
        assert_eq!(d.count_zero_during_rotation, 3);
        assert_eq!(d.count_zero_after_rotation, 3);
    }
}
