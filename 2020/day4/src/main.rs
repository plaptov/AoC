#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader};

use passport::Passport;

mod passport;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);
    let mut buf = vec![];
    let mut count = 0;
    let mut valid_count = 0;
    let mut check_passport = |b: &Vec<String>| {
        let passport = Passport::new(&b);
        if passport.all_fields_presents() {
            count += 1;
            if passport.all_fields_valid() {
                valid_count += 1;
            }
        }
    };
    for line in buffered.lines() {
        let s = line.unwrap();
        if s.is_empty() {
            check_passport(&buf);
            buf.clear();
        } else {
            buf.push(s);
        }
    }
    if !buf.is_empty() {
        check_passport(&buf);
    }
    println!("{} valid passports", count);
    println!("{} valid passports with all valid fields", valid_count);
}
