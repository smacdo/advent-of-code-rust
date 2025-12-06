use advent_of_code_data as aoc;
use ube::utils::find_digits;
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
        examples: &[yt::Example {
            input: "987654321111111\n811111111111119\n234234234234278\n818181911112111\n",
            expected: aoc::Answer::Int(3121910778619),
        }],
    },
};

fn parse_banks(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(find_digits).collect::<Vec<_>>()
}

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

fn find_largest(bank: &[u8], k: usize, start: usize) -> usize {
    if k == 0 {
        return 0;
    }

    let end = bank.len() - k + 1;
    let DigitWithIndex { index, digit } = find_largest_digit_from_right(bank, start, end);

    let digit_tens = usize::pow(10, k as u32 - 1);
    (digit as usize) * digit_tens + find_largest(bank, k - 1, index + 1)
}

pub fn day_3_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Ok(parse_banks(args.input)
        .into_iter()
        .map(|bank| find_largest(&bank, 2, 0))
        .sum::<usize>()
        .into())
}

pub fn day_3_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    Ok(parse_banks(args.input)
        .into_iter()
        .map(|bank| find_largest(&bank, 12, 0))
        .sum::<usize>()
        .into())
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

    #[test]
    fn find_largest_matches_examples() {
        assert_eq!(
            find_largest(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2, 0),
            98
        );
        assert_eq!(
            find_largest(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2, 0),
            89
        );
        assert_eq!(
            find_largest(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2, 0),
            78
        );
        assert_eq!(
            find_largest(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2, 0),
            92
        );

        assert_eq!(
            find_largest(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12, 0),
            987654321111
        );
        assert_eq!(
            find_largest(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12, 0),
            811111111119
        );
        assert_eq!(
            find_largest(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12, 0),
            434234234278
        );
        assert_eq!(
            find_largest(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12, 0),
            888911112111
        );
    }
}
