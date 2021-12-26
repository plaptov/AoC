use crate::reader::read_lines;

pub fn day7_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let positions = read_lines(bytes)
        .map(|s| {
            s.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let cheapest = (min..=max).map(|i| calc_cost(&positions, i)).min().unwrap();
    println!("{}", cheapest);
    let cheapest = (min..=max)
        .map(|i| calc_cost2(&positions, i))
        .min()
        .unwrap();
    println!("{}", cheapest);
}

fn calc_cost(nums: &[i32], pos: i32) -> i32 {
    nums.iter().map(|i| (*i - pos).abs()).sum()
}

fn calc_cost2(nums: &[i32], pos: i32) -> i32 {
    nums.iter().map(|i| calc_fuel((*i - pos).abs())).sum()
}

fn calc_fuel(len: i32) -> i32 {
    (len + 1) * len / 2
}
