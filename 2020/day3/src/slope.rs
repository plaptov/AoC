pub struct Slope {
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
    max_y: usize,
}

impl Slope {
    pub fn new(dx: usize, dy: usize, max_y: usize) -> Self {
        Slope {
            x: 0,
            y: 0,
            dx,
            dy,
            max_y,
        }
    }
}

impl Iterator for Slope {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.dx;
        self.y += self.dy;
        if self.y >= self.max_y {
            None
        } else {
            Some((self.x, self.y))
        }
    }
}
