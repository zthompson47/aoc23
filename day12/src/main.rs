use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn partition(tokens: usize, buckets: usize) -> VecDeque<VecDeque<usize>> {
    let mut result = VecDeque::new();
    if buckets == 1 {
        result.push_back(VecDeque::from(vec![VecDeque::from(vec![tokens])]));
        return result.into_iter().flatten().collect();
    }
    for i in 0..=tokens {
        result.push_back(
            partition(tokens - i, buckets - 1)
                .into_iter()
                .map(|mut x| {
                    x.push_front(i);
                    x
                })
                .collect(),
        );
    }
    result.into_iter().flatten().collect()
}

fn part2() -> usize {
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

    let mut result = 0;
    for row in rows {
        //println!("groups: {:?}", row.groups);
        for partition in &row.partitions {
            //println!("partition: {:?}", partition);
            let is_solution = row.is_solution(partition);
            //println!("________{}_____________", is_solution);
            if is_solution {
                result += 1;
            }
        }
    }
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
}

impl PipeRow {
    fn new(string: &str, groups: Vec<usize>) -> Self {
        let space = string.len() - groups.iter().sum::<usize>() - (groups.len() - 1);
        let partitions = partition(space, groups.len() + 1);
        PipeRow {
            string: String::from(string),
            groups,
            partitions,
        }
    }

    //fn solutions(&self) -> PossibleGroupings<'_> {
    //    PossibleGroupings::new(self)
    //}

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

/*
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
*/

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
    //let rows_len = rows.len();
    //let mut i = 0;

    for (i, (inner, groups)) in rows.as_slice().iter().enumerate() {
        let regex = groups_regex(groups);
        //println!("{i}/{rows_len} {regex}");
        //let inner = inner.clone();
        for possible_solution in Permutate::new(inner.to_string()) {
            if is_solution(&possible_solution, &regex) {
                if !solutions.insert((i, possible_solution.clone())) {
                    //println!("_DUPE_");
                } else {
                    //println!("Y - {possible_solution} {groups:?}");
                }
                solutions_with_dupes.push(possible_solution);
            } else {
                //println!("N - {possible_solution} {groups:?}");
            }
        }
        //i += 1;
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
