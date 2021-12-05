use crate::reader::read_lines;
use std::{collections::VecDeque, str::FromStr};

pub fn day1_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let numbers = read_lines(&bytes)
        .iter()
        .map(|l| i32::from_str(l).unwrap())
        .collect::<Vec<i32>>();
    let mut prev = i32::MAX;
    let mut count = 0;
    for n in &numbers {
        if n > &prev {
            count += 1;
        }
        prev = *n;
    }
    println!("{} increases", count);

    let mut buf = VecDeque::new();
    prev = i32::MAX;
    count = 0;
    for n in &numbers {
        buf.push_back(*n);
        if buf.len() == 3 {
            let sum = buf.iter().sum();
            if sum > prev {
                count += 1;
            }
            prev = sum;
            buf.pop_front();
        }
    }
    println!("{} increases", count);
}
