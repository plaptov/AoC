use std::collections::{HashMap, HashSet};

pub struct BagRulesSet {
    rules: HashMap<String, HashMap<String, i32>>,
}

impl BagRulesSet {
    pub fn new(rules: HashMap<String, HashMap<String, i32>>) -> Self {
        Self { rules }
    }

    pub fn count_bags_which_can_contain(&self, color: &str) -> usize {
        self.find_bags_which_can_contain(color)
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
            .len()
    }

    fn find_bags_which_can_contain<'a>(&'a self, color: &'a str) -> Vec<String> {
        let mut result: Vec<String> = self
            .rules
            .iter()
            .filter(move |kv| kv.1.contains_key(color))
            .map(|v| v.0.to_string())
            .collect();
        let mut rec: Vec<String> = result
            .iter()
            .flat_map(|c| self.find_bags_which_can_contain(c))
            .collect();
        result.append(&mut rec);
        result
    }

    pub fn count_recursive_bag_capacity(&self, color: &str) -> i32 {
        match self.rules.get(color) {
            None => 0,
            Some(rule) => rule
                .iter()
                .map(|kv| kv.1 + kv.1 * self.count_recursive_bag_capacity(&kv.0))
                .sum(),
        }
    }
}
