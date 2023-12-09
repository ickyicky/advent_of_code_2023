use clap::Parser;

use advent_of_code_2023::utils::file_reader::read_lines;

const JOKER: char = '🃏';
const CARDS: &[char] = &[
    JOKER, '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS_LEN: usize = CARDS.len();
const HAND_LEN: usize = 5;

#[derive(Debug)]
struct Hand {
    value: u64,
    bid: u64,
}

impl Hand {
    fn new(value: u64, bid: u64) -> Hand {
        Hand { value, bid }
    }

    fn parse_cards(cards: &str) -> u64 {
        let mut value = 0;
        let mut cards_amounts = [0; CARDS_LEN];

        for (i, card) in cards.chars().enumerate() {
            for (j, c) in CARDS.iter().enumerate() {
                if *c == card {
                    value += u64::pow(CARDS_LEN as u64, (HAND_LEN - i - 1) as u32) * (j + 1) as u64;
                    cards_amounts[j] += 1;
                    break;
                }
            }
        }

        let jokers = cards_amounts[0];
        cards_amounts[0] = 0;

        cards_amounts.sort();
        cards_amounts.reverse();
        cards_amounts[0] += jokers;
        let mut hand_composition = 0;

        if cards_amounts[0] == 5 {
            hand_composition = 6;
        } else if cards_amounts[0] == 4 {
            hand_composition = 5;
        } else if cards_amounts[0] == 3 && cards_amounts[1] == 2 {
            hand_composition = 4;
        } else if cards_amounts[0] == 3 {
            hand_composition = 3;
        } else if cards_amounts[0] == 2 && cards_amounts[1] == 2 {
            hand_composition = 2;
        } else if cards_amounts[0] == 2 {
            hand_composition = 1;
        }

        value += hand_composition * u64::pow(CARDS_LEN as u64, HAND_LEN as u32);

        value
    }

    fn parse_line(line: &str) -> Hand {
        let (cards, bid) = line.split_once(' ').expect("invalid line format");
        let bid = bid.parse::<u64>().expect("invalid bid");
        Hand::new(Hand::parse_cards(cards), bid)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    input_file: String,

    #[arg(short, long, default_value_t = false)]
    with_jokers: bool,
}

fn main() {
    let args = Args::parse();

    let lines = read_lines(args.input_file).expect("failed to read input file");
    let mut parsed_lines = lines
        .map(|l| l.expect("invalid line"))
        .collect::<Vec<String>>();

    if args.with_jokers {
        parsed_lines = parsed_lines
            .iter()
            .map(|l| l.replace("J", "🃏"))
            .collect::<Vec<String>>();
    }

    let mut cards = parsed_lines
        .iter()
        .map(|l| Hand::parse_line(l))
        .collect::<Vec<Hand>>();

    cards.sort_by_key(|c| c.value);

    let mut points = 0;

    for (i, card) in cards.iter().enumerate() {
        points += card.bid * (i + 1) as u64;
    }

    println!("points: {}", points);
}
