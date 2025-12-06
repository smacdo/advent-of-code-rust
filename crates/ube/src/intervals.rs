use std::ops::RangeInclusive;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseIntervalError {
    #[error("could not find the - split char in `{}`", .0)]
    SplitCharNotFound(String),
    #[error("could not parse the start value `{}` as a usize", .0)]
    StartValueParseError(String),
    #[error("could not parse the end value `{}` as a usize", .0)]
    EndValueParseError(String),
}

/// Parses an interval string into start and end values.
///
/// # Example
///
/// ```
/// use ube::intervals::parse_interval;
///
/// let (start, end) = parse_interval("10-20").expect("a valid interval");
/// assert_eq!((start, end), (10, 20));
/// ```
pub fn parse_interval(input: &str) -> Result<(usize, usize), ParseIntervalError> {
    let (first, end) = input
        .split_once("-")
        .ok_or_else(|| ParseIntervalError::SplitCharNotFound(input.to_string()))?;
    Ok((
        first
            .parse()
            .map_err(|_| ParseIntervalError::StartValueParseError(first.to_string()))?,
        end.parse()
            .map_err(|_| ParseIntervalError::EndValueParseError(end.to_string()))?,
    ))
}

/// Merges overlapping intervals into a minimal set of non-overlapping intervals.
///
/// Intervals are sorted by start position, then consecutive overlapping intervals
/// are combined. Returns an empty vector if the input is empty.
///
/// # Example
///
/// ```
/// use ube::intervals::merge_intervals;
///
/// let intervals = vec![1..=3, 5..=7, 2..=6, 10..=12];
/// let merged = merge_intervals(intervals);
/// assert_eq!(merged, vec![1..=7, 10..=12]);
/// ```
pub fn merge_intervals(mut intervals: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    // Sort intervals in ascending order by their start value before merging.
    intervals.sort_by_key(|r: &RangeInclusive<usize>| *r.start());

    // Initialize the output with the first interval, and then repeatedly try merging or adding the
    // remaining intervals.
    let mut merged_intervals: Vec<RangeInclusive<usize>> = Vec::new();

    if let Some(first) = intervals.first().cloned() {
        merged_intervals.push(first);

        for current in intervals.iter().skip(1) {
            let last_merged: RangeInclusive<usize> = merged_intervals.last().unwrap().clone();

            if current.start() <= last_merged.end() {
                let start = last_merged.start();
                let last = if current.end() > last_merged.end() {
                    current.end()
                } else {
                    last_merged.end()
                };

                let last_index = merged_intervals.len() - 1;
                merged_intervals[last_index] = *start..=*last;
            } else {
                merged_intervals.push(current.clone());
            }
        }
    }

    merged_intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_digit_interval() {
        assert_eq!(parse_interval("1-5"), Ok((1, 5)));
    }

    #[test]
    fn parse_multi_digit_interval() {
        assert_eq!(parse_interval("10-100"), Ok((10, 100)));
    }

    #[test]
    fn parse_same_start_and_end() {
        assert_eq!(parse_interval("5-5"), Ok((5, 5)));
    }

    #[test]
    fn parse_zero_start() {
        assert_eq!(parse_interval("0-10"), Ok((0, 10)));
    }

    #[test]
    fn parse_missing_separator() {
        assert_eq!(
            parse_interval("100"),
            Err(ParseIntervalError::SplitCharNotFound("100".to_string()))
        );
    }

    #[test]
    fn parse_invalid_number() {
        assert_eq!(
            parse_interval("abc-10"),
            Err(ParseIntervalError::StartValueParseError("abc".to_string()))
        );
        assert_eq!(
            parse_interval("10-def"),
            Err(ParseIntervalError::EndValueParseError("def".to_string()))
        );
    }

    #[test]
    fn merge_empty_input() {
        assert_eq!(merge_intervals(vec![]), vec![]);
    }

    #[test]
    fn merge_single_interval() {
        assert_eq!(merge_intervals(vec![1..=5]), vec![1..=5]);
    }

    #[test]
    fn merge_overlapping_intervals() {
        let result = merge_intervals(vec![1..=3, 2..=5]);
        assert_eq!(result, vec![1..=5]);
    }

    #[test]
    fn merge_multiple_overlaps_collapse() {
        let result = merge_intervals(vec![1..=3, 2..=6, 5..=9]);
        assert_eq!(result, vec![1..=9]);
    }

    #[test]
    fn merge_non_overlapping_stay_separate() {
        let result = merge_intervals(vec![1..=2, 5..=7, 10..=12]);
        assert_eq!(result, vec![1..=2, 5..=7, 10..=12]);
    }

    #[test]
    fn merge_touching_intervals_merge() {
        let result = merge_intervals(vec![1..=3, 3..=5]);
        assert_eq!(result, vec![1..=5]);
    }

    #[test]
    fn merge_unsorted_input() {
        let result = merge_intervals(vec![5..=7, 1..=3, 2..=6]);
        assert_eq!(result, vec![1..=7]);
    }
}
