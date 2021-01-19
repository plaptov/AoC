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

impl From<Action> for Direction {
    fn from(a: Action) -> Self {
        match a {
            Action::North => Direction::North,
            Action::South => Direction::South,
            Action::East => Direction::East,
            Action::West => Direction::West,
            _ => panic!("unexpected action"),
        }
    }
}

trait Position {
    fn get_position(&self) -> (i32, i32);
    fn move_to(&self, direction: Direction, value: i32) -> (i32, i32) {
        let (mut x, mut y) = self.get_position();
        match direction {
            Direction::North => y -= value,
            Direction::South => y += value,
            Direction::East => x += value,
            Direction::West => x -= value,
        }
        (x, y)
    }
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Self { x: 10, y: -1 }
    }

    fn rotate(self, degrees: i32) -> Self {
        let steps = (degrees / 90).abs();
        let (mut x, mut y) = (self.x, self.y);
        for _ in 0..steps {
            let pt = if degrees < 0 {
                Self::rotate_counter_clockwise_90(x, y)
            } else {
                Self::rotate_clockwise_90(x, y)
            };
            x = pt.0;
            y = pt.1;
        }
        Self { x, y }
    }

    fn rotate_clockwise_90(x: i32, y: i32) -> (i32, i32) {
        (-y, x)
    }

    fn rotate_counter_clockwise_90(x: i32, y: i32) -> (i32, i32) {
        (y, -x)
    }
}

impl Position for Waypoint {
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub struct Ship {
    direction: Direction,
    x: i32,
    y: i32,
    waypoint: Waypoint,
}

impl Position for Ship {
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Ship {
    pub fn new() -> Self {
        Self {
            direction: Direction::East,
            x: 0,
            y: 0,
            waypoint: Waypoint::new(),
        }
    }

    pub fn get_manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn perform(self, action: Action, value: u32) -> Self {
        match action {
            Action::Left => Self {
                direction: self.direction.rotate(-(value as i32)),
                ..self
            },
            Action::Right => Self {
                direction: self.direction.rotate(value as i32),
                ..self
            },
            Action::Forward => {
                let (x, y) = self.move_to(self.direction, value as i32);
                Self { x, y, ..self }
            }
            a => {
                let (x, y) = self.move_to(a.into(), value as i32);
                Self { x, y, ..self }
            }
        }
    }

    pub fn perform_by_waypoint(self, action: Action, value: u32) -> Self {
        match action {
            Action::Left => Self {
                waypoint: self.waypoint.rotate(-(value as i32)),
                ..self
            },
            Action::Right => Self {
                waypoint: self.waypoint.rotate(value as i32),
                ..self
            },
            Action::Forward => {
                let x = self.x + self.waypoint.x * value as i32;
                let y = self.y + self.waypoint.y * value as i32;
                Self { x, y, ..self }
            }
            a => {
                let (x, y) = self.waypoint.move_to(a.into(), value as i32);
                Self {
                    waypoint: Waypoint { x, y },
                    ..self
                }
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
