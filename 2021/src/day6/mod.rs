use crate::reader::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

pub fn day6_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let mut nums = read_lines(bytes)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().expect("parse error"))
                .collect::<Vec<_>>()
        })
        .flatten()
        .sorted()
        .group_by(|i| *i)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .collect::<HashMap<_, _>>();

    const DAYS: i32 = 80;
    calc(&mut nums, DAYS);
    print_count(DAYS, &nums);

    const DAYS2: i32 = 256;
    calc(&mut nums, DAYS2 - DAYS);
    print_count(DAYS2, &nums);
}

fn calc(nums: &mut HashMap<i32, usize>, days: i32) {
    for _day in 1..=days {
        for i in 0..=8 {
            let val;
            if let Some(count) = nums.get(&i) {
                val = *count;
            } else {
                continue;
            }
            nums.insert(i - 1, val);
            nums.remove(&i);
        }

        let new_count;
        if let Some(count) = nums.get(&-1) {
            new_count = *count;
        } else {
            continue;
        }
        nums.remove(&-1);
        nums.insert(8, new_count);
        let entry = nums.entry(6).or_default();
        *entry += new_count;
    }
}

fn print_count(day: i32, nums: &HashMap<i32, usize>) {
    let sum: usize = nums.values().sum();
    println!("After {} days it will be {} lanternfishes", day, sum);
}
