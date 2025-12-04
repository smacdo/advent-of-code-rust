use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_3_1,
        examples: &[yt::Example {
            input: "987654321111111\n811111111111119\n234234234234278\n818181911112111\n",
            expected: aoc::Answer::Int(357),
        }],
    },
    part_two: yt::SolverPart {
        func: day_3_2,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

struct DigitWithIndex {
    index: usize,
    digit: u8,
}

fn find_largest_digit_from_right(bank: &[u8], start: usize, end: usize) -> DigitWithIndex {
    assert!(end > start);
    assert!(end <= bank.len());

    let mut best_index = end - 1;

    for i in (start..end).rev() {
        if bank[i] >= bank[best_index] {
            best_index = i;
        }
    }

    DigitWithIndex {
        index: best_index,
        digit: bank[best_index],
    }
}

fn parse_banks(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("number") as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn day_3_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut sum: u64 = 0;

    for bank in parse_banks(args.input) {
        // find the largest digit from the left not including the last digit in the bank
        let a = find_largest_digit_from_right(&bank, 0, bank.len() - 1);

        // find the largest digit to the right of the digit we just found.
        let b = find_largest_digit_from_right(&bank, a.index + 1, bank.len());

        let joltage = a.digit * 10 + b.digit;
        sum += joltage as u64;
    }

    Ok(sum.into())
}

pub fn day_3_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str_to_banks() {
        assert_eq!(
            parse_banks("123\n798\n654"),
            vec![vec![1, 2, 3], vec![7, 9, 8], vec![6, 5, 4]]
        );
    }

    #[test]
    fn find_largest_digit_from_right_tests() {
        assert_eq!(find_largest_digit_from_right(&[3], 0, 1).digit, 3);

        assert_eq!(find_largest_digit_from_right(&[4, 2, 5], 0, 3).digit, 5);
        assert_eq!(find_largest_digit_from_right(&[2, 3, 1], 0, 3).digit, 3);
        assert_eq!(find_largest_digit_from_right(&[7, 3, 1], 0, 3).digit, 7);
        assert_eq!(find_largest_digit_from_right(&[5, 6, 7], 0, 2).digit, 6);
        assert_eq!(find_largest_digit_from_right(&[9, 3, 7], 0, 2).digit, 9);
        assert_eq!(find_largest_digit_from_right(&[5, 6, 7], 1, 3).digit, 7);
        assert_eq!(find_largest_digit_from_right(&[9, 8, 7], 1, 3).digit, 8);
        assert_eq!(find_largest_digit_from_right(&[9, 8, 7], 1, 2).digit, 8);
    }
}
