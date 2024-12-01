use regex::Regex;
use std::sync::OnceLock;

static RE_CELL_FIND_INTS: OnceLock<Regex> = OnceLock::new();

pub fn find_ints(text: &str) -> Vec<i64> {
    let re = RE_CELL_FIND_INTS
        .get_or_init(|| Regex::new(r"-?[0-9]+").expect("find_ints regex failed to compile"));

    re.find_iter(text)
        .map(|m| str::parse::<i64>(m.as_str()).unwrap())
        .collect()
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
}
