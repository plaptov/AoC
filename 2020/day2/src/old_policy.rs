use std::str::FromStr;

pub struct OldPolicy {
    min: i32,
    max: i32,
    letter: char,
}

impl OldPolicy {
    pub fn new(s: &str) -> Self {
        let splitted: Vec<&str> = s.split(" ").collect();
        let letter = splitted[1].chars().next().unwrap();
        let splitted: Vec<&str> = splitted[0].split("-").collect();
        let min = i32::from_str(&splitted[0]).unwrap();
        let max = i32::from_str(&splitted[1]).unwrap();
        OldPolicy { min, max, letter }
    }

    pub fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count() as i32;
        count >= self.min && count <= self.max
    }
}
