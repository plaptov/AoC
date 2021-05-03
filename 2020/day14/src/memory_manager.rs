use std::collections::HashMap;

use crate::mask::Mask;

pub struct MemoryManager {
    mask: Mask,
    memory: HashMap<i64, i64>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            mask: Mask::default(),
            memory: HashMap::new(),
        }
    }

    pub fn set(&mut self, address: i64, value: i64) {
        self.memory.insert(address, self.mask.apply(value));
    }

    pub fn set_mask(&mut self, mask: Mask) {
        self.mask = mask
    }

    pub fn sum_values(&self) -> i64 {
        self.memory.values().sum()
    }
}
