use std::{collections::HashMap, str::Chars};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let (labels, locations, paths, _) = process_input();
    let mut paths = paths.cycle();

    let mut index = labels["AAA"];
    let mut steps = 0;

    while index != labels["ZZZ"] {
        index = match paths.next().unwrap() {
            'L' => locations[index].0,
            'R' => locations[index].1,
            _ => panic!(),
        };
        steps += 1;
    }

    steps
}

/*
fn _part2() -> usize {
    let (labels, locations, paths) = process_input();
    let mut paths = paths.cycle();

    let endpoints = labels
        .iter()
        .filter_map(|(k, v)| if k.ends_with('Z') { Some(v) } else { None })
        .copied()
        .collect::<Vec<usize>>();
    let mut startpoints = labels
        .iter()
        .filter_map(|(k, v)| if k.ends_with('A') { Some(v) } else { None })
        .copied()
        .collect::<Vec<usize>>();

    assert_eq!(endpoints.len(), startpoints.len());

    println!("{endpoints:#?}");
    println!("{startpoints:#?}");

    let mut steps = 0;

    while !is_end(&startpoints, &endpoints) {
        let direction = paths.next().unwrap();

        for point in startpoints.iter_mut() {
            *point = match direction {
                'L' => locations[*point].0,
                'R' => locations[*point].1,
                _ => panic!(),
            };
        }
        steps += 1;
    }

    steps
}

fn __part2() -> usize {
    let (labels, locations, path, label_list) = process_input();

    // Where one cycle of the path ends up for each starting point.
    let mut path_transitions: HashMap<&str, &str> = HashMap::new();

    // Indices of ending points within a path.
    let mut path_endpoints: HashMap<&str, Vec<usize>> = HashMap::new();

    for (i, label) in label_list.iter().enumerate() {
        let mut next_index = i;

        println!("{:?}", path.clone());
        for (step, direction) in path.clone().enumerate() {
            //println!("--step,direction {step},{direction}");
            let label = label_list[next_index];
            println!("{label}");

            if label.ends_with('Z') {
                println!("zz -->>>  {label}");
                path_endpoints
                    .entry(label)
                    .and_modify(|x| x.push(step))
                    .or_insert(vec![step]);
            }

            next_index = match direction {
                'L' => locations[next_index].0,
                'R' => locations[next_index].1,
                _ => panic!(),
            };
        }
        println!("\n\n");

        path_transitions.insert(label, label_list[next_index]);
    }

    println!("{path_endpoints:#?}");
    println!("{path_transitions:#?}");

    0
}
*/

fn part2() -> usize {
    let (labels, locations, directions, label_list) = process_input();

    //println!(
    //    "diretions: {}",
    //    directions.clone().collect::<Vec<char>>().len()
    //);

    let startpoints = labels
        .iter()
        .filter_map(|(k, v)| if k.ends_with('A') { Some(v) } else { None })
        .copied()
        .collect::<Vec<usize>>();
    //let startpoints = (0..labels.len()).collect::<Vec<usize>>();

    let endpoints = labels
        .iter()
        .filter_map(|(k, v)| if k.ends_with('Z') { Some(v) } else { None })
        .copied()
        .collect::<Vec<usize>>();

    let mut shortcuts = HashMap::new();
    let mut passed_ends: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();

    for (i, label) in label_list.iter().enumerate() {
        let mut destination = label;
        let mut location = i;

        for (j, direction) in directions.clone().enumerate() {
            //println!("{destination}");
            location = match direction {
                'L' => locations[location].0,
                'R' => locations[location].1,
                _ => panic!(),
            };
            destination = &label_list[location];
            if destination.ends_with('Z') {
                passed_ends
                    .entry(label)
                    .and_modify(|x| x.push((destination, j)))
                    .or_insert(vec![(destination, j)]);
            }
        }
        shortcuts.insert(label, destination);
    }

    //println!("shortcuts: {shortcuts:#?}");
    //println!("passed_ends: {passed_ends:#?}");

    //assert_eq!(startpoints.len(), endpoints.len());

    //println!("endpoints: {endpoints:#?}");
    //println!("startpoints: {startpoints:#?}");

    let mut result = Vec::new();
    let _len = directions.clone().collect::<Vec<_>>().len();

    let _label_endsteps: HashMap<&str, usize> = HashMap::default();

    for start in startpoints.iter() {
        let mut location = *start;

        for (i, direction) in directions.clone().cycle().enumerate() {
            //if i % len == 0 {
            //    println!("label: {}", label_list[location]);
            //}
            if endpoints.contains(&location) {
                result.push(i);
                //println!("in endpoints: {} {i}", label_list[location]);
                break;
            }
            location = match direction {
                'L' => locations[location].0,
                'R' => locations[location].1,
                _ => panic!(),
            };
        }
        //println!("-----------------------");
    }

    //println!("{result:#?}");

    lcmx::lcmx(&result).unwrap()

    /*
    let mut steps = 0;
    while !is_end(&startpoints, &endpoints) {
        startpoints.iter_mut().for_each(|point| {
            let label = label_list[*point];
            let shortcut = *shortcuts[&label];
            *point = labels[shortcut];
        });
        steps += 1;
        if steps % 100000 == 0 {
            println!(
                "{steps} {:?}",
                startpoints
                    .iter()
                    .map(|x| label_list[*x])
                    .collect::<Vec<&str>>()
            );
        }
    }
    steps
    */
}

fn _is_end(startpoints: &[usize], endpoints: &[usize]) -> bool {
    //println!("{startpoints:?} {endpoints:?}");
    for x in startpoints {
        if !endpoints.contains(x) {
            return false;
        }
    }

    true
}

type Labels = HashMap<&'static str, usize>;
type Locations = Vec<(usize, usize)>;
type Directions = Chars<'static>;
type LabelList = Vec<&'static str>;

fn process_input() -> (Labels, Locations, Directions, LabelList) {
    let mut lines = include_str!("input.txt").lines();
    let path = lines.next().unwrap();
    let _ = lines.next();

    let mut labels = HashMap::new();
    let mut locations = Vec::new();
    let mut label_list = Vec::new();

    for (i, line) in lines.enumerate() {
        let mut split = line.split('=');
        let label = split.next().unwrap().trim();

        labels.insert(label, i);
        label_list.push(label);

        let mut location = split
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

    let path = path.chars();

    (labels, locations, path, label_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(16697, part1());
    }

    #[test]
    fn part2_answer() {
        assert_eq!(10668805667831, part2());
    }
}
