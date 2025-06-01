use regex::Regex;
use std::sync::OnceLock;

static RE_CELL_FIND_INTS: OnceLock<Regex> = OnceLock::new();

/// Find all integers present in `text`, ignoring any values that are not digit
/// characters.
///
/// This utility is helpful for quickly extracting all integers from text when
/// they are separated by spaces, tabs etc and you don't expect any non-integer
/// values.
///
/// ```
/// use workshop::utils::find_ints;
///
/// assert_eq!(find_ints("123   -57 \n  2321"), vec![123, -57, 2321]);
/// assert_eq!(find_ints("123, -57xxx2321"), vec![123, -57, 2321]);
/// ```
pub fn find_ints(text: &str) -> Vec<i64> {
    let re = RE_CELL_FIND_INTS
        .get_or_init(|| Regex::new(r"-?[0-9]+").expect("find_ints regex failed to compile"));

    re.find_iter(text)
        .map(|m| str::parse::<i64>(m.as_str()).unwrap())
        .collect()
}

/// Return an iterator over the combination of `item`s taken two at a time with-
/// out repetition.
///
/// ```
/// use workshop::utils::pairwise_combinations;
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
        assert_eq!(find_ints(""), Vec::<i64>::new());
        assert_eq!(find_ints("5"), vec![5]);
        assert_eq!(find_ints("-51 19"), vec![-51, 19]);
        assert_eq!(
            find_ints("-51 19  513123 -9123 +35 zc 5x23"),
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
