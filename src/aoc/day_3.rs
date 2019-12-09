/* ----- Day 3 ----- */

use std::fs;
use std::io::{Error};

/* STRUCTS */
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Undefined
}

#[derive(Debug)]
struct DirectionInstruction {
    direction: Direction,
    distance: u32
}

/* MAIN */
pub fn aoc_3_1() {
    println!("{:#?}",input_3().unwrap());
}

/* HELPERS */
fn input_3() -> Result<Vec<Vec<DirectionInstruction>>, Error> {
    let string = fs::read_to_string("inputs/3.txt").unwrap();
    
    let arr = string.split("\n");
    println!("{:#?}", arr);

    let mut data = Vec::new();
    for line in string.split("\n") {
        let directions = strings_to_directions(line.split(",").collect());
        data.push(directions);
    }

    Ok(data)
}

fn strings_to_directions(input: Vec<&str>) -> Vec<DirectionInstruction> {
    let mut direction_set = Vec::new();
    for i in input {
        // Split strings (ie. "R324") and make a DirectionInstruction (ie. { "Right", 324 })
        let substrings = i.split_at(1);
        direction_set.push(DirectionInstruction {
            direction: match substrings.0 {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => Direction::Undefined
            },
            distance: substrings.1.parse().unwrap()
        });
    }
    direction_set
}