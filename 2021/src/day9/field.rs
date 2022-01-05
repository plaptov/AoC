use array2d::Array2D;

pub struct Field {
    data: Array2D<i8>,
}

impl Field {
    pub fn new(elements: &[Vec<i8>]) -> Self {
        Self {
            data: Array2D::from_rows(elements),
        }
    }

    pub fn get_low_points(&self) -> Vec<i8> {
        let mut result = Vec::new();
        for y in 0..self.data.num_rows() {
            for x in 0..self.data.num_columns() {
                let point = *self.data.get(y, x).unwrap();
                let neighbors = self.get_neighbors(x, y);
                if neighbors.iter().all(|&n| n > point) {
                    result.push(point);
                }
            }
        }
        result
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<i8> {
        let mut result = Vec::new();
        if x > 0 {
            result.push(self.data.get(y, x - 1));
        }
        if y > 0 {
            result.push(self.data.get(y - 1, x));
        }
        result.push(self.data.get(y, x + 1));
        result.push(self.data.get(y + 1, x));
        result.iter().filter_map(|x| *x).cloned().collect()
    }
}
