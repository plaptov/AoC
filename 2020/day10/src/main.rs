#![warn(clippy::all)]
use flat_node::*;
use itertools::Itertools;
use std::{fs::File, io::BufRead, io::BufReader, iter, str::FromStr};

mod flat_node;

fn main() {
    const FILENAME: &str = "input.txt";
    let input = File::open(FILENAME).unwrap();
    let buffered = BufReader::new(input);
    let numbers: Vec<_> = buffered
        .lines()
        .map(|line| i32::from_str(&line.unwrap()).unwrap())
        .chain(iter::once(0))
        .sorted()
        .collect();
    let len = numbers.len();

    let diffs: Vec<_> = numbers
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if i < len - 1 {
                (numbers[i + 1] - v) as i32
            } else {
                3 // device has adapter rated for 3 jolts higher than the highest-rated adapter in your bag
            }
        })
        .sorted()
        .group_by(|x| *x)
        .into_iter()
        .map(|(i, gr)| (i, gr.count()))
        .collect();
    for (i, count) in diffs {
        println!("{}: {}", i, count);
    }

    let count = calc_by_flat_nodes(&numbers);
    println!("{} valid ways", count);
}
