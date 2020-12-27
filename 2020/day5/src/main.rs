#![warn(clippy::all)]
use std::io::{BufRead, BufReader};
use std::{collections::HashSet, fs::File};

use seat::Seat;

mod binary_decoder;
mod seat;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);

    let seats: Vec<Seat> = buffered
        .lines()
        .map(|r| r.unwrap())
        .map(|line| Seat::new(&line))
        .collect();

    let ids: HashSet<u32> = seats.iter().map(|seat| seat.id()).collect();
    let max_id = ids.iter().max().unwrap();
    println!("Max id is {}", max_id);
    let free_seats: Vec<u32> = (1..*max_id)
        .filter(|id| !ids.contains(id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)))
        .collect();
    for id in free_seats {
        println!("{}", id);
    }
}
