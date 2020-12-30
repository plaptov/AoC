use std::collections::HashSet;

use crate::instruction::*;

pub enum RunResult {
    Duplicate(i32),
    Ok(i32),
    InvalidCode,
}

pub fn run_code(instruction_set: &[Instruction], try_fix: Option<i32>) -> RunResult {
    let mut accumulator = 0;
    let mut used = HashSet::new();
    let mut current_instruction: i32 = 0;
    loop {
        let instruction = &instruction_set[current_instruction as usize];
        used.insert(current_instruction);
        let mut inst_type = instruction.name;
        if let Some(fixed) = try_fix {
            if fixed == current_instruction {
                inst_type = fix(inst_type);
            }
        }
        if inst_type == InstructionType::Jmp && instruction.data == 0 {
            return RunResult::InvalidCode;
        }
        match inst_type {
            InstructionType::Nop => current_instruction += 1,
            InstructionType::Acc => {
                accumulator += instruction.data;
                current_instruction += 1;
            }
            InstructionType::Jmp => current_instruction += instruction.data,
        }
        if used.contains(&current_instruction) {
            return RunResult::Duplicate(accumulator);
        }
        if current_instruction >= instruction_set.len() as i32 {
            return RunResult::Ok(accumulator);
        }
    }
}

fn fix(inst_type: InstructionType) -> InstructionType {
    match inst_type {
        InstructionType::Nop => InstructionType::Jmp,
        InstructionType::Jmp => InstructionType::Nop,
        InstructionType::Acc => panic!(),
    }
}
