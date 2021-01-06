use std::collections::VecDeque;

pub struct InvalidNumbers<'a> {
    size: usize,
    buf: VecDeque<u64>,
    source: &'a mut dyn Iterator<Item = u64>,
}

impl<'a> InvalidNumbers<'a> {
    pub fn new(source: &'a mut dyn Iterator<Item = u64>, size: usize) -> Self {
        Self {
            size,
            buf: VecDeque::with_capacity(size),
            source,
        }
    }

    pub fn init(&mut self) {
        while self.buf.len() < self.size {
            self.buf.push_back(self.source.next().unwrap());
        }
    }

    pub fn find_invalid_number(&mut self) -> u64 {
        while let Some(n) = self.source.next() {
            if !self.is_valid(n) {
                return n;
            }
            self.buf.pop_front();
            self.buf.push_back(n);
        }
        panic!("Invalid number not found")
    }

    fn is_valid(&self, n: u64) -> bool {
        self.buf
            .iter()
            .enumerate()
            .any(|(i, val)| self.buf.iter().skip(i + 1).any(|val2| val + val2 == n))
    }
}
