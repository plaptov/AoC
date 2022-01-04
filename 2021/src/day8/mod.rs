use crate::reader::read_lines;

pub fn day8_fun() {
    let bytes: &[u8] = include_bytes!("input.txt");
    let lines = read_lines(&bytes).collect::<Vec<_>>();
    let lines = lines
        .iter()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(l, r)| {
            (
                l.split(' ').collect::<Vec<_>>(),
                r.split(' ').collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let count1478: usize = lines
        .iter()
        .map(|(_, r)| {
            r.iter()
                .filter(|s| {
                    let len = s.len();
                    len == 2 || len == 4 || len == 3 || len == 7
                })
                .count()
        })
        .sum();
    println!("{}", count1478);
}
