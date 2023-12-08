use clap::Parser;
use regex::Regex;
use lazy_static::lazy_static;

use advent_of_code_2023::utils::file_reader::read_lines;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").expect("invalid regex");
}


fn parse_numbers(line: &String) -> Vec::<u64> {
    let mut numbers = vec![];

    for cap in NUMBER_REGEX.captures_iter(line) {
        numbers.push(cap[1].parse::<u64>().unwrap());
    }

    numbers
}


fn solve_equation(a: f64, b: f64, c: f64) -> Option::<(u64, u64)> {
    let delta = b * b - 4.0 * a * c;

    if delta < 0.0 {
        return None;
    }

    let delta_sqrt = delta.sqrt();

    let x1 = (-b + delta_sqrt) / (2.0 * a);
    let x2 = (-b - delta_sqrt) / (2.0 * a);

    let mut start = f64::min(x1, x2);
    let mut end = f64::max(x1, x2);

    if start % 1.0 != 0.0 {
        start = start.ceil();
    } else {
        start = start.ceil() + 1.0;
    }

    if end % 1.0 != 0.0 {
        end = end.floor();
    } else {
        end = end.floor() - 1.0;
    }

    if end < 0.0 {
        return None;
    }

    if start < 0.0 {
        start = 0.0;
    }

    let start = start as u64;
    let end = end as u64;

    // sanity checks here, just in case
    if start > end {
        return None;
    }

    Some((start, end))
}


fn solver(t: u64, d: u64) -> u64 {
    if let Some((x1, x2)) = solve_equation(1.0, -(t as f64), d as f64) {
        return x2 - x1 + 1;
    }

    0
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index=1)]
    input_file: String,

    #[arg(short, long, default_value_t = false)]
    ignore_spaces: bool,
}


fn main() {
    let args = Args::parse();

    let mut lines = read_lines(args.input_file).expect("failed to read input file");
    let mut result = 0;

    let mut times_str = lines
        .next()
        .expect("first line should contain numbers")
        .expect("first line should contain numbers")
    ;

    let mut distances_str = lines
        .next()
        .expect("first line should contain numbers")
        .expect("first line should contain numbers")
    ;

    if args.ignore_spaces {
        times_str = times_str.replace(' ', "");
        distances_str = distances_str.replace(' ', "");
    }


    let times = parse_numbers(&times_str);
    let distances = parse_numbers(&distances_str);

    if times.len() != distances.len() {
        panic!("times and distances should have the same length");
    }

    for i in 0..times.len() {
        let possible_time = solver(times[i], distances[i]);
        if result == 0 {
            result = possible_time;
        } else {
            result *= possible_time;
        }
    }

    println!("{}", result);
}
