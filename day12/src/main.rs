use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

fn main() {
    //println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());

    /*
    let cache = Arc::new(Mutex::new(HashMap::new()));
    for i in 1..15 {
        for j in 1..15 {
            let _ = partition(i, j, cache.clone());
        }
    }
    */

    //println!("{:?}", part);
    //println!("part: {}", part.len());
    //println!("cache: {}", cache.lock().unwrap().keys().len());
}

#[derive(Clone)]
struct Context {
    cache: Cache,
    string: String,
}

type Cache = Arc<Mutex<HashMap<(usize, usize), VecDeque<VecDeque<usize>>>>>;

fn partition(tokens: usize, buckets: usize, context: &Context) -> VecDeque<VecDeque<usize>> {
    let mut result = VecDeque::new();
    //println!("{:?}", context.cache.lock().unwrap().keys().len());

    if buckets == 1 {
        result.push_back(VecDeque::from(vec![VecDeque::from(vec![tokens])]));
    } else {
        for i in 0..=tokens {
            let partition = partition(tokens - i, buckets - 1, context)
                .into_iter()
                .map(|mut x| {
                    x.push_front(i);
                    x
                })
                .collect::<VecDeque<VecDeque<usize>>>();

            context
                .cache
                .lock()
                .unwrap()
                .insert((tokens, buckets), partition.clone());

            result.push_back(partition);
        }
    }

    result.into_iter().flatten().collect()
}

fn part2() -> usize {
    let cache = Arc::new(Mutex::new(HashMap::new()));
    let rows = include_str!("input.txt")
        .lines()
        .fold(Vec::new(), |mut acc, row| {
            let mut split = row.split_ascii_whitespace();
            let string = split.next().unwrap();
            let groups = split
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let a = String::from(string);
            let b = [a.as_str(), a.as_str(), a.as_str(), a.as_str(), a.as_str()].join("?");
            let g = [
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
            ]
            .concat();

            let context = Context {
                cache: cache.clone(),
                string: string.to_string(),
            };
            acc.push(PipeRow::new(&b, g, context));
            acc

            //acc.push(PipeRow::new(string, groups, cache.clone()));
            //acc
        });
    let mut max = (0, 0);
    let mut result = 0;

    for row in rows {
        println!("@@@ {} {} {}", row.space, row.groups.len(), row.string);
        if row.space > max.0 {
            max.0 = row.space;
        }
        if row.groups.len() > max.1 {
            max.1 = row.groups.len();
        }

        for partition in &row.partitions {
            let is_solution = row.is_solution(partition);
            if is_solution {
                result += 1;
            }
        }
    }

    println!("{:?}", max);
    result
}

#[derive(Debug)]
/// ?????#?????????.??? 4,5,2,2
/// XXXX.XXXXX....XX.XX
///
/// a b c
/// 3 0 0
/// 2 1 0
/// 2 0 1
/// 1 2 0
/// 1 1 1
/// 1 0 2
/// 0 3 0
/// 0 2 1
/// 0 1 2
/// 0 0 3
struct PipeRow {
    string: String,
    groups: Vec<usize>,
    partitions: VecDeque<VecDeque<usize>>,
    space: usize,
}

impl PipeRow {
    fn new(string: &str, groups: Vec<usize>, context: Context) -> Self {
        let space = string.len() - groups.iter().sum::<usize>() - (groups.len() - 1);
        let partitions = partition(space, groups.len() + 1, &context);

        println!("cache len: {}", context.cache.lock().unwrap().keys().len());

        PipeRow {
            string: String::from(string),
            groups,
            partitions,
            space,
        }
    }

    fn is_solution(&self, partition: &VecDeque<usize>) -> bool {
        //println!("{partition:?}");
        let mut partition = partition.iter();
        let mut solution = vec!['.'; self.string.len()];
        let mut index = *partition.next().unwrap();

        for _ in 0..self.groups[0] {
            solution[index] = '#';
            index += 1;
        }
        index += 1;

        partition.zip(self.groups[1..].iter()).for_each(|(p, g)| {
            for _ in 0..*p {
                solution[index] = '.';
                index += 1;
            }
            for _ in 0..*g {
                solution[index] = '#';
                index += 1;
            }
            index += 1;
        });

        //println!("ground: {}", self.string);
        //println!("signal: {}", solution.iter().collect::<String>());

        for (solution, string) in solution.iter().zip(self.string.chars()) {
            match string {
                '#' => {
                    if *solution != '#' {
                        return false;
                    }
                }
                '.' => {
                    if *solution != '.' {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }
}

#[allow(unused)]
fn part1() -> usize {
    let rows = include_str!("input.txt")
        .lines()
        .fold(Vec::new(), |mut acc, row| {
            let mut split = row.split_ascii_whitespace();
            let status = split.next().unwrap();
            let groups = split
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            // Part 2
            /*
            let a = String::from(status);
            let b = [a.as_str(), a.as_str(), a.as_str(), a.as_str(), a.as_str()].join("?");
            let g = [
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
            ]
            .concat();
            acc.push((b.clone(), g.clone()));
            acc
            */

            // Part 1
            acc.push((status, groups));
            acc
        });

    let mut solutions = HashSet::new();
    let mut solutions_with_dupes = Vec::new();

    for (i, (inner, groups)) in rows.as_slice().iter().enumerate() {
        let regex = groups_regex(groups);
        for possible_solution in Permutate::new(inner.to_string()) {
            if is_solution(&possible_solution, &regex) {
                solutions.insert((i, possible_solution.clone()));
                solutions_with_dupes.push(possible_solution);
            }
        }
    }

    //println!("------>> {}", solutions_with_dupes.len());
    solutions.len()
}

fn groups_regex(groups: &[usize]) -> regex::Regex {
    let mut result = String::from("^");
    for (i, group) in groups.iter().enumerate() {
        result.push_str("[?.]*[?#]{");
        let num = format!("{group}");
        result.push_str(num.as_str());
        if i < groups.len() - 1 {
            result.push_str("}[?.]");
        } else {
            result.push_str("}[?.]*$");
        }
    }
    //let result = format!(r"([?#]{{{}}}[?.])", groups[0]);
    //println!("-->> {} <<--", result);

    regex::Regex::new(&result).unwrap()
}

fn is_solution(solution: &str, regex: &regex::Regex) -> bool {
    if let Some(m) = regex.find(solution) {
        !m.is_empty()
    } else {
        false
    }
}

struct Permutate {
    inner: String,
    count: usize,
}

impl Permutate {
    fn new(inner: String) -> Self {
        Permutate { inner, count: 0 }
    }
}

impl Iterator for Permutate {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        //println!("{}", self.inner);
        //println!("!!!!!!! {}", self.inner.len());
        if (self.count as u64) < 2u64.pow(self.inner.len() as u32) {
            let mask = format!("{:01$b}", self.count, self.inner.len());

            let result = mask
                .chars()
                .zip(self.inner.chars())
                .map(|(mask, inner)| match inner {
                    '?' => match mask {
                        '0' => '.',
                        '1' => '#',
                        _ => panic!(),
                    },
                    c => c,
                })
                .collect::<String>();

            self.count += 1;

            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_1() {
        assert_eq!(7694, part1());
    }
}
