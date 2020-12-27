#![warn(clippy::all)]
use std::fs::File;
use std::io::{BufRead, BufReader};

use people_group::PeopleGroup;

mod people_group;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let mut lines_buffer = vec![];
    let mut groups = vec![];
    for res in buffered.lines() {
        let line = res.unwrap();
        if line.is_empty() {
            groups.push(PeopleGroup::new(&lines_buffer));
            lines_buffer.clear();
        } else {
            lines_buffer.push(line);
        }
    }
    if !lines_buffer.is_empty() {
        groups.push(PeopleGroup::new(&lines_buffer));
    }

    let all_groups_answers_count: usize = groups.iter().map(|g| g.answers_count()).sum();
    println!("All counts sum: {}", all_groups_answers_count);

    let all_groups_everyone_answers_count: usize =
        groups.iter().map(|g| g.everyone_answers_count()).sum();
    println!(
        "Everyone answers counts sum: {}",
        all_groups_everyone_answers_count
    );
}
