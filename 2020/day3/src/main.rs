mod map;
mod slope;

use map::Map;
use slope::Slope;

fn main() {
    let map = Map::load();
    let height = map.height();
    let mut slopes = vec![
        Slope::new(1, 1, height),
        Slope::new(3, 1, height),
        Slope::new(5, 1, height),
        Slope::new(7, 1, height),
        Slope::new(1, 2, height),
    ];

    let counts: Vec<usize> = slopes
        .drain(..)
        .map(|slope| slope.filter(|(x, y)| map.is_tree(*x, *y)).count())
        .collect();

    for count in &counts {
        println!("Encountered {} trees", count);
    }

    let result: &usize = &counts.iter().product();
    println!("Final result: {}", result);
}
