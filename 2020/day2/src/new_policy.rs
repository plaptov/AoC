use std::str::FromStr;

pub struct NewPolicy {
    first_pos: usize,
    second_pos: usize,
    letter: char,
}

impl NewPolicy {
    pub fn new(s: &str) -> Self {
        let splitted: Vec<&str> = s.split(" ").collect();
        let letter = splitted[1].chars().next().unwrap();
        let splitted: Vec<&str> = splitted[0].split("-").collect();
        let first_pos = usize::from_str(&splitted[0]).unwrap();
        let second_pos = usize::from_str(&splitted[1]).unwrap();
        NewPolicy {
            first_pos,
            second_pos,
            letter,
        }
    }

    pub fn is_valid(&self, password: &str) -> bool {
        let first = password.chars().skip(self.first_pos - 1).next().unwrap();
        let second = password.chars().skip(self.second_pos - 1).next().unwrap();
        (first == self.letter) != (second == self.letter)
    }
}
