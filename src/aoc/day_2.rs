/* ----- Day 2 ----- */
use crate::aoc;

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
    calculate_0(parse_data())
}

pub fn aoc_2_2() -> u32 {
    for a in 0..100 {
        for b in 0..100 {
            let mut data = parse_data();
            data[1] = a;
            data[2] = b;
            if calculate_0(data) == 19690720 {
                return 100 * a + b;
            }
        }
    }
    0
}

fn parse_data() -> Vec<u32> {
    let string = aoc::util::read_input(2).unwrap();
    string.split(",").map(|i| i.parse().unwrap()).collect()
}

fn input_to_opcode(input: &Vec<u32>) -> Vec<Opcode> {
    let mut codes: Vec<Opcode> = Vec::new();
    for chunk in input.chunks_exact(4) {
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

fn calculate_0(input: Vec<u32>) -> u32 {
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
