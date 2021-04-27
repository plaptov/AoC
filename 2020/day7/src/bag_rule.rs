use const_format::concatcp;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

const NO_BAGS: &str = "no other bags.";
const COLORED_BAGS: &str = r"(\w+ \w+) bag";
const NUMBER_OF_BAGS: &str = concatcp!(r"(\d+) ", COLORED_BAGS);

lazy_static! {
    static ref COLORED_BAGS_REGEX: Regex = Regex::new(COLORED_BAGS).unwrap();
    static ref NUMBER_OF_BAGS_REGEX: Regex = Regex::new(NUMBER_OF_BAGS).unwrap();
}

pub struct BagRule {
    pub color: String,
    pub children: HashMap<String, i32>,
}

impl BagRule {
    pub fn new(line: &str) -> Self {
        // split_once not stabilized yet
        //let (parent, children) = line.split_once(" contain ").unwrap();
        let mut split = line.splitn(2, " contain ");
        let parent = split.next().unwrap();
        let children = split.next().unwrap();
        let color = COLORED_BAGS_REGEX
            .captures(parent)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned();
        let children = if children == NO_BAGS {
            HashMap::new()
        } else {
            NUMBER_OF_BAGS_REGEX
                .captures_iter(children)
                .map(|c| {
                    (
                        c.get(2).unwrap().as_str().to_owned(),
                        i32::from_str(c.get(1).unwrap().as_str()).unwrap(),
                    )
                })
                .collect()
        };
        Self { color, children }
    }
}
