pub fn decode(code: &str, size: u8, zero_char: char, one_char: char) -> u8 {
    let mut start: u8 = 0;
    let mut end: u8 = size - 1;
    for c in code.chars() {
        match c {
            zero if zero == zero_char => end = start + (end - start) / 2,
            one if one == one_char => start = start + (end - start) / 2 + 1,
            unknown => panic!("Unknown symbol: {}", unknown),
        }
    }
    if start != end {
        panic!("Too many values");
    }
    start
}
