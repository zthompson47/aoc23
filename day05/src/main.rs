use std::{collections::HashMap, ops::Range};

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut maps: HashMap<&'static str, Vec<Rule>> = HashMap::default();
    let mut current_map = None;

    lines.for_each(|line| {
        if line.contains("map") {
            if let Some((map_name, _)) = line.split_once(' ') {
                current_map = Some(map_name);
                maps.insert(map_name, Vec::new());
            }
        } else if line.is_empty() {
            current_map = None;
        }

        if line.starts_with(|x: char| x.is_ascii_digit()) {
            if let Some(map_name) = current_map {
                let mut values = line.split(' ').map(|x| x.parse::<usize>());

                let destination_start = values.next().unwrap().unwrap();
                let source_start = values.next().unwrap().unwrap();
                let source_end = source_start + values.next().unwrap().unwrap();

                let rule = Rule {
                    destination_start,
                    source_range: source_start..source_end,
                };

                maps.get_mut(map_name).unwrap().push(rule);
            }
        }
    });

    let mut result = Vec::new();

    for seed in seeds {
        let mut cur_val = seed;
        let mut new_val = None;

        for map_name in MAP_ORDER {
            for rule in maps.get(map_name).unwrap() {
                if let Some(result) = rule.eval(cur_val) {
                    new_val = Some(result);
                }
            }
            if let Some(new_val) = new_val {
                cur_val = new_val;
            }
        }

        result.push(cur_val);
    }

    println!("{}", result.iter().min().unwrap());
}

const MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Debug)]
struct Rule {
    destination_start: usize,
    source_range: Range<usize>,
}

impl Rule {
    fn eval(&self, value: usize) -> Option<usize> {
        if self.source_range.contains(&value) {
            Some(self.destination_start + value - self.source_range.start)
        } else {
            None
        }
    }
}
