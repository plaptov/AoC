use crate::reader::read_lines;
use itertools::*;

use self::bingo_board::BingoBoard;

mod bingo_board;

pub fn day4_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let mut lines = read_lines(bytes);
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    lines.next();
    let mut boards = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| BingoBoard::new(chunk.map(|s| parse_row(&s)).flatten().collect()))
        .collect::<Vec<_>>();

    let mut first = None;
    let mut last = None;
    for number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.is_win() {
                if first.is_none() {
                    first = Some((number as i32, board.clone()));
                }
                last = Some((number as i32, board.clone()));
            }
        }
        boards.retain(|board| !board.is_win());
    }
    if let Some((num, board)) = first {
        println!("first: {}", num * board.sum_of_unchecked());
    }
    if let Some((num, board)) = last {
        println!("last: {}", num * board.sum_of_unchecked());
    }
}

fn parse_row(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
