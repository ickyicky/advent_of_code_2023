use std::io::Error;
use clap::Parser;
use regex::Regex;
use lazy_static::lazy_static;

use advent_of_code_2023::utils::file_reader::read_lines;
use advent_of_code_2023::linspace::{Linspace, LinspaceUnion};


lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").expect("invalid regex");
    static ref MAP_REGEX: Regex = Regex::new(r"\w+-to-(\w+) map:").expect("invalid regex");
}


struct MapSegment {
    target: u64,
    from: u64,
    range: u64,
}


impl MapSegment {
    fn map(&self, ln_union: &mut LinspaceUnion) -> Option::<LinspaceUnion> {
        if let Some(mut ln) = ln_union.extract(self.from, self.range) {
            if self.from > self.target {
                ln.negative_shift(self.from - self.target);
            } else {
                ln.shift(self.target - self.from);
            }
            return Some(ln);
        }

        return None;
    }

    fn parse(line: &String) -> Option::<MapSegment> {
        let numbers = parse_numbers(line);

        if numbers.len() != 3 {
            return None;
        }

        return Some(MapSegment {
            target: numbers[0],
            from: numbers[1],
            range: numbers[2],
        });
    }
}


struct Map {
    segments: Vec::<MapSegment>,
    name: String,
}


impl Map {
    fn map(&self, ln_union: &mut LinspaceUnion) {
        let mut to_extend = vec![];

        for segment in self.segments.iter() {
            if let Some(ln) = segment.map(ln_union) {
                to_extend.push(ln);
            }
        }

        for ln in to_extend {
            ln_union.extend(&ln);
        }
    }

    fn parse(lines_iterable: &mut dyn Iterator<Item=Result<String, Error>>) -> Option::<Map> {
        let mut name: Option::<String> = None;

        while let Some(line) = lines_iterable.next() {
            if let Ok(ip) = line {
                let caps = MAP_REGEX.captures(&ip);

                if let Some(caps) = caps {
                    name = Some(caps[1].to_string());
                    break;
                }
            }
        }

        if name.is_none() {
            return None;
        }

        let mut segments = vec![];

        while let Some(line) = lines_iterable.next() {
            if let Ok(ip) = line {
                if let Some(segment) = MapSegment::parse(&ip) {
                    segments.push(segment);
                } else {
                    break;
                }
            }
        }

        return Some(Map {
            segments,
            name: name.unwrap(),
        });
    }
}


fn parse_numbers(line: &String) -> Vec::<u64> {
    let mut numbers = vec![];

    for cap in NUMBER_REGEX.captures_iter(line) {
        numbers.push(cap[1].parse::<u64>().unwrap());
    }

    numbers
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index=1)]
    input_file: String,

    #[arg(short, long, default_value_t = false)]
    ranges: bool,
}


fn main() {
    let args = Args::parse();

    let mut lines = read_lines(args.input_file).expect("failed to read input file");

    let numbers = parse_numbers(
        &lines
        .next()
        .expect("first line should contain numbers")
        .expect("first line should contain numbers")
    );

    let mut ln_union = LinspaceUnion::new();

    if args.ranges {
        let mut num_iterator = numbers.iter();

        while let Some(start) = num_iterator.next() {
            if let Some(len) = num_iterator.next() {
                ln_union.push(Linspace::new(*start, *len));
            } else {
                break;
            }
        }
    } else {
        for number in numbers.iter() {
            ln_union.push(Linspace::new(*number, 1));
        }
    }

    while let Some(map) = Map::parse(&mut lines) {
        map.map(&mut ln_union);
    }

    println!("min: {}", ln_union.min());
}
