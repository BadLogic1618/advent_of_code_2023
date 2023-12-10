use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("../../advent_of_code_2023_input/day_08/input.txt");

pub fn part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    //remove blank line 
    lines.next();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        let key = &line[0..3];
        let values = (&line[7..10], &line[12..15]);
        nodes.insert(key, values);
    }

    let mut current_node = "AAA";
    let mut step_count = 0;

    for instruction in instructions.chars().cycle() {
        if current_node == "ZZZ" { return step_count; }

        step_count += 1;
        let options = nodes[current_node];
        current_node = match instruction {
            'L' => options.0,
            'R' => options.1,
            _ => unreachable!()
        }
    }

    unreachable!()
}

pub fn part_two(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    //remove blank line 
    lines.next();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_nodes = HashSet::new();

    for line in lines {
        let key = &line[0..3];
        if key.ends_with("A") {
            current_nodes.insert(key);
        }
        let values = (&line[7..10], &line[12..15]);
        nodes.insert(key, values);
    }

    let mut step_count = 0;
    let mut cycles = Vec::new();

    for instruction in instructions.chars().cycle() {
        if current_nodes.is_empty() { break; }
        step_count +=1;

        let mut remove = HashSet::new();
        let mut add = HashSet::new();

        for node in &current_nodes {
            let options = nodes[node];
            let next = match instruction {
                'L' => options.0,
                'R' => options.1,
                _ => unreachable!()
            };
            remove.insert(*node);

            if !next.ends_with('Z') {
                add.insert(next);
            }
            else {
                cycles.push(step_count);
            }
        }
        current_nodes = &current_nodes - &remove;
        current_nodes.extend(add);
    }

    let cycles_init = cycles.clone();

    loop {
        if cycles.windows(2).all(|v| v[0] == v[1]) {
            return cycles[0];
        }

        let mut lowest = (usize::MAX, usize::MAX);
        for i in 0..cycles.len() {
            if cycles[i] < lowest.0 { 
                lowest.0 = cycles[i];
                lowest.1 = i;
            }
        }
        cycles[lowest.1] += cycles_init[lowest.1];
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_08/test.txt");
    const TEST2_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_08/test2.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 6);
    }
    
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST2_INPUT), 6);
    }
}