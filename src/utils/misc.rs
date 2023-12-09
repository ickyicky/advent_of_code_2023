use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").expect("invalid regex");
}

pub fn parse_numbers(line: &str) -> Vec<u64> {
    let mut numbers = vec![];

    for cap in NUMBER_REGEX.captures_iter(line) {
        numbers.push(cap[1].parse::<u64>().unwrap());
    }

    numbers
}
