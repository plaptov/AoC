use std::io::{BufRead, BufReader};

pub fn read_lines(bytes: &[u8]) -> Vec<String> {
    let buffered = BufReader::new(bytes);
    buffered.lines().map(|line| line.unwrap()).collect()
}
