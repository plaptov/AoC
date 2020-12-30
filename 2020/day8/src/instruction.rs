use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
pub enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

pub struct Instruction {
    pub name: InstructionType,
    pub data: i32,
}

impl Instruction {
    pub fn new(s: &str) -> Instruction {
        let name = &s[0..3];
        let data = i32::from_str(&s[4..]).unwrap();
        let name = match name {
            "nop" => InstructionType::Nop,
            "acc" => InstructionType::Acc,
            "jmp" => InstructionType::Jmp,
            _ => panic!("unexpected instruction name"),
        };
        Instruction { name, data }
    }
}
