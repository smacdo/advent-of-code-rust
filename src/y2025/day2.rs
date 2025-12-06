use advent_of_code_data as aoc;
use ube::intervals::parse_interval;
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
        func: day_2_2,
        examples: &[yt::Example {
            input: "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
            expected: aoc::Answer::Int(4174379265),
        }],
    },
};

fn parse_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .split(",")
        .map(|range| parse_interval(range).unwrap())
        .collect::<Vec<_>>()
}

fn is_valid_id_p1(id: usize) -> bool {
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

fn is_valid_id_p2(id: usize) -> bool {
    //eprintln!("-----[{id}]-----");

    let id: Vec<char> = id.to_string().chars().collect();
    if id.len() > 1 {
        let mid: usize = id.len() / 2;

        // try patterns of size `k` starting from 1 .. (Eg, one digit).
        for k in 1..=mid {
            // skip if this number can't be divided into chunks of size k?
            if id.len() % k != 0 {
                continue;
            }

            if has_pattern_of_size(&id, k) {
                return false;
            }
        }
    }

    true
}

fn has_pattern_of_size(id: &[char], k: usize) -> bool {
    let subsets = id.len() / k;
    //eprintln!("\tk={k}, subsets={subsets}");

    for i in 0..k {
        //eprintln!("\t\ti={i}");

        // check if its the same character in each subset
        for j in 1..subsets {
            /*eprintln!(
                "\t\t\tj={j} s[{}] {} =?= s[{}] {}",
                i + j * k,
                id[i + j * k],
                i + (j - 1) * k,
                id[i + (j - 1) * k]
            );*/

            if id[i + j * k] != id[i + (j - 1) * k] {
                return false;
            }
        }
    }

    true
}

pub fn day_2_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut sum: usize = 0;

    for (first_id, last_id) in parse_ranges(args.input) {
        sum += (first_id..=last_id)
            .filter(|i| !is_valid_id_p1(*i))
            .sum::<usize>();
    }

    Ok(sum.into())
}

pub fn day_2_2(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {
    let mut sum: usize = 0;

    for (first_id, last_id) in parse_ranges(args.input) {
        sum += (first_id..=last_id)
            .filter(|i| !is_valid_id_p2(*i))
            .sum::<usize>();
    }

    Ok(sum.into())
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
    fn example_bad_ids_p2() {
        assert_eq!(
            (11..=22)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![11, 22]
        );
        assert_eq!(
            (95..=115)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![99, 111]
        );
        assert_eq!(
            (998..=1012)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![999, 1010]
        );
        assert_eq!(
            (1188511880..=1188511890)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![1188511885]
        );
        assert_eq!(
            (222220..=222224)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![222222]
        );
        assert_eq!(
            (1698522..=1698528)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            Vec::<usize>::new()
        );
        assert_eq!(
            (446443..=446449)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![446446]
        );
        assert_eq!(
            (38593856..=38593862)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![38593859]
        );
        assert_eq!(
            (565653..=565659)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![565656]
        );
        assert_eq!(
            (824824821..=824824827)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![824824824]
        );
        assert_eq!(
            (2121212118..=2121212124)
                .filter(|i| !is_valid_id_p2(*i))
                .collect::<Vec<usize>>(),
            vec![2121212121]
        );
    }

    #[test]
    fn check_p2s() {
        assert!(!is_valid_id_p2(11));
        assert!(!is_valid_id_p2(111));
        assert!(!is_valid_id_p2(1111));
        assert!(!is_valid_id_p2(1010));
        assert!(!is_valid_id_p2(321321));

        assert!(is_valid_id_p2(1));
        assert!(is_valid_id_p2(101));
        assert!(is_valid_id_p2(1221));
        assert!(is_valid_id_p2(1211));
        assert!(is_valid_id_p2(121213));
    }

    #[test]
    fn example_bad_ids_p1() {
        assert_eq!(
            (11..=22)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![11, 22]
        );
        assert_eq!(
            (95..=115)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![99]
        );
        assert_eq!(
            (998..=1012)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![1010]
        );
        assert_eq!(
            (1188511880..=1188511890)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![1188511885]
        );
        assert_eq!(
            (222220..=222224)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![222222]
        );
        assert_eq!(
            (1698522..=1698528)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            Vec::<usize>::new()
        );
        assert_eq!(
            (446443..=446449)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![446446]
        );
        assert_eq!(
            (38593856..=38593862)
                .filter(|i| !is_valid_id_p1(*i))
                .collect::<Vec<usize>>(),
            vec![38593859]
        );
    }
}
