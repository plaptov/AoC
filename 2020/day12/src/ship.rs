use crate::actions::Action;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Direction {
    pub fn rotate(&self, degrees: i32) -> Direction {
        let ind = DIRECTIONS.iter().position(|x| x == self).unwrap() as i32;
        let mut new_ind = ind + degrees / 90;
        while new_ind < 0 {
            new_ind += DIRECTIONS.len() as i32;
        }
        while new_ind >= DIRECTIONS.len() as i32 {
            new_ind -= DIRECTIONS.len() as i32;
        }
        DIRECTIONS[new_ind as usize]
    }
}

impl From<&Direction> for Action {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::North => Action::North,
            Direction::South => Action::South,
            Direction::East => Action::East,
            Direction::West => Action::West,
        }
    }
}

pub struct Ship {
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }

    pub fn get_manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn perform(self, action: Action, value: u32) -> Self {
        match action {
            Action::North => Self {
                y: self.y - value as i32,
                ..self
            },
            Action::South => Self {
                y: self.y + value as i32,
                ..self
            },
            Action::East => Self {
                x: self.x + value as i32,
                ..self
            },
            Action::West => Self {
                x: self.x - value as i32,
                ..self
            },
            Action::Left => Self {
                direction: self.direction.rotate(-(value as i32)),
                ..self
            },
            Action::Right => Self {
                direction: self.direction.rotate(value as i32),
                ..self
            },
            Action::Forward => {
                let d = (&self.direction).into();
                self.perform(d, value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn rotate_90() {
        let rotated = Direction::East.rotate(90);
        assert_eq!(rotated, Direction::South);
    }

    #[test]
    fn rotate_270() {
        let rotated = Direction::East.rotate(270);
        assert_eq!(rotated, Direction::North);
    }

    #[test]
    fn rotate_minus_270() {
        let rotated = Direction::East.rotate(-270);
        assert_eq!(rotated, Direction::South);
    }
}
