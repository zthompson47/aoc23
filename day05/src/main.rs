use std::{collections::HashMap, ops::Range, str::Lines};

const MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn main() {
    let lines = include_str!("input.txt").lines();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines));
}

fn part1(mut lines: Lines<'static>) -> usize {
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let maps = build_maps(lines);
    let mut result = Vec::new();

    for seed in seeds.clone() {
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

    *result.iter().min().unwrap()
}

fn part2(mut lines: Lines<'static>) -> usize {
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let maps = build_maps(lines);
    let mut result = Vec::new();

    for seed_range in seeds.chunks(2).map(|r| r[0]..r[0] + r[1]) {
        //println!("{seed_range:?}");
        let mut prior_ranges = vec![seed_range.clone()];
        let mut next_ranges = Vec::new();

        for map_name in MAP_ORDER {
            //println!("  {map_name}");
            for rule in maps.get(map_name).unwrap() {
                for prior_range in &prior_ranges {
                    if let Some(result) = rule.eval_range(prior_range.clone()) {
                        //next_ranges = result;
                        next_ranges.push(result);
                        //println!("!! -->>  {next_ranges:?}");
                    }
                }
            }
            //println!("__set cur_range");
            prior_ranges = next_ranges.clone();
            next_ranges.clear();
        }

        result.push(prior_ranges);
    }

    //println!("{:?}", result);

    result.iter().flatten().map(|r| r.start).min().unwrap()
}

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

    fn eval_range(&self, input: Range<usize>) -> Option<Range<usize>> {
        if input.start < self.source_range.end && input.end > self.source_range.start {
            let source_start = input.start.max(self.source_range.start);
            let source_end = input.end.min(self.source_range.end);
            let index = source_start - self.source_range.start;
            let size = source_end - source_start;
            let dest_start = self.destination_start + index;
            let dest_end = dest_start + size;

            //println!("    source_range: {:?}", self.source_range);
            //println!("    dest_start: {}", self.destination_start);
            //println!("    ss: {source_start}, se: {source_end}, idx: {index}, sz: {size}\n    ds: {dest_start}, de: {dest_end}");

            Some(dest_start..dest_end)
        } else {
            None
        }
    }
}

fn build_maps(lines: Lines<'static>) -> HashMap<&'static str, Vec<Rule>> {
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

    maps
}
