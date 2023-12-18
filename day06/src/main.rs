use std::str::Lines;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut lines = include_str!("input.txt").lines();
    let times = next_vec(&mut lines);
    let distances = next_vec(&mut lines);
    assert!(times.len() == distances.len());

    times
        .into_iter()
        .zip(distances)
        .fold(1, |acc, (race_duration, distance_record)| {
            possible_wins(race_duration, distance_record) * acc
        })
}

fn part2() -> usize {
    let mut lines = include_str!("input.txt").lines();
    let race_duration = next_num(&mut lines);
    let distance_record = next_num(&mut lines);

    possible_wins(race_duration, distance_record)
}

fn possible_wins(race_duration: usize, distance_record: usize) -> usize {
    let mut possible_wins = 0;

    for i in 0..race_duration {
        let speed = i;
        let distance = (race_duration - speed) * speed;

        if distance > distance_record {
            possible_wins += 1;
        }
    }

    possible_wins
}

fn next_vec(lines: &mut Lines<'_>) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn next_num(lines: &mut Lines<'_>) -> usize {
    lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}
