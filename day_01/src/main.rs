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
    println!("Answer for part 1: {0}", part_one());
    println!("Answer for part 2: {0}", part_two());
}

fn part_one() -> u32 {
    INPUT.lines()
        .map(|val| line_to_calibration_val(val))
        .sum()
}

fn line_to_calibration_val(input: &str) -> u32 {
    let mut input = input.chars();
    let output = format!(
        "{}{}",
        first_numeric_char(&mut input),
        first_numeric_char(&mut input.rev())
    );
    
    output.parse::<u32>().expect("could not parse value")
}

fn first_numeric_char(input: &mut (impl Iterator<Item=char> + Clone)) -> char {
    input.clone()
        .find(|char| char.is_numeric())
        .expect("could not find digit")
}

fn part_two() -> u32 {
    INPUT.lines()
        .map(|val| preprocess_line(val))
        .map(|val| line_to_calibration_val(&val))
        .sum()
}

fn preprocess_line(line: &str) -> String {
    let mut line = line.to_owned();
    for pair in REPLACEMENT_PAIRS {
        line = line.replace(pair.0, pair.1);
    }
    line
}