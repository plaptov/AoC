#![warn(clippy::all)]
use std::fs::File;
use std::io::{BufRead, BufReader};

use bag_rule::BagRule;
use bag_rules_set::BagRulesSet;
#[macro_use]
extern crate lazy_static;

mod bag_rule;
mod bag_rules_set;

fn main() {
    let input = File::open("input.txt").unwrap();
    let buffered = BufReader::new(input);

    let rules_list = buffered
        .lines()
        .map(|line| BagRule::new(&line.unwrap()))
        .map(|rule| (rule.color, rule.children))
        .collect();

    let rules_set = BagRulesSet::new(rules_list);
    let count = rules_set.count_bags_which_can_contain("shiny gold");
    println!(
        "Number of bag colors that can contain shiny gold bag: {}",
        count
    );

    let recursive_count = rules_set.count_recursive_bag_capacity("shiny gold");
    println!(
        "Shiny gold bag required {} individual bags inside",
        recursive_count
    );
}
