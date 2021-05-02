#[warn(clippy::all)]
use std::{fs::File, io::BufRead, io::BufReader};

use std::str::FromStr;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines().map(|x| x.unwrap());
    let timestamp = i32::from_str(&(lines.next().unwrap())).unwrap();
    let s = lines.next().unwrap();
    let numbers = s
        .split(',')
        .filter(|a| a != &"x")
        .map(|a| i32::from_str(a).unwrap())
        .collect::<Vec<i32>>();
    let mut buses: Vec<Bus> = numbers.iter().map(|n| Bus::new(*n)).collect();
    buses.sort_by_key(|b| b.arriving(timestamp));
    let first_bus = &buses[0];
    let waiting_time = first_bus.arriving(timestamp) - timestamp;
    println!(
        "bus number:{}\narriving time: {}\nwaiting time: {}\nresult: {}",
        first_bus.number,
        first_bus.arriving(timestamp),
        waiting_time,
        waiting_time * first_bus.number
    );
}

struct Bus {
    number: i32,
}

impl Bus {
    fn new(number: i32) -> Self {
        Self { number }
    }

    fn arriving(&self, not_before: i32) -> i32 {
        let x = not_before / self.number * self.number;
        if x < not_before {
            x + self.number
        } else {
            x
        }
    }
}
