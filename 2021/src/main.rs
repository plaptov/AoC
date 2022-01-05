#[warn(clippy::all)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod reader;

fn main() {
    run(9)
}

fn run(day: i32) {
    match day {
        1 => day1::day1_fun(),
        2 => day2::day2_fun(),
        3 => day3::day3_fun(),
        4 => day4::day4_fun(),
        5 => day5::day5_fun(),
        6 => day6::day6_fun(),
        7 => day7::day7_fun(),
        8 => day8::day8_fun(),
        9 => day9::day9_fun(),
        _ => {}
    }
}
