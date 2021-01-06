use std::collections::VecDeque;

pub struct Weakness<'a> {
    requested_sum: u64,
    buf: VecDeque<u64>,
    source: &'a [u64],
}

impl<'a> Weakness<'a> {
    pub fn new(requested_sum: u64, source: &'a [u64]) -> Self {
        Self {
            requested_sum,
            buf: VecDeque::new(),
            source,
        }
    }

    pub fn find(&mut self) -> Vec<u64> {
        for n in self.source {
            self.buf.push_back(*n);
            let mut sum: u64 = self.buf.iter().sum();
            while sum > self.requested_sum {
                self.buf.pop_front();
                sum = self.buf.iter().sum();
            }
            if sum == self.requested_sum {
                return self.buf.iter().copied().collect();
            }
        }
        panic!()
    }
}
