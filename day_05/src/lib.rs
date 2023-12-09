use itertools::Itertools;
use rayon::prelude::*;

pub const INPUT: &str = include_str!("../../advent_of_code_2023_input/day_05/input.txt");

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    pub fn inside_inclusive(&self, num: u64) -> bool {
        return num >= self.start && num <= (self.start + self.length)
    }
}

pub fn part_one(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next().unwrap()
        .split_once(": ").unwrap().1
        .split_whitespace()
        .map(|s|s.parse::<u64>().unwrap());

    let transformation_sections: Vec<Vec<(Range, i64)>> = sections
        .map(|v| v.split_once(":\n").unwrap().1)
        .map(section_to_transformation_vec)
        .collect();

    seeds
        .map(|val| apply_transformations(val, &transformation_sections, 0))
        .flatten()
        .min().unwrap()
}

fn apply_transformations(mut val: u64, transformation_sections: &Vec<Vec<(Range, i64)>>, idx: usize) -> Vec<u64> {
    //reached the end of transformation list
    if idx >= transformation_sections.len() { return vec![val] }

    let mut output = Vec::new();
    for (range, offset) in &transformation_sections[idx] {
        if range.inside_inclusive(val) {
            output.push((val as i64 + offset) as u64)
        }
    }

    if output.is_empty() { return apply_transformations(val, &transformation_sections, idx+1); }

    output.iter()
        .map(|v| apply_transformations(*v, &transformation_sections, idx+1))
        .flatten()
        .collect_vec()
}

fn section_to_transformation_vec(lines: &str)-> Vec<(Range, i64)> {
    lines.lines()
        .map(line_to_transformation)
        .collect()
}

fn line_to_transformation(line: &str) -> (Range, i64) {
    let mut nums = line.split_whitespace();
    let destination = nums.next().expect("should not be possible to fail").parse::<i64>().expect("this should always be a number");
    let start = nums.next().expect("should not be possible to fail").parse::<u64>().expect("this should always be a number");
    let length = nums.next().expect("should not be possible to fail").parse::<u64>().expect("this should always be a number");

    (Range{start, length}, destination-start as i64)
}

pub fn part_two(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next().unwrap()
        .split_once(": ").unwrap().1
        .split_whitespace()
        .map(|s|s.parse::<u64>().unwrap())
        .chunks(2);

    let seeds = seeds.into_iter()
        .map(|mut v| {
            let start = v.next().unwrap();
            let len = v.next().unwrap();
            start..(start+len)
        })
        .flatten()
        .collect_vec();

    let transformation_sections: Vec<Vec<(Range, i64)>> = sections
        .map(|v| v.split_once(":\n").unwrap().1)
        .map(section_to_transformation_vec)
        .collect();

    seeds
        .par_iter()
        .map(|val| apply_transformations(*val, &transformation_sections, 0))
        .flatten()
        .min().unwrap()
    //57451710 is wrong
}


#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_05/test.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 35);
    }
    
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 46);
    }
}