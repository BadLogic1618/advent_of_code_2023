const INPUT: &'static str = include_str!("input.txt");

// including first and last characters of orignal str in replacement because some values in input have overlapping start/end characters
const REPLACEMENT_PAIRS: [(&'static str, &'static str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn main() {
    println!("Answer for part 1: {0}", part_one(INPUT));
    println!("Answer for part 2: {0}", part_two(INPUT));
}

fn part_one(input: &str) -> u32 {
    input.lines()
        .map(|val| line_to_calibration_val(val))
        .sum()
}

fn line_to_calibration_val(input: &str) -> u32 {
    let output = format!(
        "{}{}",
        first_numeric_char(&mut input.chars()),
        first_numeric_char(&mut input.chars().rev())
    );
    
    output.parse::<u32>()
        .expect("could not parse value")
}

fn first_numeric_char(input: &mut impl Iterator<Item=char>) -> char {
    input.find(|char| char.is_numeric())
        .expect("could not find digit")
}

fn part_two(input: &str) -> u32 {
    let input = preprocess_input(input);
    part_one(&input)
}

fn preprocess_input(input: &str) -> String {
    let mut input = input.to_owned();
    for pair in REPLACEMENT_PAIRS {
        input = input.replace(pair.0, pair.1);
    }
    input
}