use std::collections::HashSet;

pub struct PeopleGroup {
    all_answers: HashSet<char>,
    everyone_answers: HashSet<char>,
}

impl PeopleGroup {
    pub fn new(lines: &[String]) -> Self {
        let all_answers: HashSet<char> = lines.iter().flat_map(|line| line.chars()).collect();
        let everyone_answers = lines
            .iter()
            .map(|line| line.chars().collect::<HashSet<_>>())
            .fold(all_answers.clone(), |acc, line| {
                acc.intersection(&line).cloned().collect()
            });
        Self {
            all_answers,
            everyone_answers,
        }
    }

    pub fn answers_count(&self) -> usize {
        self.all_answers.len()
    }

    pub fn everyone_answers_count(&self) -> usize {
        self.everyone_answers.len()
    }
}
