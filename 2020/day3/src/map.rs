use bitvec::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Map {
    tile: Vec<BitVec>,
}

impl Map {
    pub fn load() -> Self {
        let input = File::open("input.txt").unwrap();
        let buffered = BufReader::new(input);
        let tile: Vec<BitVec> = buffered
            .lines()
            .map(|line| {
                let mut bv = bitvec![];
                for c in line.unwrap().chars() {
                    bv.push(c == '#');
                }
                bv
            })
            .collect();
        Map { tile }
    }

    pub fn height(&self) -> usize {
        self.tile.len()
    }

    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        let line = &self.tile[y];
        line[x % line.len()]
    }
}
