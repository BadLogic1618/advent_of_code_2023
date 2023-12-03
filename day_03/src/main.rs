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
            
        }
    }

    0
}