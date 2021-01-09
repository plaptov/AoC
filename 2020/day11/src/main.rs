use cellular_automaton::*;
use std::{fs::File, io::BufRead, io::BufReader};

mod cellular_automaton;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let rows: Vec<Vec<_>> = buffered
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| {
                    if c == '.' {
                        CellState::Floor
                    } else {
                        CellState::Empty
                    }
                })
                .collect()
        })
        .collect();

    let mut automaton = CellularAutomaton::new(&rows);
    let mut hash = 0;
    loop {
        let new_hash = automaton.get_state_hash();
        if new_hash == hash {
            break;
        }
        hash = new_hash;
        automaton.tick();
    }
    println!("Occupied seats: {}", automaton.occupied_count());

    let mut automaton = CellularAutomaton::new(&rows);
    let mut hash = 0;
    loop {
        let new_hash = automaton.get_state_hash();
        if new_hash == hash {
            break;
        }
        hash = new_hash;
        automaton.tick2();
    }
    println!("Occupied seats 2: {}", automaton.occupied_count());
}
