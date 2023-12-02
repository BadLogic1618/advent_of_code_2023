use crate::input::*;

pub fn part_one() -> usize {
    INPUT.lines()
        .filter_map(process_line)
        .sum()
}

pub fn process_line(line: &str) -> Option<usize> {
    let (id, record) = line.split_once(": ")?;
    let hands = record.split("; ");
    let colors = hands.flat_map(|hand| hand.split(", ")).map(|color| color.split_once(' ').expect("could not split count and color"));

    for (count, color) in colors {
        let count = count.parse::<usize>().ok()?;
        let threshold = match color {
            "red" => 12,
            "green" => 13,
            "blue" => 14,
            _ => 0,
        };

        if count > threshold {
            return None;
        }
    }

    Some(id.split_at(5).1.parse::<usize>().ok()?)
}