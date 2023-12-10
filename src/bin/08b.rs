use clap::Parser;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAP_REGEX: Regex = Regex::new(r"^(?P<point>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)$").expect("invalid regex");
}

use advent_of_code_2023::utils::file_reader::read_lines;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    input_file: String,
}


fn parse_line(ip: &str) -> (&str, (&str, &str)) {
    let parsed_line = MAP_REGEX.captures(&ip).expect("invalid line");

    let point = parsed_line.name("point").unwrap().as_str();
    let left = parsed_line.name("left").unwrap().as_str();
    let right = parsed_line.name("right").unwrap().as_str();

    (point, (left, right))
}


fn find_next_winning_node(map: &HashMap<&str, (&str, &str)>, current_node: &str, directions: &Vec<usize>, directions_len: usize) -> u64 {
    let mut current_point = current_node;
    let mut steps: u64 = 0;

    loop {
        let next_point_map = map.get(current_point).expect("invalid point");
        current_point = if directions[(steps % directions_len as u64) as usize] == 0 {
            next_point_map.0
        } else {
            next_point_map.1
        };

        steps += 1;

        if current_point.ends_with('Z') {
            break;
        }
    }

    steps
}


pub fn lcm(nums: &Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}


fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}


fn main() {
    let args = Args::parse();

    let mut lines = read_lines(args.input_file)
        .expect("failed to read input file");

    let directions = lines
        .next()
        .expect("invalid input file")
        .expect("invalid input file")
        .chars()
        .map(|c| {
            match c {
                'R' => 1,
                'L' => 0,
                _ => panic!("invalid direction"),
            }
        }
        )
        .collect::<Vec<usize>>();

    let directions_len = directions.len();

    lines.next(); // skip empty line
    let map_lines = lines.map(|l| l.expect("invalid line")).collect::<Vec<String>>();
    let map = map_lines.iter().map(|l| parse_line(l)).collect::<HashMap<&str, (&str, &str)>>();

    let starting_points = map.keys().filter(|x| x.ends_with('A')).map(|x| *x).collect::<Vec<&str>>();
    let path_len = starting_points.iter().map(|p| find_next_winning_node(&map, p, &directions, directions_len)).collect::<Vec<u64>>();
    println!("{}", lcm(&path_len));
}
