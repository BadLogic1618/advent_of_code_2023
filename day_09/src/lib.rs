use std::net;

pub const INPUT: &str = include_str!("../../advent_of_code_2023_input/day_09/input.txt");

pub fn part_one(input: &str) -> isize {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<isize>().unwrap()
                })
                .collect::<Vec<isize>>()
        })
        .map(|v| next_val_recursive(&v))
        .sum()
}

fn next_val_recursive(vals: &[isize]) -> isize {
    if vals.iter().all(|v| *v == 0 ) { return 0; }

    let last_val = vals.last().unwrap();
    let next_row: Vec<isize> = vals.windows(2).map(|v| v[1]-v[0]).collect();

    last_val + next_val_recursive(&next_row)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_09/test.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 114);
    }
    
    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST2_INPUT), 6);
    // }
}