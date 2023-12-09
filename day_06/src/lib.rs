pub const INPUT: &str = include_str!("../../advent_of_code_2023_input/day_06/input.txt");

pub fn part_one(input: &str) -> isize {
    let mut lines = input.lines();
    
    let times = lines.next().unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<isize>().ok());

    let distances = lines.next().unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<isize>().ok());

    let pairs = times.zip(distances);

    pairs.map(process_pair)
        .reduce(|a, b|  a*b)
        .unwrap()
}

fn process_pair(pair: (isize, isize)) -> isize {
    let (time, distance) = pair;

    let a = -1.0;
    let b = time as f64;
    let c = (-distance) as f64;

    let intersect = (-b + (b.powi(2) - 4.0*a*c).sqrt())/2.0*a;
    let mut val = (time as f64 - intersect).trunc() - intersect.trunc();
    if intersect.fract() == 0.0 { val -= 1.0;}
    val.round() as isize
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_06/test.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 288);
    }
    
    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), 46);
    // }
}