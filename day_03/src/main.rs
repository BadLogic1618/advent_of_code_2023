const INPUT: &str = include_str!("input.txt");
const BOUND: usize = 140;

***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***
***REMOVED***

fn main() {
    println!("part one: {}", part_one(INPUT));
    println!("part one: {}", part_two(INPUT));
}

pub fn part_one(input: &str) -> usize {
    let input: Vec<&[u8]> = input.lines().map(|val| val.as_bytes()).collect();
    let mut sum = 0;

    for row in 0..input.len() {
        let mut col= 0;
        while col < BOUND {
            if !input[row][col].is_ascii_digit() { 
                col += 1;
                continue; 
            }

            let num_length = get_num_length(row, col, &input);
            if scan_range_for_symbol(row, col, num_length, &input) {
                for i in (0..num_length).rev() {
                    let num = (input[row][col] - b'0') as usize;
                    sum += 10_usize.pow(i as u32) * num;
                    col += 1;
                }
            }
            col += 1;
        }
    }

    sum
}

fn get_num_length(row: usize, col: usize, input: &[&[u8]]) -> usize {
    let mut count = 1;
    while col+count < BOUND && input[row][col+count].is_ascii_digit() {
        count += 1;
    }
    count
}

fn scan_range_for_symbol(row: usize, col: usize, len: usize, input: &[&[u8]]) -> bool {
    if row > 0 {
        for i in col.max(1)-1..(col+1+len).min(BOUND) {
            if char_is_symbol(input[row-1][i]) { return true; }
        }
    }

    if col > 0 && char_is_symbol(input[row][col-1]) { return true; }
    if col+len < BOUND && char_is_symbol(input[row][col+len]) { return true; }

    if row < BOUND-1 {
        for i in col.max(1)-1..(col+1+len).min(BOUND) {
            if char_is_symbol(input[(row+1).min(BOUND-1)][i]) { return true; }
        }
    }


    false
}

fn char_is_symbol(char: u8) -> bool {
    char != b'.' && !char.is_ascii_digit()
}

fn part_two(input: &str) -> usize {
    let input: Vec<&[u8]> = input.lines().map(|val| val.as_bytes()).collect();
    let mut sum = 0;

    for row in 0..BOUND {
        for col in 0..BOUND {
            if input[row][col] != b'*' { continue; }
            if let Some(locations) = get_valid_neighbouring_part_nums(row, col, &input) {
                let num_1 = scan_for_num_at(locations[0].0, locations[0].1, &input);
                let num_2 = scan_for_num_at(locations[1].0, locations[1].1, &input);
                sum += num_1 * num_2;
            }
        }
    }

    sum
}

fn get_valid_neighbouring_part_nums(row: usize, col: usize, input: &[&[u8]]) -> Option<Vec<(usize, usize)>> {
    
    let mut part_num_locations = scan_row_for_part_num(row-1, col, input);

    if col > 0 && input[row][col-1].is_ascii_digit() {
        part_num_locations.push((row, col-1));
    }

    if col < BOUND-1 && input[row][col+1].is_ascii_digit() {
        part_num_locations.push((row, col+1))
    }

    part_num_locations.append(&mut scan_row_for_part_num(row+1, col, input));

    match part_num_locations.len() {
        2 => Some(part_num_locations),
        _ => None,
    }
}

fn scan_row_for_part_num(row: usize, col: usize, input: &[&[u8]]) -> Vec<(usize, usize)> {
    let mut locations = Vec::new();

    if row > BOUND-1 { return locations; }

    let mut last_char = b'.';
    for i in col.max(1)-1..=(col+1).min(BOUND) {
        let next_char = input[row][i];
        if next_char.is_ascii_digit() && !last_char.is_ascii_digit() {
            locations.push((row, i))
        }
        last_char = next_char;
    }

    locations
}

fn scan_for_num_at(row: usize, col:usize, input: &[&[u8]]) -> usize {
    let mut sum = 0;
    
    let mut i = 0;
    while i <= col && input[row][col-i].is_ascii_digit() {
        sum += 10_usize.pow(i as u32) * (input[row][col-i] - b'0') as usize;
        i += 1;
    }

    i = 1;
    while (col+i) < BOUND && input[row][col+i].is_ascii_digit() {
        sum *= 10;
        sum += (input[row][col+i] - b'0') as usize;
        i += 1;
    }

    sum
}