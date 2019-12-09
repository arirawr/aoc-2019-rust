/* ----- Day 4 ----- */
use crate::aoc;

struct Range {
    min: u32,
    max: u32
}

pub fn aoc_4_1() -> u32 {
    let range = parse_data();
    let mut results = 0;
    for i in range.min..range.max {
        if check_password(number_to_vec(i)) {
            results += 1;
        }
    }
    results
}

pub fn aoc_4_2() -> u32 {
    let range = parse_data();
    let mut results = 0;
    for i in range.min..range.max {
        if check_password_2(number_to_vec(i)) {
            results += 1;
        }
    }
    results
}

fn parse_data() -> Range {
    let string = aoc::util::read_input(4).unwrap();
    let split: Vec<&str> = string.split("-").collect();
    Range {
        min: split[0].parse::<u32>().unwrap(),
        max: split[1].parse::<u32>().unwrap()
    }
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn check_password(password: Vec<u32>) -> bool {
    let mut doubles = false;
    let mut prev = 0;
    for digit in password {
        if digit < prev {
            return false
        }
        if digit == prev {
            doubles = true;
        }
        prev = digit;
    }
    
    if doubles {
        return true
    }
    false
}

fn check_password_2(password: Vec<u32>) -> bool {
    let mut doubles = false;
    let mut prev = 0;
    let mut streak = 1;
    for digit in password {
        if digit < prev {
            return false
        }
        if digit == prev {
            streak += 1;
        } else {
            if streak == 2 {
                doubles = true;
            }
            // Reset the streak
            streak = 1;
        }
        prev = digit;
    }
    
    if doubles || streak == 2 {
        return true
    }
    false
}
