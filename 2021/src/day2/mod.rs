use crate::reader::read_lines;
use std::str::FromStr;

enum Move {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
pub struct MoveParseError {}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Move::Forward),
            "up" => Ok(Move::Up),
            "down" => Ok(Move::Down),
            _ => Err(MoveParseError {}),
        }
    }
}

pub fn day2_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let moves: Vec<(Move, i32)> = read_lines(bytes)
        .map(|s| {
            let (s1, s2) = s.split_once(&" ").unwrap();
            (s1.parse().unwrap(), s2.parse().unwrap())
        })
        .collect();

    let mut horizontal_pos = 0;
    let mut depth = 0;
    for (m, i) in moves.iter() {
        match m {
            Move::Forward => horizontal_pos += i,
            Move::Up => depth -= i,
            Move::Down => depth += i,
        }
    }

    println!("Product: {}", horizontal_pos * depth);

    horizontal_pos = 0;
    depth = 0;
    let mut aim = 0;
    for (m, i) in moves.iter() {
        match m {
            Move::Forward => {
                horizontal_pos += i;
                depth += aim * i;
            }
            Move::Up => aim -= i,
            Move::Down => aim += i,
        }
    }

    println!("Product 2: {}", horizontal_pos * depth);
}
