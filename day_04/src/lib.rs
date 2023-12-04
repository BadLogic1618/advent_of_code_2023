use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");

pub fn part_one(input: &str) -> usize {
    input.lines()
        .map(process_line)
        .sum()
}

fn process_line(line: &str) -> usize {
    let count = get_matching_num_count(line);

    match count {
        0 => 0,
        _ => 1 << count - 1,
    }
}

fn get_matching_num_count(line: &str) -> usize {
    let (winning_nums, actual_nums) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let winning_nums: HashSet<&str> = winning_nums.split_whitespace().collect();
    let actual_nums: HashSet<&str> = actual_nums.split_whitespace().collect();

    winning_nums.intersection(&actual_nums).count()
}

pub fn part_two(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut card_counts = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let count = get_matching_num_count(line);
        for j in (i+1)..(i+1+count).min(lines.len()) {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 13);
    }
    
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 30);
    }
}