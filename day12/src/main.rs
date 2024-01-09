use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

fn main() {
    //println!("Part 0: {}", part00());
    //println!("{:#?}", partition(4, 5));
    for row in partition(3, 3) {
        println!("{:?}", row);
    }
}

fn partition(tokens: usize, buckets: usize) -> VecDeque<VecDeque<usize>> {
    //println!("call: ({}, {})", tokens, buckets);
    let mut result = VecDeque::new();

    if buckets == 1 {
        //println!("_______--X--{}__________", tokens);
        result.push_back(VecDeque::from(vec![VecDeque::from(vec![tokens])]));
        return result.into_iter().flatten().collect();
    }

    for i in 0..=tokens {
        //if buckets == 1 {
        //    println!("_______--X--{}__________", tokens);
        //    result.push_back(VecDeque::from(vec![VecDeque::from(vec![tokens])]));
        //} else {
        //println!("_______recurse__________");
        result.push_back(
            partition(tokens - i, buckets - 1)
                .into_iter()
                .map(|mut x| {
                    x.push_front(i);
                    x
                })
                .collect(),
        );
        //}
    }
    //println!("end: ({}, {})", tokens, buckets);
    result.into_iter().flatten().collect()
    //result
}

fn part00() -> usize {
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
            acc.push(PipeRow::new(string, groups));
            acc
        });

    for row in rows {
        println!("{row:?}");
        for bla in row.solutions() {
            println!("bla: {bla}");
        }
    }

    0
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
    space: usize,
}

impl PipeRow {
    fn new(string: &str, groups: Vec<usize>) -> Self {
        let space = string.len() - groups.iter().sum::<usize>() - (groups.len() - 1);
        PipeRow {
            string: String::from(string),
            groups,
            space,
        }
    }

    fn solutions(&self) -> PossibleGroupings<'_> {
        PossibleGroupings::new(self)
    }
}

struct PossibleGroupings<'a> {
    row: &'a PipeRow,
    index: usize,
}

impl<'a> PossibleGroupings<'a> {
    fn new(pipe_row: &'a PipeRow) -> Self {
        PossibleGroupings {
            row: pipe_row,
            index: 0,
        }
    }
}

impl Iterator for PossibleGroupings<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        match index {
            5 => None,
            _ => Some(String::from("42")),
        }
    }
}

fn part0() -> usize {
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

            // Part 1
            //acc.push((status, groups));
            //acc
        });

    let mut solutions = HashSet::new();
    let mut solutions_with_dupes = Vec::new();
    let rows_len = rows.len();
    //let mut i = 0;

    for (i, (inner, groups)) in rows.as_slice().iter().enumerate() {
        let regex = groups_regex(groups);
        println!("{i}/{rows_len} {regex}");
        let inner = inner.clone();
        for possible_solution in Permutate::new(inner) {
            if is_solution(&possible_solution, &regex) {
                if !solutions.insert((i, possible_solution.clone())) {
                    //println!("_DUPE_");
                } else {
                    println!("Y - {possible_solution} {groups:?}");
                }
                solutions_with_dupes.push(possible_solution);
            } else {
                println!("N - {possible_solution} {groups:?}");
            }
        }
        //i += 1;
    }

    println!("------>> {}", solutions_with_dupes.len());

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
        println!("{}", self.inner);
        println!("!!!!!!! {}", self.inner.len());
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
        assert_eq!(7694, part0());
    }
}
