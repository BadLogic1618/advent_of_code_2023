use itertools::Itertools;

pub const INPUT: &str = include_str!("../../advent_of_code_2023_input/day_05/input.txt");

#[derive(PartialEq)]
enum RangeIntersection {
    NoIntersection,
    IntersectsLower,
    IntersectsUpper,
    Encapsulates,
    IsEncapsulated,
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    length: usize,
}

impl Range {
    pub fn inside_inclusive(&self, num: usize) -> bool {
        return num >= self.start && num <= (self.start + self.length)
    }

    pub fn min(&self) -> usize {
        self.start
    }

    pub fn max(&self) -> usize {
        self.start + self.length - 1
    }

    pub fn offset(&mut self, offset: isize)  {
        self.start = (self.start as isize + offset) as usize
    }

    pub fn split_at(self, val: usize) -> (Option<Range>, Option<Range>) {
        if val < self.min() { return (None, Some(self)); }
        if val > self.max() { return (Some(self), None); }

        let lower_range = Range{start: self.start, length: 1 + val - self.start};
        let upper_range = Range{start: val+1, length: self.max() - val };

        (Some(lower_range), Some(upper_range))
    }

    pub fn intersects_range(&self, other: &Range) -> RangeIntersection {
        match other {
            other if other.min() > self.max() || other.max() < self.min() => RangeIntersection::NoIntersection,
            other if other.min() >= self.min() && other.max() <= self.max() => RangeIntersection::Encapsulates,
            other if other.min() < self.min() && other.max() > self.max() => RangeIntersection::IsEncapsulated,
            other if other.min() >= self.min() => RangeIntersection::IntersectsLower,
            other if other.max() <= self.max() => RangeIntersection::IntersectsUpper,
            _ => unreachable!()
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next().unwrap()
        .split_once(": ").unwrap().1
        .split_whitespace()
        .map(|s|s.parse::<usize>().unwrap());

    let transformation_sections: Vec<Vec<(Range, isize)>> = sections
        .map(|v| v.split_once(":\n").unwrap().1)
        .map(section_to_transformation_vec)
        .collect();

    seeds
        .map(|val| apply_transformations(val, &transformation_sections))
        .min().unwrap()
}

fn apply_transformations(mut val: usize, transformation_sections: &Vec<Vec<(Range, isize)>>) -> usize {
    for section in transformation_sections {
        if let Some((_, offset)) = section.iter().find(|(range, _)| range.inside_inclusive(val)) {
            val = (val as isize + offset) as usize;
        }
    }
    val
}

fn section_to_transformation_vec(lines: &str)-> Vec<(Range, isize)> {
    lines.lines()
        .map(line_to_transformation)
        .collect()
}

fn line_to_transformation(line: &str) -> (Range, isize) {
    let mut nums = line.split_whitespace();
    let destination = nums.next().expect("should not be possible to fail").parse::<usize>().expect("this should always be a number") as isize;
    let start = nums.next().expect("should not be possible to fail").parse::<usize>().expect("this should always be a number");
    let length = nums.next().expect("should not be possible to fail").parse::<usize>().expect("this should always be a number");

    (Range{start, length}, destination-start as isize)
}

pub fn part_two(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    let seed_ranges = seed_line_to_ranges(sections.next().unwrap());

    let transformation_sections: Vec<Vec<(Range, isize)>> = sections
        .map(|v| v.split_once(":\n").unwrap().1)
        .map(section_to_transformation_vec)
        .collect();

    let seed_ranges = seed_ranges.iter()
        .map(|range| recursive_transform(*range, (&transformation_sections, 0)))
        .flatten()
        .collect_vec();
        // .map(|range|{
        //     range.min()
        // })
        // .min().unwrap()

    let mut lowest = usize::MAX;
    for range in seed_ranges {
        if range.min() < lowest {
            lowest = range.min();
        }
    }
    lowest
}

fn recursive_transform<'a>(mut seeds: Range, sections: (&Vec<Vec<(Range, isize)>>, usize)) -> Vec<Range> {
    if sections.1 >= sections.0.len() { return vec![seeds]; }
    let mut output = Vec::new();

    for entry in &sections.0[sections.1] {
        let (range, offset) = entry;
        match range.intersects_range(&seeds) {
            RangeIntersection::NoIntersection => output.append(&mut recursive_transform(seeds, (sections.0, sections.1 + 1))),
            RangeIntersection::IntersectsLower => {
                let ranges = seeds.split_at(range.max());
                let mut lower = ranges.0.unwrap();
                lower.offset(*offset);
                let upper = ranges.1.unwrap();
                output.append(&mut recursive_transform(lower, (sections.0, sections.1 + 1)));
                output.append(&mut recursive_transform(upper, (sections.0, sections.1 + 1)));
            },
            RangeIntersection::IntersectsUpper => {
                let ranges = seeds.split_at(range.min()-1);
                let lower = ranges.0.unwrap();
                let mut upper = ranges.1.unwrap();
                upper.offset(*offset);
                output.append(&mut recursive_transform(lower, (sections.0, sections.1 + 1)));
                output.append(&mut recursive_transform(upper, (sections.0, sections.1 + 1)));
            },
            RangeIntersection::Encapsulates => {
                seeds.offset(*offset);
                output.append(&mut recursive_transform(seeds, (sections.0, sections.1 + 1)));
            },
            RangeIntersection::IsEncapsulated => {
                let lower_ranges = seeds.split_at(range.min()-1);
                let lower = lower_ranges.0.unwrap();
                let upper_ranges = lower_ranges.1.unwrap().split_at(range.max());
                let mut middle = upper_ranges.0.unwrap();
                middle.offset(*offset);
                let upper = upper_ranges.1.unwrap();
                output.append(&mut recursive_transform(lower, (sections.0, sections.1 + 1)));
                output.append(&mut recursive_transform(middle, (sections.0, sections.1 + 1)));
                output.append(&mut recursive_transform(upper, (sections.0, sections.1 + 1)));
            },
        }
    }

    output
}

fn seed_line_to_ranges(line: &str) -> Vec<Range> {
    let seeds = line.split_once(": ").unwrap().1
        .split_whitespace()
        .map(|s|s.parse::<usize>().unwrap())
        .chunks(2);

    seeds.into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let length = chunk.next().unwrap();
            Range{start, length}
        }).collect()
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