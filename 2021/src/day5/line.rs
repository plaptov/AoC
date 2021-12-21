use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xstr, ystr) = s
            .split_once(',')
            .expect("coordinates must be in 'x,y' format");

        let x = xstr.parse::<i32>()?;
        let y = ystr.parse::<i32>()?;

        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn get_range(a: i32, b: i32) -> Box<dyn Iterator<Item = i32>> {
        if a == b {
            Box::new((a..=a).cycle())
        } else if b >= a {
            Box::new(a..=b)
        } else {
            Box::new((b..=a).into_iter().rev())
        }
    }

    pub fn get_all_points(&self) -> Vec<Point> {
        Line::get_range(self.from.x, self.to.x)
            .zip(Line::get_range(self.from.y, self.to.y))
            .map(|(x, y)| Point { x, y })
            .collect()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fromstr, tostr) = s
            .split_once(" -> ")
            .expect("coordinates must be in 'x,y -> x,y' format");
        let from = fromstr.parse::<Point>()?;
        let to = tostr.parse::<Point>()?;
        Ok(Line { from, to })
    }
}
