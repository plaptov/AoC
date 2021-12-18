use itertools::*;

#[derive(Clone)]
pub struct BingoBoard {
    numbers: Vec<(i32, bool)>,
}

impl BingoBoard {
    pub fn new(nums: Vec<i32>) -> BingoBoard {
        BingoBoard {
            numbers: nums.iter().map(|num| (*num, false)).collect(),
        }
    }

    pub fn mark(&mut self, num: i32) {
        for number in self.numbers.iter_mut() {
            if number.0 == num {
                number.1 = true;
                break;
            }
        }
    }

    pub fn is_win(&self) -> bool {
        let cols = (0..5)
            .map(|i| (0..5).map(|j| self.numbers[j * 5 + i]).collect_vec())
            .collect_vec();
        let rows = self.numbers.chunks_exact(5).collect_vec();

        rows.iter().any(|row| row.iter().all(|(_, marked)| *marked))
            || cols
                .iter()
                .any(|column| column.iter().all(|(_, marked)| *marked))
    }

    pub fn sum_of_unchecked(&self) -> i32 {
        self.numbers
            .iter()
            .filter_map(|(num, marked)| if *marked { None } else { Some(*num as i32) })
            .sum()
    }
}
