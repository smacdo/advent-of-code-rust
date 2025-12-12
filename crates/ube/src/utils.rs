use regex::Regex;
use std::{str::FromStr, sync::OnceLock};

static RE_CELL_FIND_INTS: OnceLock<Regex> = OnceLock::new();

/// Find all digits (0-9 chars) in `text`, ignoring any values that are not
/// digit characters.
///
/// This utility is helpful for quickly extracting all digits as ints from text.
/// Digits can be directly adjacent to each other; they will be treated as
/// separate values. They can also be separated by spaces, commas etc. All non-
/// digit values are ignored.
///
/// ```
/// use ube::utils::find_digits;
///
/// assert_eq!(find_digits("01859"), vec![0, 1, 8, 5, 9]);
/// assert_eq!(find_digits("0,1  8.5hi9"), vec![0, 1, 8, 5, 9]);
/// ```
pub fn find_digits(text: &str) -> Vec<u8> {
    text.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Vec<_>>()
}

/// Find all integers present in `text`, ignoring any values that are not digit
/// characters.
///
/// This utility is helpful for quickly extracting all integers from text when
/// they are separated by spaces, tabs etc and you don't expect any non-integer
/// values.
///
/// ```
/// use ube::utils::find_ints;
///
/// assert_eq!(find_ints("123   -57 \n  2321"), vec![123, -57, 2321]);
/// assert_eq!(find_ints("123, -57xxx2321"), vec![123, -57, 2321]);
/// ```
pub fn find_ints<F: FromStr>(text: &str) -> Result<Vec<F>, <F as FromStr>::Err> {
    let re = RE_CELL_FIND_INTS
        .get_or_init(|| Regex::new(r"-?[0-9]+").expect("find_ints regex failed to compile"));

    re.find_iter(text)
        .map(|m| str::parse::<F>(m.as_str()))
        .collect()
}

/// Return an iterator over the combination of `item`s taken two at a time with-
/// out repetition.
///
/// ```
/// use ube::utils::pairwise_combinations;
///
/// // (abc) -> [(ab), (ac), (bc)]
/// assert_eq!(
///   pairwise_combinations(&['a', 'b', 'c']).collect::<Vec<_>>(),
///   vec![(&'a', &'b'), (&'a', &'c'), (&'b', &'c')]
/// );
/// ```
pub fn pairwise_combinations<T>(items: &[T]) -> PairwiseCombinations<'_, T> {
    PairwiseCombinations::new(items)
}

/// Iterates over the combination of `item`s taken two at a time without
/// repetition.
pub struct PairwiseCombinations<'a, T> {
    items: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T> PairwiseCombinations<'a, T> {
    fn new(items: &'a [T]) -> Self {
        Self { items, i: 0, j: 1 }
    }
}

impl<'a, T> Iterator for PairwiseCombinations<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.items.len() && self.j < self.items.len() {
            let i = self.i;
            let j = self.j;

            self.j += 1;

            if self.j >= self.items.len() {
                self.i += 1;
                self.j = self.i + 1;
            }

            Some((&self.items[i], &self.items[j]))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_ints_in_string() {
        assert_eq!(find_ints::<i64>("").unwrap(), Vec::<i64>::new());
        assert_eq!(find_ints::<u32>("5").unwrap(), vec![5]);
        assert_eq!(find_ints::<i64>("-51 19").unwrap(), vec![-51, 19]);
        assert_eq!(
            find_ints::<isize>("-51 19  513123 -9123 +35 zc 5x23").unwrap(),
            vec![-51, 19, 513123, -9123, 35, 5, 23]
        );
    }

    #[test]
    fn test_combinations() {
        assert_eq!(
            pairwise_combinations(&[10, 20, 30, 40])
                .map(|(a, b)| (*a, *b))
                .collect::<Vec<_>>(),
            vec![(10, 20), (10, 30), (10, 40), (20, 30), (20, 40), (30, 40)]
        );
    }

    #[test]
    fn test_combinations_empty() {
        assert_eq!(
            pairwise_combinations(&[])
                .map(|(a, b)| (*a, *b))
                .collect::<Vec<(i32, i32)>>(),
            vec![]
        );
    }

    #[test]
    fn test_combinations_single() {
        assert_eq!(
            pairwise_combinations(&[36])
                .map(|(a, b)| (*a, *b))
                .collect::<Vec<_>>(),
            vec![]
        );
    }

    #[test]
    fn test_combinations_double() {
        assert_eq!(
            pairwise_combinations(&[36, -18])
                .map(|(a, b)| (*a, *b))
                .collect::<Vec<_>>(),
            vec![(36, -18)]
        );
    }
}
