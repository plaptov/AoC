#[warn(clippy::all)]
use std::str::FromStr;
use std::{fs::File, io::BufRead, io::BufReader};

mod mask;
mod memory_manager;

use mask::Mask;
use memory_manager::MemoryManager;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);

    let mut memory_manager = MemoryManager::new();
    buffered.lines().map(|l| l.unwrap()).for_each(|line| {
        // split_once not stabilized yet
        //let (operator, operand) = line.split_once(" contain ").unwrap();
        let mut split = line.splitn(2, " = ");
        let operator = split.next().unwrap();
        let operand = split.next().unwrap();
        if operator == "mask" {
            memory_manager.set_mask(Mask::new(operand));
        } else {
            let address_str = &operator[4..&operator.len() - 1];
            let address = i64::from_str(address_str).unwrap();
            let value = i64::from_str(operand).unwrap();
            memory_manager.set(address, value);
        }
    });

    println!("Sum of values: {}", memory_manager.sum_values());
}
