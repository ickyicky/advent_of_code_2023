use clap::Parser;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAP_REGEX: Regex = Regex::new(r"^(?P<point>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)$").expect("invalid regex");
}

use advent_of_code_2023::utils::file_reader::read_lines;

const START: char = 'A';
const END: char = 'Z';


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

    let mut steps = 0;
    let mut all_found = false;

    let mut current_points = map.keys().filter(|x| x.ends_with(START)).map(|x| *x).collect::<Vec<&str>>();

    while !all_found {
        all_found = true;
        let mut new_points = vec![];

        for point in current_points.iter() {
            let next_point_map = map.get(point).expect("invalid point");
            let next_point = if directions[steps % directions_len] == 0 {
                next_point_map.0
            } else {
                next_point_map.1
            };
            all_found = all_found && next_point.ends_with(END);
            new_points.push(next_point);
        }

        steps += 1;
        current_points = new_points;
        println!("steps: {}, points: {:?}", steps, current_points);
    }

    println!("steps: {}", steps);
}
