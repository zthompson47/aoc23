use std::collections::HashSet;

fn main() {
    println!("Part 0: {}", part0());
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

    let mut solutions = HashSet::new();
    let mut solutions_with_dupes = Vec::new();
    let rows_len = rows.len();
    //let mut i = 0;

    for (i, (inner, groups)) in rows.as_slice().iter().enumerate() {
        let regex = groups_regex(groups);
        println!("{i}/{rows_len} {regex}");
        for possible_solution in Permutate::new(inner) {
            if is_solution(&possible_solution, &regex) {
                if !solutions.insert((i, possible_solution.clone())) {
                    //println!("_DUPE_");
                } else {
                    println!("Y - {possible_solution} {groups:?}");
                }
                solutions_with_dupes.push(possible_solution);
            } else {
                //println!("N - {possible_solution} {groups:?}");
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
    inner: &'static str,
    count: usize,
}

impl Permutate {
    fn new(inner: &'static str) -> Self {
        Permutate { inner, count: 0 }
    }
}

impl Iterator for Permutate {
    type Item = String;

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
