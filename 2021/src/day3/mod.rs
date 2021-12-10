use crate::reader::read_lines;

fn parse_line(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '1').collect()
}

fn apply_line(a: &mut [i32], line: &[bool]) {
    for (sum, flag) in a.iter_mut().zip(line.iter()) {
        if *flag {
            *sum += 1;
        } else {
            *sum -= 1;
        }
    }
}

fn bin_to_hex(b: &[bool]) -> i32 {
    b.iter()
        .rev()
        .enumerate()
        .map(|(pos, val)| if *val { 1 << pos } else { 0 })
        .sum()
}

pub fn day3_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let data = read_lines(bytes)
        .map(|s| parse_line(&s))
        .collect::<Vec<_>>();
    let size = &data[0].len();
    let mut sum = vec![0; *size];
    data.iter().for_each(|line| apply_line(&mut sum, line));
    let gamma = sum.iter().map(|i| *i > 0).collect::<Vec<_>>();
    let gamma_value = bin_to_hex(&gamma);
    let epsilon = gamma.iter().map(|flag| !flag).collect::<Vec<_>>();
    let epsilon_value = bin_to_hex(&epsilon);
    println!("power consumption is {}", gamma_value * epsilon_value);

    let oxygen_rating = bin_to_hex(&find_rating(&data, false));
    let co2_rating = bin_to_hex(&find_rating(&data, true));
    println!("life support rating is {}", oxygen_rating * co2_rating);
}

fn find_rating(lines: &[Vec<bool>], inverted: bool) -> Vec<bool> {
    let mut lines = Vec::from(lines);
    let mut index: usize = 0;
    loop {
        if lines.len() == 1 {
            return lines[0].clone();
        }
        let common_bit = calc_common_bit(&lines, index);
        let filter = if inverted { !common_bit } else { common_bit };
        lines.retain(|line| line[index] == filter);
        index += 1;
    }
}

fn calc_common_bit(lines: &[Vec<bool>], index: usize) -> bool {
    lines
        .iter()
        .map(|line| if line[index] { 1 } else { -1 })
        .sum::<i32>()
        >= 0
}
