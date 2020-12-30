#![warn(clippy::all)]
use std::fs::File;
use std::io::{BufRead, BufReader};

use code_runner::*;
use instruction::*;

mod code_runner;
mod instruction;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let instructions: Vec<_> = buffered
        .lines()
        .map(|line| Instruction::new(&line.unwrap()))
        .collect();
    let res = run_code(&instructions, None);
    print_result(&res);
    let fixed = instructions
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.name != InstructionType::Acc)
        .map(|(ind, _)| ind)
        .take_while(|ind| {
            let res = run_code(&instructions, Some(*ind as i32));
            match res {
                RunResult::Ok(_) => {
                    print_result(&res);
                    false
                }
                _ => true,
            }
        })
        .last();
    println!("Fixed instruction: {:?}", fixed);
}

fn print_result(res: &RunResult) {
    match res {
        RunResult::Duplicate(acc) => println!("Duplicate! Accumulator value: {}", acc),
        RunResult::Ok(acc) => println!("OK! Accumulator value: {}", acc),
        RunResult::InvalidCode => println!("Invalid code"),
    }
}
