use std::fs::read_to_string;

use itertools::Itertools;

const VALID_DIGITS: [&'static str; 18] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];

fn first_numeric_char(input: &mut (impl Iterator<Item=char> + Clone)) -> char {
    input.clone().find(|char|char.is_numeric()).expect("could not find digit")
}

fn text_to_calibration_val_simple(input: &str) -> u32 {
    let mut chars = input.chars();
    let mut value = String::new();
    value.push(first_numeric_char(&mut chars));
    value.push(first_numeric_char(&mut chars.rev()));
    
    value.parse::<u32>().expect("could not parse value")
}

fn part_one() -> u32 {
    let file_str = read_to_string("./day_01/src/input.txt").expect("could not find file");

    file_str.lines()
        .map(|val| text_to_calibration_val_simple(val))
        .sum()
}

fn text_to_char_digit(val: &str) -> char {
    if val.len() == 1 { return val.chars().next().unwrap(); }

    match val {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0'
    }
}

fn text_to_calibration_val(input: &str) -> u32 {
    let mut all_digits = VALID_DIGITS.into_iter()
        .map(|val| input.match_indices(val))
        .flatten()
        .sorted_by_key(|val| val.0)
        .map(|val| val.1);

    String::from_iter([
            text_to_char_digit(all_digits.clone().next().unwrap()), 
            text_to_char_digit(all_digits.next_back().unwrap())
        ])
        .parse::<u32>()
        .unwrap()
}

fn part_two() -> u32 {
    let file_str = read_to_string("./day_01/src/input.txt").expect("could nto find file");

    file_str.lines()
        .map(|val| text_to_calibration_val(val))
        .sum()
}

fn main() {
    println!("Answer for part 1: {0}", part_one());
    println!("Answer for part 2: {0}", part_two());
}