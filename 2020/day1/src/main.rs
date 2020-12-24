use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);

    let numbers: Vec<i32> = buffered
        .lines()
        .map(|line| i32::from_str(&line.unwrap()).unwrap())
        .collect();

    for (i, num) in numbers.iter().enumerate() {
        for num2 in &numbers[i + 1..] {
            if num + num2 == 2020 {
                println!("{} + {} = {}", num, num2, num * num2);
            }
        }
    }

    for (i, num) in numbers.iter().enumerate() {
        for (j, num2) in &mut numbers[i + 1..].iter().enumerate() {
            for num3 in &numbers[j + 1..] {
                if num + num2 + num3 == 2020 {
                    println!("{} + {} + {} = {}", num, num2, num3, num * num2 * num3);
                }
            }
        }
    }

    Ok(())
}
