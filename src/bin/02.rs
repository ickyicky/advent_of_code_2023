use advent_of_code_2023::utils::argparse::{read_arg,parse_arg};
use advent_of_code_2023::utils::file_reader::read_lines;
use regex::Regex;
use lazy_static::lazy_static;


struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}


impl Game {
    fn power (&self) -> u32 {
        return self.max_red * self.max_green * self.max_blue;
    }
}


lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"Game (\d+)").expect("invalid regex");
    static ref GREEN_REGEX: Regex = Regex::new(r"(\d+) green").expect("invalid regex");
    static ref BLUE_REGEX: Regex = Regex::new(r"(\d+) blue").expect("invalid regex");
    static ref RED_REGEX: Regex = Regex::new(r"(\d+) red").expect("invalid regex");
}


fn max_matching(line: &String, regex: &Regex) -> u32 {
    let max = regex.captures_iter(&line).map(
        |c| c.extract::<1>().1[0]
        .parse::<u32>()
        .expect("invalid number found with regex")
    ).max().expect("no match found");

    return max;
}

fn process_line(line: &String) -> Game {
    let game_id = max_matching(line, &GAME_REGEX);
    let red = max_matching(line, &RED_REGEX);
    let green = max_matching(line, &GREEN_REGEX);
    let blue = max_matching(line, &BLUE_REGEX);

    return Game {
        id: game_id,
        max_red: red,
        max_green: green,
        max_blue: blue,
    };
}


fn main() {
    let input_path = read_arg(1, "input path");
    let red = parse_arg::<u32>(2, "red");
    let green = parse_arg::<u32>(3, "green");
    let blue = parse_arg::<u32>(4, "blue");

    let mut sum = 0 ;
    let mut total_power = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(ip) = line {
                let game = process_line(&ip);

                if game.max_red <= red && game.max_green <= green && game.max_blue <= blue {
                    sum += game.id;
                    println!("Game {} is a match", game.id);
                }

                total_power += game.power();
            }
        }
    }

    println!("possible games id sum: {}", sum);
    println!("total power: {}", total_power);
}