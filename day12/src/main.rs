use std::cmp::Ordering;

fn main() {
    println!("Part 0: {}", part0());
    //println!("Part 1: {}", part1());
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
            acc.push((status, groups));
            acc
        });

    for (inner, groups) in rows.as_slice() {
        let iter = ConditionStr::new(inner, groups);
        let regex = groups_regex(groups);
        for (possible_solution, groups) in iter {
            if is_solution(&possible_solution, &regex) {
                //println!("Y - {possible_solution} {groups:?}");
            } else {
                println!("N - {possible_solution} {groups:?}");
            }
        }
    }

    0
}

struct ConditionStr<'a> {
    inner: &'static str,
    groups: &'a [usize],
    count: usize,
}

fn groups_regex(groups: &[usize]) -> regex::Regex {
    let result = format!(r"([?#]{{{}}}[?.])", groups[0]);
    regex::Regex::new(&result).unwrap()
}

fn is_solution(solution: &str, regex: &regex::Regex) -> bool {
    if let Some(m) = regex.find(solution) {
        !m.is_empty()
    } else {
        false
    }
}

impl<'a> ConditionStr<'a> {
    fn new(inner: &'static str, groups: &'a [usize]) -> Self {
        ConditionStr {
            inner,
            groups,
            count: 0,
        }
    }
}

impl Iterator for ConditionStr<'_> {
    type Item = (String, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 2usize.pow(self.inner.len() as u32) {
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

            Some((result, self.groups.to_vec()))
        } else {
            None
        }
    }
}

fn split_min(input: &str, size: usize) -> (&str, &str) {
    let re = format!(r"([?#]{{{size}}}[?.])");
    let re = regex::Regex::new(&re).unwrap();
    let m = re.find(input).unwrap();
    input.split_at(m.end())
}

fn _split_min(input: &str, size: usize) -> (&str, &str) {
    let mut index = 0;
    let mut count = 0;
    for (i, ch) in input.chars().enumerate() {
        println!("{i} {ch}");
        match ch {
            '?' => match count.cmp(&size) {
                Ordering::Less => count += 1,
                Ordering::Equal => {
                    index = i;
                    break;
                }
                Ordering::Greater => panic!(),
            },
            '#' => match count.cmp(&size) {
                Ordering::Less => count += 1,
                Ordering::Equal => count = 0,
                Ordering::Greater => panic!(),
            },

            '.' => match count.cmp(&size) {
                Ordering::Less => count = 0,
                Ordering::Equal => {
                    index = i + 1;
                    break;
                }
                Ordering::Greater => panic!(),
            },
            _ => panic!(),
        }
        println!("cnt: {count}");
    }
    println!("idx: {index}");

    input.split_at(index)
}

fn part1() -> usize {
    let conditions = include_str!("input.txt")
        .lines()
        .map(Condition::from)
        .collect::<Vec<_>>();

    dbg!(conditions);

    0
}

#[derive(Debug, Default)]
struct Condition {
    inner: &'static str,
    status: Vec<Status>,
    groups: Vec<usize>,
}

impl Condition {
    fn take_min(&self) -> (&Self, &Self) {
        let len = self.groups[0];
        let mut iter = self.status.iter().copied();

        let mut first = Condition::default();
        let mut rest = Condition::default();

        let mut front = iter
            .take_while(|x| [Status::Unknown, Status::Damaged].contains(x))
            .collect::<Vec<_>>();

        while front.len() < len {
            first.status.append(&mut front);
        }

        (self, self)
    }
}

impl From<&'static str> for Condition {
    fn from(inner: &'static str) -> Self {
        let mut parts = inner.split_ascii_whitespace();
        let status = parts
            .next()
            .unwrap()
            .chars()
            .map(Status::from)
            .collect::<Vec<_>>();
        let groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            inner,
            status,
            groups,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    Unknown,
    Damaged,
    Operational,
}

impl From<char> for Status {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '#' => Self::Damaged,
            '.' => Self::Operational,
            _ => panic!(),
        }
    }
}
