use std::collections::HashMap;

use self::line::Line;
use crate::reader::read_lines;
use itertools::Itertools;

mod line;

pub fn day5_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let lines = read_lines(bytes)
        .map(|line| line.parse::<Line>().unwrap())
        .collect_vec();

    let mut used_points = HashMap::new();
    for line in lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
    {
        for point in line.get_all_points() {
            used_points
                .entry(point.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    let overlapping_count = used_points.values().filter(|v| **v >= 2).count();
    println!("Overlapping count: {}", overlapping_count);

    used_points.clear();
    for line in lines {
        for point in line.get_all_points() {
            used_points
                .entry(point.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    let overlapping_count = used_points.values().filter(|v| **v >= 2).count();
    println!("Overlapping with diagonals count: {}", overlapping_count);
}
