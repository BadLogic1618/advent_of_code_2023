use std::collections::HashMap;

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


#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_08/test.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 6);
    }
    
    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), 71503);
    // }
}