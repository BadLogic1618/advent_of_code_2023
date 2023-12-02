use std::collections::HashMap;

use crate::input::*;

pub fn part_two() -> usize {
    INPUT.lines()
        .map(process_line)
        .sum()
}

pub fn process_line(line: &str) -> usize {
    let (_, record) = line.split_once(": ").expect("could not split game record and id");
    let hands = record.split("; ");

    let colors = hands.flat_map(|hand| hand.split(", "))
        .map(|color| color.split_once(' ').expect("could not split count and color"));

    let mut color_map: HashMap<&str, usize> = HashMap::with_capacity(3);

    for (count, color) in colors {
        let count = count.parse::<usize>().expect("could not parse color count");
        
        color_map.entry(color)
            .and_modify(|v| *v = (*v).max(count))
            .or_insert(count);
    }

    color_map.values()
        .cloned()
        .reduce(|acc, e| acc*e)
        .expect("could not reduce iterator")
}