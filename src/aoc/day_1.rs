/* ----- Day 1 ----- */
use crate::aoc;

pub fn aoc_1_1() -> u32 {
    let data = parse_data();
    let mut total = 0;
    for module in data.iter() {
        total += calculate_fuel(*module);
    }
    total
}

pub fn aoc_1_2() -> u32 {
    let data = parse_data();
    let mut total = 0;
    for module in data.iter() {
        let mut module_mass = 0;
        let mut fuel = calculate_fuel(*module);
        while fuel > 0 {
            module_mass += fuel;
            fuel = calculate_fuel(fuel);
        }
        total += module_mass;
    }
    total
}

fn parse_data() -> Vec<u32> {
    let string = aoc::util::read_input(1).unwrap();
    string.split("\n").map(|i| i.parse().unwrap()).collect()
}

fn calculate_fuel(mass: u32) -> u32 {
    (mass / 3).checked_sub(2).unwrap_or(0)
}
