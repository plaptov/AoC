#![warn(clippy::all)]
use std::{fs::File, io::BufRead, io::BufReader, str::FromStr};

use invalid_numbers::InvalidNumbers;
use weakness::Weakness;

mod invalid_numbers;
mod weakness;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let numbers: Vec<_> = buffered
        .lines()
        .map(|line| u64::from_str(&line.unwrap()).unwrap())
        .collect();

    let invalid_number = {
        let mut iter = numbers.iter().cloned();
        let mut numbers_buf = InvalidNumbers::new(&mut iter, 25);
        numbers_buf.init();
        numbers_buf.find_invalid_number()
    };
    println!("Invalid number is {}", invalid_number);

    let weakness = Weakness::new(invalid_number, &numbers).find();
    let min = *weakness.iter().min().unwrap();
    let max = *weakness.iter().max().unwrap();
    println!("Encryption weakness is {}", min + max);
}
