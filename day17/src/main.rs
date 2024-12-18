use std::io::Read;

fn combo_operand_value(operand: u8, registers: &[u64; 3]) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4..=6 => registers[operand as usize - 4],
        _ => panic!("Invalid operand"),
    }
}

fn parse_input(input: &str) -> ([u64; 3], Vec<u8>) {
    let mut registers = [0; 3];
    let mut lines = input.lines();
    for i in 0..3 {
        registers[i] = lines.next().unwrap().split_whitespace().nth(2).unwrap().parse().unwrap();
    }

    let instructions: Vec<u8> = lines.nth(1).unwrap().split_whitespace().nth(1).unwrap().split(',').map(|num|
        num.parse().unwrap()
    ).collect();

    (registers, instructions)
}

fn run_program(initial_registers: &[u64; 3], instructions: &[u8], result: &mut Vec<u8>) {
    let mut registers = initial_registers.clone();
    let mut pc = 0;

    while pc < instructions.len() {
        let operator = instructions[pc];
        let operand = instructions[pc + 1];
        pc += 2;
        match operator {
            0 => {
                registers[0] >>= combo_operand_value(operand, &registers);
            },
            1 => {
                registers[1] ^= operand as u64;
            },
            2 => {
                registers[1] = combo_operand_value(operand, &registers) & 0x7;
            },
            3 => {
                if registers[0] != 0 {
                    pc = operand as usize;
                }
            },
            4 => {
                registers[1] ^= registers[2];
            },
            5 => {
                result.push((combo_operand_value(operand, &registers) & 0x7) as u8);
            },
            6 => {
                registers[1] = registers[0] >> combo_operand_value(operand, &registers);
            },
            7 => {
                registers[2] = registers[0] >> combo_operand_value(operand, &registers);
            },
            _ => panic!("Invalid operator"),
        }
    }
}

fn part_1(input: &str) -> String {
    let (registers, instructions) = parse_input(input);
    let mut result = Vec::new();
    run_program(&registers, &instructions, &mut result);
    result.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
}

fn lowest_quine_input(current_val: u64, position: usize, initial_registers: &mut [u64; 3], instructions: &[u8], result: &mut Vec<u8>) -> Option<u64> {
    result.clear();
    initial_registers[0] = current_val;
    run_program(&initial_registers, &instructions, result);
    if result[position] != instructions[position] {
        None
    } else if position == 0 {
        Some(current_val)
    } else {
        (0..=7).find_map(|x| {
            let position = position - 1;
            lowest_quine_input(current_val | (x << (position * 3)), position, initial_registers, &instructions, result)
        })
    }
}

fn part_2(input: &str) -> u64 {
    let (mut registers, instructions) = parse_input(input);

    (1..).find_map(|x| {
        let position = instructions.len() - 1;
        lowest_quine_input(x << (position * 3), position, &mut registers, &instructions, &mut Vec::new())
    }).unwrap()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
