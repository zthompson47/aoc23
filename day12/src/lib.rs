#![deny(elided_lifetimes_in_paths)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
struct Context<'a> {
    cache: Cache,
    string: &'a [char],
    groups: Vec<usize>,
}

type Cache = Arc<Mutex<HashMap<(usize, usize), VecDeque<VecDeque<usize>>>>>;

fn partition(tokens: usize, buckets: usize, context: &Context<'_>) -> VecDeque<VecDeque<usize>> {
    let mut result = VecDeque::new();
    //println!("{:?}", context.cache.lock().unwrap().keys().len());

    //println!("{:?}", context);

    if buckets == 1 {
        result.push_back(VecDeque::from(vec![VecDeque::from(vec![tokens])]));
    } else {
        //let st = context.string.as_str().chars().collect::<Vec<char>>();
        let st = context.string;
        for i in 0..=tokens {
            //println!(
            //    "tok {}, buc {}, str {}, grp {:?}",
            //    i, buckets, context.string, context.groups
            //);
            //println!("   ln is {}, i is {}", st.len(), i);

            // Check that the spaces don't cover a pipe.
            if st[..i].contains(&'#') {
                //println!("   #");
                continue;
            }

            let group_len = context.groups[(context.groups.len() + 1) - buckets];
            //println!("   _____group_len__: {}", group_len);

            // Check that the following pipes don't cover a space.
            if group_len >= i && st[i..group_len].contains(&'.') {
                //println!("  .");
                continue;
            }

            // Check that there's ground after the last pipe.
            if i + group_len < st.len() && st[i + group_len] == '#' {
 //               println!("   #(end)");
                continue;
            }

/*            // Check that the solution will still fit.
            if i + group_len
                + 1
                + (context.groups[1..].iter().sum::<usize>() - 1) // ??
                + context.groups[1..].len()
                > st.len()
            {
                println!("OVERFLOW {:?}", context.string);
                println!("OVERFLOW {:?}", st);
                println!("OVERFLOW {:?}", context.groups);
                println!(
                    "OVERFLOW {} {}",
                    i + context.groups.iter().sum::<usize>() - 1,
                    st.len()
                );
                continue;
            }
*/
            let end = if i + group_len < st.len() {
                group_len + 1
            } else {
                group_len
            };

            let string = &st[i + end..];

            let context = Context {
                cache: context.cache.clone(),
                string,
                groups: context.groups.clone(),
            };

            let partition = partition(tokens - i, buckets - 1, &context)
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

pub enum Part {
    One,
    Two,
}

pub fn run(part: Part) -> Vec<usize> {
    let cache = Arc::new(Mutex::new(HashMap::new()));
    let rows = include_str!("input.orig")
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

            match part {
                Part::One => {
                    let context = Context {
                        cache: cache.clone(),
                        string: &string.chars().collect::<Vec<char>>(),
                        groups: groups.clone(),
                    };
                    acc.push(PipeRow::new(string, groups, context));
                    acc
                }
                Part::Two => {
                    let a = String::from(string);
                    let string =
                        [a.as_str(), a.as_str(), a.as_str(), a.as_str(), a.as_str()].join("?");
                    let string = string.as_str();
                    let groups = [
                        groups.as_slice(),
                        groups.as_slice(),
                        groups.as_slice(),
                        groups.as_slice(),
                        groups.as_slice(),
                    ]
                    .concat();

                    let context = Context {
                        cache: cache.clone(),
                        //string: string.to_string(),
                        string: &string.chars().collect::<Vec<char>>(),
                        groups: groups.clone(),
                    };
                    acc.push(PipeRow::new(string, groups, context));
                    acc
                }
            }
        });
    let mut max = (0, 0);
    let mut _result = 0;
    let mut result = Vec::new();

    for row in rows {
        //println!("@@@ {} {} {}", row.space, row.groups.len(), row.string);
        if row.space > max.0 {
            max.0 = row.space;
        }
        if row.groups.len() > max.1 {
            max.1 = row.groups.len();
        }

        let mut row_count = 0;
        for partition in &row.partitions {
            let is_solution = row.is_solution(partition);
            if is_solution {
                _result += 1;
                row_count += 1;
            }
        }
        println!("{row_count} {} {} {}", row.space, row.groups.len(), row.string);
        result.push(row_count);
    }

    //println!("!!!!!!!!!!!!___?????______{:?}", max);
    //_result;
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
    fn new(string: &str, groups: Vec<usize>, context: Context<'_>) -> Self {
        //println!("______piperow: {} {:?}", string, groups);
        let space = string.len() - groups.iter().sum::<usize>() - (groups.len() - 1);
        let partitions = partition(space, groups.len() + 1, &context);

        //println!("cache len: {}", context.cache.lock().unwrap().keys().len());

        //println!("::::::::::::::>>> {:?}", partitions);

        PipeRow {
            string: String::from(string),
            groups,
            partitions,
            space,
        }
    }

    fn is_solution(&self, partition: &VecDeque<usize>) -> bool {
        //println!(">>is_solution<< {partition:?}");
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
pub fn part1() -> usize {
    let rows = include_str!("input.orig")
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
