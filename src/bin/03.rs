use advent_of_code_2023::utils::argparse::read_arg;
use advent_of_code_2023::utils::file_reader::read_lines;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").expect("invalid regex");
    static ref GREAR_REGEX: Regex = Regex::new(r"(\*{1})").expect("invalid regex");
}


fn process_line(prev_line: &String, cur_line: &String, next_line: &String) -> i32 {
    let mut sum = 0;

    let lines = [
        prev_line,
        cur_line,
        next_line,
    ];

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
                    adjucent.push(number.as_str().parse::<i32>().expect("attemted to parse invalid number"));
                }
            }
        }

        if adjucent.len() == 2 {
            sum += adjucent[0] * adjucent[1];
        }
    }

    return sum;
}


fn main() {
    let input_path = read_arg(1, "input path");
    let mut prev_line = String::new();
    let mut cur_line = "".to_string();
    let mut sum = 0;

    if let Ok(lines) = read_lines(input_path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {

                if i > 0 {
                    sum += process_line(&prev_line, &cur_line, &ip);
                }

                prev_line = cur_line;
                cur_line = ip;
            }
        }
    }

    sum += process_line(&prev_line, &cur_line, &String::new());
    println!("sum: {}", sum);
}
