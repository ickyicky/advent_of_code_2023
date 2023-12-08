use advent_of_code_2023::utils::argparse::read_arg;
use advent_of_code_2023::utils::file_reader::read_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").expect("invalid regex");
    static ref GREAR_REGEX: Regex = Regex::new(r"(\*{1})").expect("invalid regex");
}

fn process_line(prev_line: &String, cur_line: &String, next_line: &String) -> (i32, i32) {
    let mut gear_ratio = 0;
    let mut part_number_sum = 0;

    let lines = [prev_line, cur_line, next_line];

    let gears = GREAR_REGEX.find_iter(cur_line);

    for gear in gears {
        let mut adjucent = vec![];

        for line in lines.iter() {
            for number in NUMBER_REGEX.find_iter(line) {
                let number_start = match usize::checked_sub(number.start(), 1) {
                    Some(n) => n,
                    None => number.start(),
                };

                if number_start <= gear.start() && gear.start() <= number.end() {
                    adjucent.push(
                        number
                            .as_str()
                            .parse::<i32>()
                            .expect("attemted to parse invalid number"),
                    );
                }
            }
        }

        if adjucent.len() == 2 {
            gear_ratio += adjucent[0] * adjucent[1];
        }
    }

    'outer: for number in NUMBER_REGEX.find_iter(cur_line) {
        let number_parsed = number
            .as_str()
            .parse::<i32>()
            .expect("attemted to parse invalid number");
        let mut start = number.start();
        let end = number.end();

        if let Some(start_minus_one) = usize::checked_sub(start, 1) {
            start = start_minus_one;
            if let Some(prev_char) = cur_line.chars().nth(start) {
                if prev_char != '.' {
                    part_number_sum += number_parsed;
                    continue 'outer;
                }
            }
        }

        if let Some(next_char) = cur_line.chars().nth(end) {
            if next_char != '.' {
                part_number_sum += number_parsed;
                continue 'outer;
            }
        }

        for i in start..end + 1 {
            for line in [prev_line, next_line].iter() {
                if let Some(char) = line.chars().nth(i) {
                    if char != '.' {
                        part_number_sum += number_parsed;
                        continue 'outer;
                    }
                }
            }
        }
    }

    (part_number_sum, gear_ratio)
}

fn main() {
    let input_path = read_arg(1, "input path");
    let mut prev_line = String::new();
    let mut cur_line = "".to_string();

    let mut gear_ratios = 0;
    let mut part_number_sum = 0;

    if let Ok(lines) = read_lines(input_path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if i > 0 {
                    let (cur_part_number_sum, cur_gear_ratios) =
                        process_line(&prev_line, &cur_line, &ip);
                    part_number_sum += cur_part_number_sum;
                    gear_ratios += cur_gear_ratios;
                }

                prev_line = cur_line;
                cur_line = ip;
            }
        }
    }

    let (cur_part_number_sum, cur_gear_ratios) =
        process_line(&prev_line, &cur_line, &String::new());
    part_number_sum += cur_part_number_sum;
    gear_ratios += cur_gear_ratios;

    println!("part_number_sum: {}", part_number_sum);
    println!("gear_ratios: {}", gear_ratios);
}
