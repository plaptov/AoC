use crate::binary_decoder::decode;

pub struct Seat {
    pub row: u8,
    pub column: u8,
}

impl Seat {
    pub fn id(&self) -> u32 {
        self.row as u32 * 8 + self.column as u32
    }

    pub fn new(s: &str) -> Self {
        let (row_str, column_str) = s.split_at(7);
        let row = decode(row_str, 128, 'F', 'B');
        let column = decode(column_str, 8, 'L', 'R');
        Seat { row, column }
    }
}
