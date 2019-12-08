fn main() {
    println!("Advent of Code!");

    println!("\nDay 1");
    println!("Part 1: {}", aoc_1_1(AOC_1_INPUT));
    println!("Part 2: {}", aoc_1_2(AOC_1_INPUT));

    println!("\nDay 2");
    println!("Part 1: {}", aoc_2_1(AOC_2_INPUT));
    println!("Part 2: {}", aoc_2_2(AOC_2_INPUT));
}

/* Day 1 */

const AOC_1_INPUT: [u32; 100] = [
    149579, 95962, 97899, 149552, 65085, 111896, 127591, 115128, 64630, 120430, 81173, 136775,
    137806, 132042, 65902, 87894, 97174, 126829, 88716, 85284, 61178, 106423, 89821, 51123, 85350,
    53905, 74259, 59710, 80358, 111938, 129027, 144036, 68717, 69382, 64163, 65114, 58548, 74559,
    142855, 115617, 107847, 133264, 111657, 125402, 129254, 67275, 120955, 110940, 139146, 96810,
    147085, 103471, 89560, 111940, 120332, 55717, 73498, 133817, 102095, 57518, 57725, 58673,
    84918, 143693, 149361, 74432, 51048, 99136, 128220, 141591, 79477, 116798, 93622, 113316,
    143888, 143155, 57861, 112833, 70928, 116310, 126836, 93835, 101281, 116599, 107776, 138215,
    107034, 74826, 73372, 127785, 105051, 124720, 147682, 97320, 74957, 113446, 101566, 96278,
    144766, 55755,
];

fn aoc_1_1(input: [u32; 100]) -> u32 {
    let mut total = 0;
    for module in input.iter() {
        total += calculate_fuel(*module);
    }
    total
}

fn aoc_1_2(input: [u32; 100]) -> u32 {
    let mut total = 0;
    for module in input.iter() {
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

fn calculate_fuel(mass: u32) -> u32 {
    (mass/3).checked_sub(2).unwrap_or(0)
}

/* Day 2 */

const AOC_2_INPUT: [u32; 148] = [1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,19,5,23,2,23,6,27,1,
27,5,31,2,6,31,35,1,5,35,39,2,39,9,43,1,43,5,47,1,10,47,51,1,51,6,55,1,55,10,59,1,59,6,63,2,13,63,
67,1,9,67,71,2,6,71,75,1,5,75,79,1,9,79,83,2,6,83,87,1,5,87,91,2,6,91,95,2,95,9,99,1,99,6,103,1,
103,13,107,2,13,107,111,2,111,10,115,1,115,6,119,1,6,119,123,2,6,123,127,1,127,5,131,2,131,6,135,1,
135,2,139,1,139,9,0,99,2,14,0];

enum OpcodeType {
    Add = 1,
    Multiply = 2,
    Halt = 99
}

struct Opcode {
    opcode: OpcodeType,
    inputs: [u32; 2],
    output: u32,
}

fn aoc_2_1(input: [u32;148]) -> u32 {
    calculate_0(input)
}

fn aoc_2_2(input: [u32;148]) -> u32 {
    for a in 0..100 {
        for b in 0..100 {
            let mut data = input;
            data[1] = a;
            data[2] = b;
            if calculate_0(data) == 19690720 {
                return 100 * a + b;
            }
        }
    }
    0
}

fn input_to_opcode(input: &[u32;148]) -> Vec<Opcode> {
    let mut codes: Vec<Opcode> = Vec::new();
    for chunk in input.chunks(4) {
        let code = Opcode {
            opcode: match chunk[0] {
                1 => OpcodeType::Add,
                2 => OpcodeType::Multiply,
                _ => OpcodeType::Halt
            },
            inputs: [chunk[1], chunk[2]],
            output: chunk[3]
        };
        codes.push(code);    
    }
    codes
}

fn calculate_0(input: [u32;148]) -> u32 {
    let codes = input_to_opcode(&input);
    let mut data = input;
    for code in codes {
        match code.opcode {
            OpcodeType::Add => { 
                data[code.output as usize] = data[code.inputs[0] as usize] + data[code.inputs[1] as usize] 
            },
            OpcodeType::Multiply => {
                data[code.output as usize] = data[code.inputs[0] as usize] * data[code.inputs[1] as usize]
            },
            _ => { return data[0] }
        }
    }
    data[0]
}
