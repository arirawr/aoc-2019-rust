/* ----- Day 2 ----- */

const AOC_2_INPUT: [u32; 148] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 2, 23, 6, 27, 1,
    27, 5, 31, 2, 6, 31, 35, 1, 5, 35, 39, 2, 39, 9, 43, 1, 43, 5, 47, 1, 10, 47, 51, 1, 51, 6, 55,
    1, 55, 10, 59, 1, 59, 6, 63, 2, 13, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75, 79, 1, 9, 79,
    83, 2, 6, 83, 87, 1, 5, 87, 91, 2, 6, 91, 95, 2, 95, 9, 99, 1, 99, 6, 103, 1, 103, 13, 107, 2,
    13, 107, 111, 2, 111, 10, 115, 1, 115, 6, 119, 1, 6, 119, 123, 2, 6, 123, 127, 1, 127, 5, 131,
    2, 131, 6, 135, 1, 135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0,
];

enum OpcodeType {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

struct Opcode {
    opcode: OpcodeType,
    inputs: [u32; 2],
    output: u32,
}

pub fn aoc_2_1() -> u32 {
    calculate_0(AOC_2_INPUT)
}

pub fn aoc_2_2() -> u32 {
    for a in 0..100 {
        for b in 0..100 {
            let mut data = AOC_2_INPUT;
            data[1] = a;
            data[2] = b;
            if calculate_0(data) == 19690720 {
                return 100 * a + b;
            }
        }
    }
    0
}

fn input_to_opcode(input: &[u32; 148]) -> Vec<Opcode> {
    let mut codes: Vec<Opcode> = Vec::new();
    for chunk in input.chunks(4) {
        let code = Opcode {
            opcode: match chunk[0] {
                1 => OpcodeType::Add,
                2 => OpcodeType::Multiply,
                _ => OpcodeType::Halt,
            },
            inputs: [chunk[1], chunk[2]],
            output: chunk[3],
        };
        codes.push(code);
    }
    codes
}

fn calculate_0(input: [u32; 148]) -> u32 {
    let codes = input_to_opcode(&input);
    let mut data = input;
    for code in codes {
        match code.opcode {
            OpcodeType::Add => {
                data[code.output as usize] =
                    data[code.inputs[0] as usize] + data[code.inputs[1] as usize]
            }
            OpcodeType::Multiply => {
                data[code.output as usize] =
                    data[code.inputs[0] as usize] * data[code.inputs[1] as usize]
            }
            _ => return data[0],
        }
    }
    data[0]
}
