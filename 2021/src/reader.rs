use std::io::{BufRead, BufReader};

pub fn read_lines(bytes: &'static [u8]) -> impl Iterator<Item = String> {
    let buffered = BufReader::new(bytes);
    buffered.lines().map(move |line| line.unwrap())
}
