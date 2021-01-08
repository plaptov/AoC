pub fn calc_by_flat_nodes(numbers: &[i32]) -> usize {
    let nodes: Vec<_> = numbers
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let step_indexes: Vec<_> = numbers
                .iter()
                .enumerate()
                .skip(i + 1)
                .take_while(|(_, v)| **v <= val + 3)
                .map(|(i, _)| i)
                .collect();
            step_indexes
        })
        .collect();
    let mut counts = vec![0; nodes.len()];
    nodes.iter().enumerate().rev().for_each(|(i, steps)| {
        let count = if steps.is_empty() {
            1
        } else {
            steps.iter().map(|step| counts[*step]).sum()
        };
        counts[i] = count;
    });
    *counts.first().unwrap()
}
