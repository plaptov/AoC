use std::{fs::File, io::BufRead, io::BufReader};

use actions::Action;
use ship::Ship;

mod actions;
mod ship;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let actions: Vec<_> = buffered
        .lines()
        .map(|line| {
            let s = line.unwrap();
            let (head, tail) = s.split_at(1);
            (
                head.parse::<Action>().unwrap(),
                tail.parse::<u32>().unwrap(),
            )
        })
        .collect();

    let ship = actions
        .iter()
        .cloned()
        .fold(Ship::new(), |ship, (action, value)| {
            ship.perform(action, value)
        });
    println!("Manhattan distance is {}", ship.get_manhattan_distance());

    let ship = actions
        .iter()
        .cloned()
        .fold(Ship::new(), |ship, (action, value)| {
            ship.perform_by_waypoint(action, value)
        });
    println!("Manhattan distance is {}", ship.get_manhattan_distance());
}
