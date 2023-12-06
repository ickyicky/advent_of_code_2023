use advent_of_code_2023::utils::argparse::read_arg;
use advent_of_code_2023::utils::file_reader::read_lines;


fn process_line(
    line: String,
    digits_map: &[(&str, u32)]
) -> Option<u32> {
    let mut temp = line;
    let mut first_num = None;
    let mut last_num = None;

    // search first digit or word in the line
    while temp.len() > 0 {
        let first_char = temp.chars().nth(0).unwrap();
        if first_char.is_digit(10) {
            first_num = Some(first_char.to_digit(10).unwrap());
            break
        } else {
            let mut found_match = false;

            for (word, num) in digits_map {
                if temp.starts_with(word) {
                    first_num = Some(*num);
                    found_match = true;
                    break
                }
            }

            if found_match {
                break
            }
        }

        temp = temp[1..].to_string();
    }

    // search last digit or word in the line
    while temp.len() > 0 {
        let last_char = temp.chars().nth(temp.len() - 1).unwrap();
        if last_char.is_digit(10) {
            last_num = Some(last_char.to_digit(10).unwrap());
            break
        } else {
            let mut found_match = false;

            for (word, num) in digits_map {
                if temp.ends_with(word) {
                    last_num = Some(*num);
                    found_match = true;
                    break
                }
            }

            if found_match {
                break
            }
        }

        temp = temp[..temp.len() - 1].to_string();
    }

    // either both are found or not
    if let (Some(first), Some(last)) = (first_num, last_num) {
        return Some(first * 10 + last);
    }

    None
}


fn main() {
    let input_path = read_arg(1, "input path");

    let digits_map = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0 ;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(ip) = line {
                if let Some(num) = process_line(ip, &digits_map) {
                    sum += num;
                }
            }
        }
    }

    println!("{}", sum);
}
