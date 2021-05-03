pub struct Mask {
    or_mask: i64,
    and_mask: i64,
}

impl Mask {
    pub fn new(s: &str) -> Self {
        let mut or_mask: i64 = 0;
        let mut and_mask: i64 = !0;
        s.chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| c != &'X')
            .for_each(|(i, c)| {
                if c == '1' {
                    or_mask |= 1 << i;
                } else {
                    and_mask &= !(1 << i);
                }
            });
        Self { or_mask, and_mask }
    }

    pub fn apply(&self, value: i64) -> i64 {
        (value | self.or_mask) & self.and_mask
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            or_mask: 0,
            and_mask: !0,
        }
    }
}
