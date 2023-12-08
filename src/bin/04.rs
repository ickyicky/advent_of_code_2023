use advent_of_code_2023::utils::argparse::read_arg;
use advent_of_code_2023::utils::file_reader::read_lines;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").expect("invalid regex");
    static ref GAME_CARD_REGEX: Regex = Regex::new(r"^Card +\d+:(?P<winning>[\d ]+) \| (?P<have>[\d ]+)$").expect("invalid regex");
}


fn process_line(line: &String) -> Option::<usize> {
    let parsed_line = GAME_CARD_REGEX.captures(line).expect("invalid line");

    // using hash sets for funsies, i know with small number of elements
    // it's not worth it, but wanna learn it
    let winning = NUMBER_REGEX.captures_iter(
        parsed_line.name("winning").unwrap().as_str()
    ).map(
        |c| c.extract::<1>().1[0]
        .parse::<u32>()
        .expect("invalid number found with regex")
    ).collect::<HashSet<u32>>();

    let have = NUMBER_REGEX.captures_iter(
        parsed_line.name("have").unwrap().as_str()
    ).map(
        |c| c.extract::<1>().1[0]
        .parse::<u32>()
        .expect("invalid number found with regex")
    ).collect::<HashSet<u32>>();

    let common = winning.intersection(&have).count();

    if common == 0 {
        return None;
    }

    Some(common)
}


fn main() {
    let input_path = read_arg(1, "input path");
    let mut points = 0;

    let mut total_cards = 0;
    let mut to_check = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for (i, line) in lines.enumerate() {
            let amount_of_current_card = to_check.iter().filter(|&x| *x == i).count() + 1;
            total_cards += amount_of_current_card;

            to_check.retain(|&x| x != i);

            if let Ok(ip) = line {
                if let Some(common) = process_line(&ip) {
                    points += 2_i32.pow(common as u32 - 1);

                    for j in 1..common+1 {
                        for _ in 0..amount_of_current_card {
                            to_check.push(i + j);
                        }
                    }
                }
            }
        }
    }

    println!("total cards: {}", total_cards);
    println!("points: {}", points);
}
