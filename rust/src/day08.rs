use crate::aoc;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

#[derive(Debug, Clone)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

impl Instruction {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(nop|acc|jmp) ([\+\-])(\d+)$").unwrap();
        }
        let cap = RE.captures(line).unwrap();
        let operation = match &cap[1] {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => panic!("Operation could not be parsed"),
        };
        let raw_arg = cap[3].parse::<isize>().unwrap();
        let argument = match &cap[2] {
            "+" => raw_arg,
            "-" => 0 - raw_arg,
            _ => panic!("Argument sign could not be parsed"),
        };
        Self {
            operation,
            argument,
        }
    }

    fn perform(&self, pointer: &mut usize, accumulator: &mut isize) {
        match self.operation {
            Operation::Nop => {
                *pointer += 1;
            }
            Operation::Acc => {
                *accumulator += self.argument;
                *pointer += 1;
            }
            Operation::Jmp => {
                let new_position = *pointer as isize + self.argument;
                if new_position < 0 {
                    panic!("Attempted to jump to negative memory address.")
                }
                *pointer = new_position as usize;
            }
        }
    }
}

fn run_program(instructions: &HashMap<usize, Instruction>) -> (usize, isize) {
    let mut instructions_copy = instructions.clone();
    let mut instruction_pointer = 0;
    let mut accumulator = 0;

    while instructions_copy.get(&instruction_pointer).is_some() {
        let current_instruction = instructions_copy.remove(&instruction_pointer).unwrap();
        current_instruction.perform(&mut instruction_pointer, &mut accumulator);
    }
    (instruction_pointer, accumulator)
}

pub fn part1() {
    let lines = aoc::read_lines("src/day08.txt");
    let mut instructions = HashMap::new();
    for (idx, line) in lines.iter().enumerate() {
        instructions.insert(idx, Instruction::from(line));
    }

    let (_, accumulator) = run_program(&instructions);
    aoc::print_response(8, 1, accumulator);
}

pub fn part2() {
    let lines = aoc::read_lines("src/day08.txt");
    let mut instructions = HashMap::new();
    let mut jmp_nop_ids = vec![];
    for (idx, line) in lines.iter().enumerate() {
        let instruction = Instruction::from(line);
        match &instruction.operation {
            Operation::Acc => (),
            _ => jmp_nop_ids.push(idx),
        };
        instructions.insert(idx, instruction);
    }

    for jmp_nop_idx in jmp_nop_ids {
        let mut instructions_copy = instructions.clone();
        let swapped = instructions_copy.get_mut(&jmp_nop_idx).unwrap();
        match swapped.operation {
            Operation::Nop => swapped.operation = Operation::Jmp,
            Operation::Jmp => swapped.operation = Operation::Nop,
            _ => (),
        };

        let (pointer, accumulator) = run_program(&instructions_copy);
        if pointer >= instructions.len() {
            aoc::print_response(8, 2, accumulator);
            break;
        }
    }
}
