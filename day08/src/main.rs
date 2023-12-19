use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let mut lines = include_str!("input.txt").lines();
    let path = lines.next().unwrap();
    let _ = lines.next();

    let mut labels = HashMap::new();
    let mut locations = Vec::new();

    for (i, line) in lines.enumerate() {
        let mut sides = line.split('=');
        let label = sides.next().unwrap().trim();

        labels.insert(label, i);

        let mut location = sides
            .next()
            .unwrap()
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
            .map(|x| x.trim());
        let l = location.next().unwrap();
        let r = location.next().unwrap();

        locations.push((l, r));
    }

    let locations = locations
        .into_iter()
        .map(|(l, r)| (labels[l], labels[r]))
        .collect::<Vec<(usize, usize)>>();

    //println!("{labels:#?}");
    //println!("{locations:#?}");

    let mut path = path.chars().cycle();
    let mut index = labels["AAA"];
    let mut steps = 0;

    while index != labels["ZZZ"] {
        index = match path.next().unwrap() {
            'L' => locations[index].0,
            'R' => locations[index].1,
            _ => panic!(),
        };
        steps += 1;
    }

    steps
}
