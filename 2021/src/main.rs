#[warn(clippy::all)]
mod day1;
mod day2;
mod day3;
mod reader;

fn main() {
    run(3)
}

fn run(day: i32) {
    match day {
        1 => day1::day1_fun(),
        2 => day2::day2_fun(),
        3 => day3::day3_fun(),
        _ => {}
    }
}
