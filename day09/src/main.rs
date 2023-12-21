fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let lines = include_str!("input.txt").lines();

    lines
        .map(|line| {
            next_in_seq(
                &line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .sum()
}

fn part2() -> i32 {
    let lines = include_str!("input.txt").lines();

    lines
        .map(|line| {
            let mut rev_line = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            rev_line.reverse();
            next_in_seq(&rev_line)
        })
        .sum()
}

fn next_in_seq(seq: &[i32]) -> i32 {
    if all_zeroes(seq) {
        0
    } else {
        let mut reduced = Vec::new();
        for i in 1..seq.len() {
            reduced.push(seq[i] - seq[i - 1]);
        }
        seq[seq.len() - 1] + next_in_seq(&reduced)
    }
}

fn all_zeroes(seq: &[i32]) -> bool {
    for i in seq {
        if *i != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        assert_eq!(1584748274, part1());
    }

    #[test]
    fn pt2() {
        assert_eq!(1026, part2());
    }
}
