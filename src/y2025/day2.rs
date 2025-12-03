use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {
    modpath: std::module_path!(),
    part_one: yt::SolverPart {
        func: day_2_1,
        examples: &[yt::Example {
            input: "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
            expected: aoc::Answer::Int(1227775554),
        }],
    },
    part_two: yt::SolverPart {
        func: day_2_1,
        examples: &[/*yt::Example {
            input: "",
            expected: aoc::Answer::Int(0),
        }*/],
    },
};

fn parse_ranges(input: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for range in input.trim().split(",") {
        let (first_id, last_id) = range.split_once("-").expect("int-int");
        ranges.push((
            first_id
                .parse()
                .unwrap_or_else(|_| panic!("invalid int for first id `{first_id}`")),
            last_id
                .parse()
                .unwrap_or_else(|_| panic!("invalid int for last id `{last_id}`")),
        ));
    }

    ranges
}

fn is_valid_id(id: usize) -> bool {
    let id: Vec<char> = id.to_string().chars().collect();
    if !id.is_empty() && id.len() % 2 == 0 {
        let mid: usize = id.len() / 2;

        for i in 0..mid {
            if id[i] != id[i + mid] {
                return true;
            }
        }

        return false;
    }

    true
}

pub fn day_2_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut sum: usize = 0;

    for (first_id, last_id) in parse_ranges(args.input) {
        sum += (first_id..=last_id)
            .filter(|i| !is_valid_id(*i))
            .sum::<usize>();
    }

    Ok(sum.into())
}

pub fn day_2_2(_input: &str) -> yt::Result<aoc::Answer> {
    Err(yt::SolverError::NotFinished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str_to_ranges() {
        assert_eq!(
            parse_ranges("11-22,95-115,998-1012,1188511880-1188511890"),
            vec![(11, 22), (95, 115), (998, 1012), (1188511880, 1188511890)]
        );
    }

    #[test]
    fn example_bad_ids() {
        assert_eq!(
            (11..=22)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![11, 22]
        );
        assert_eq!(
            (95..=115)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![99]
        );
        assert_eq!(
            (998..=1012)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![1010]
        );
        assert_eq!(
            (1188511880..=1188511890)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![1188511885]
        );
        assert_eq!(
            (222220..=222224)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![222222]
        );
        assert_eq!(
            (1698522..=1698528)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            Vec::<usize>::new()
        );
        assert_eq!(
            (446443..=446449)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![446446]
        );
        assert_eq!(
            (38593856..=38593862)
                .filter(|i| !is_valid_id(*i))
                .collect::<Vec<usize>>(),
            vec![38593859]
        );
    }
}
