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

// TODO: Write tests for `find_ints`.
