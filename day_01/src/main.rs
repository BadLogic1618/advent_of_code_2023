pub const INPUT: &'static str = include_str!("../../advent_of_code_2023_input/day_01/input.txt");

// including first and last characters of orignal str in replacement because some values in input have overlapping start/end characters
const REPLACEMENT_PAIRS: [(&[u8], u8); 9] = [
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

fn main() {
    println!("Answer for part 1: {0}", part_one(INPUT));
    println!("Answer for part 2: {0}", part_two(INPUT));
}

pub fn part_one(input: &str) -> u32 {
    input.lines()
        .map(line_to_calibration_val)
        .sum()
}

fn line_to_calibration_val(input: &str) -> u32 {
    let input = input.as_bytes();

    let first_digit = input.iter().find(|byte| byte.is_ascii_digit()).unwrap() - b'0';
    let second_digit = input.iter().rfind(|byte| byte.is_ascii_digit()).unwrap() - b'0';
    first_digit as u32 * 10 + second_digit as u32
}

pub fn part_two(input: &str) -> u32 {
    input.lines()
        .map(line_to_calibration_val_words)
        .sum()
}

fn line_to_calibration_val_words(line: &str) -> u32 {
    let input = line.as_bytes();
    let mut n = 0;

    'outer: for i in 0..input.len() {
        if input[i].is_ascii_digit() {
            n += (input[i] - b'0') * 10;
            break;
        }

        for val in REPLACEMENT_PAIRS {
            if input[i..].starts_with(val.0){
                n += val.1 * 10;
                break 'outer;
            }
        }
    }

    'outer: for i in (0..input.len()).rev(){
        if input[i].is_ascii_digit() {
            n += input[i] - b'0';
            break;
        }
        for val in REPLACEMENT_PAIRS {
            if input[..=i].ends_with(val.0){
                n += val.1;
                break 'outer;
            }
        }
    }

    n as u32
}