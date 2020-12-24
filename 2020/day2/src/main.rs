use std::fs::File;
use std::io::{BufRead, BufReader, Error};

mod new_policy;
mod old_policy;

use new_policy::NewPolicy;
use old_policy::OldPolicy;

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);

    let mut old_count = 0;
    let mut new_count = 0;
    for line in buffered.lines() {
        let s = line?;
        let splitted: Vec<&str> = s.split(": ").collect();
        let old_policy = OldPolicy::new(splitted[0]);
        let new_policy = NewPolicy::new(splitted[0]);
        let password = splitted[1];
        if old_policy.is_valid(password) {
            //println!("{}", s);
            old_count += 1;
        }
        if new_policy.is_valid(password) {
            println!("{}", s);
            new_count += 1;
        }
    }
    println!("old valid count: {}", old_count);
    println!("new valid count: {}", new_count);

    Ok(())
}
