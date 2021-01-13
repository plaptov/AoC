use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
pub struct ActionParseError {}

impl FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Action::North),
            "S" => Ok(Action::South),
            "E" => Ok(Action::East),
            "W" => Ok(Action::West),
            "L" => Ok(Action::Left),
            "R" => Ok(Action::Right),
            "F" => Ok(Action::Forward),
            _ => Err(Self::Err {}),
        }
    }
}
