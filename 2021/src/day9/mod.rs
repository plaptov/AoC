use crate::reader::read_lines;

use self::field::Field;

mod field;

pub fn day9_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let lines = read_lines(&bytes)
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let field = Field::new(&lines);
    let low_points = field.get_low_points();
    let sum = low_points.iter().map(|p| (*p + 1) as i32).sum::<i32>();
    println!("{}", sum);
}
