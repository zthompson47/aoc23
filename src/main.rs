#![allow(clippy::needless_range_loop, clippy::let_and_return, unused)]
use std::{collections::HashMap, fmt::Display};

fn main() {
    let mut springs: Vec<Row> =
        include_str!("input.txt")
            .lines()
            .fold(Vec::new(), |mut springs, row| {
                let (map, conditions) = row.split_once(' ').unwrap();
                let row = Row {
                    map: map.chars().map(Spring::from).collect(),
                    conditions: conditions
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect(),
                };
                springs.push(row);
                springs
            });
    let part1: usize = springs
        .iter()
        .map(|x| {
            let count = x.possibilities();
            println!("{count}: {x}");
            count
        })
        .sum();
    println!("Part 1: {part1}");

    springs = springs
        .iter_mut()
        .map(|row| {
            let mut new_map = Vec::new();
            let mut new_conditions = Vec::new();
            for _ in 0..5 {
                new_map.push(row.map.clone());
                new_conditions.push(row.conditions.clone());
            }
            let new_map = new_map.join(&Spring::Unknown);

            Row {
                map: new_map,
                conditions: new_conditions.into_iter().flatten().collect(),
            }
        })
        .collect();
    let part2: usize = springs
        .iter()
        .map(|x| {
            //println!("trying: {x}");
            let count = x.possibilities();
            println!("{count}: {x}");
            count
        })
        .sum();
    println!("Part 2: {part2}");
}

fn test() {
    let s = "??#??";
    let map = Spring::vec_from_str(s);
    assert_eq!(3, Row::possibilities_from_condition(&map, 3));
    assert_eq!(2, Row::possibilities_from_condition(&map, 2));
    assert_eq!(1, Row::possibilities_from_condition(&map, 1));
    let s = ".....?#?#??";
    let map = Spring::vec_from_str(s);
    assert_eq!(1, Row::possibilities_from_condition(&map, 3));
    assert_eq!(2, Row::possibilities_from_condition(&map, 4));
    assert_eq!(0, Row::possibilities_from_condition(&map, 2));
    let s = "...???#?#";
    let map = Spring::vec_from_str(s);
    assert_eq!(1, Row::possibilities_from_condition(&map, 3));
    let s = "...???.?.";
    let map = Spring::vec_from_str(s);
    assert_eq!(1, Row::possibilities_from_condition(&map, 3));
    assert_eq!(4, Row::possibilities_from_condition(&map, 1));
    assert_eq!(2, Row::possibilities_from_condition(&map, 2));
    let s = "?????????";
    let map = Spring::vec_from_str(s);
    assert_eq!(7, Row::possibilities_from_condition(&map, 3));
    assert_eq!(8, Row::possibilities_from_condition(&map, 2));
    assert_eq!(2, Row::possibilities_from_condition(&map, 8));
    assert_eq!(1, Row::possibilities_from_condition(&map, 9));
    let s = "###";
    let map = Spring::vec_from_str(s);
    assert_eq!(1, Row::possibilities_from_condition(&map, 3));
    assert_eq!(0, Row::possibilities_from_condition(&map, 2));
    assert_eq!(0, Row::possibilities_from_condition(&map, 1));
    assert_eq!(0, Row::possibilities_from_condition(&map, 4));
    let s = "?#????";
    let map = Spring::vec_from_str(s);
    assert_eq!(1, Row::possibilities_from_condition(&map, 1));
    let s = "#?.???";
    let map = Spring::vec_from_str(s);
    assert_eq!(1, Row::possibilities_from_condition(&map, 1));

    let mut cache = HashMap::new();

    let s = "?#????????????##??";
    let map = Spring::vec_from_str(s);
    assert_eq!(
        9,
        Row::possibilities_from_conditions(&map, &[1, 1, 10], &mut cache)
    );
    let s = "?.???.?#???#???.?";
    let map = Spring::vec_from_str(s);
    assert_eq!(
        5,
        Row::possibilities_from_conditions(&map, &[3, 6, 1], &mut cache)
    );
    let s = ".???#?#?..??";
    let map = Spring::vec_from_str(s);
    assert_eq!(
        2,
        Row::possibilities_from_conditions(&map, &[4, 1, 1], &mut cache)
    );
    let s = "???.#????#.?";
    let map = Spring::vec_from_str(s);
    assert_eq!(
        12,
        Row::possibilities_from_conditions(&map, &[1, 1, 1, 1], &mut cache)
    );
    let s = "?##?????#.#..??.?";
    let map = Spring::vec_from_str(s);
    assert_eq!(
        7,
        Row::possibilities_from_conditions(&map, &[3, 3, 1, 1, 1], &mut cache)
    );
}

impl Row {
    fn possibilities(&self) -> usize {
        let mut cache = HashMap::new();
        Row::possibilities_from_conditions(&self.map, &self.conditions, &mut cache)
    }

    fn possibilities_from_condition(map: &[Spring], condition: usize) -> usize {
        let rr = Row {
            map: map.to_vec(),
            conditions: vec![condition],
        };
        //println!("_____single___________ {rr}");

        let mut result = 0;
        let mut start = 0;
        let first_damaged = map.iter().position(|x| *x == Spring::Damaged);
        let last_damaged = map.iter().rposition(|x| *x == Spring::Damaged);
        //println!("{first_damaged:?} {last_damaged:?}");

        loop {
            //println!("--start:{start}");
            let possibility = map.iter().skip(start).enumerate().find(|(i, _)| {
                if let Some(first_damaged) = first_damaged {
                    // Need to include first damaged spring in match.
                    if first_damaged < start + i {
                        return false;
                    }
                }
                if let Some(last_damaged) = last_damaged {
                    // Need to include last damaged spring in match.
                    if last_damaged > start + i + condition - 1 {
                        return false;
                    }
                }

                // Check for match out of bounds.
                if start + *i + condition > map.len() {
                    return false;
                }

                // Match must span non-operational springs.
                for j in *i..*i + condition {
                    if map[start + j] == Spring::Operational {
                        return false;
                    }
                }

                true
            });

            if let Some((i, _)) = possibility {
                //println!("FOUND i:{i}");
                result += 1;

                // Check again starting one spring after this match.
                start += i + 1;
            } else {
                //println!("BREAK");
                break;
            }
        }

        result
    }

    fn possibilities_from_conditions(
        map: &[Spring],
        conditions: &[usize],
        cache: &mut HashMap<Row, usize>,
    ) -> usize {
        //let sp = format!("{:1$}", " ", (6 - conditions.len()) * 4);
        let rr = Row {
            map: map.to_vec(),
            conditions: conditions.to_vec(),
        };
        if let Some(result) = cache.get(&rr) {
            return *result;
        }

        //println!("________________ {rr}");
        //println!("{sp}________________ {rr}");
        if conditions.len() == 1 {
            let single = Row::possibilities_from_condition(map, conditions[0]);
            //println!("{sp}---------------------------------->>> SINGLE {single}");
            return single;
        }

        let mut result = 0;
        let mut start = 0;
        let condition = conditions[0];
        let first_damaged = map.iter().position(|x| *x == Spring::Damaged);
        //println!("{sp}{first_damaged:?}");

        loop {
            //println!("{sp}--start:{start}");
            let possibility = map.iter().skip(start).enumerate().find(|(i, _)| {
                if let Some(first_damaged) = first_damaged {
                    // Need to include first damaged spring in match.
                    if first_damaged < start + i {
                        //|| first_damaged == start + i + 1 {
                        return false;
                    }

                    //if first_damaged == start + i + condition {
                    //    return false;
                    //}

                    // Position after match cannot be damaged.
                    if start + i + condition < map.len()
                        && map[start + i + condition] == Spring::Damaged
                    {
                        return false;
                    }
                }

                // Check for match out of bounds.
                if start + *i + condition > map.len() {
                    return false;
                }

                // Match must span non-operational springs.
                for j in *i..*i + condition {
                    if map[start + j] == Spring::Operational {
                        return false;
                    }
                }

                true
            });

            if let Some((i, _)) = possibility {
                //println!("{sp}FOUND i:{i}");
                if start + i + condition + 1 > map.len() - 1 {
                    //println!(">>>>>>BREAK<<<<<<<<<<<<<<");
                    break;
                }
                result += Row::possibilities_from_conditions(
                    &map[start + i + condition + 1..],
                    &conditions[1..],
                    cache,
                );

                // Check again starting one spring after this match.
                start += i + 1;
            } else {
                //println!("{sp}BREAK");
                break;
            }
        }

        cache.insert(rr, result);
        result
    }

    /*
    //#[allow(unused)]
    fn find_possibilites(map: &[Spring], conditions: &[usize]) -> usize {
        //let sp = format!("{:1$}", " ", (2 - conditions.len()) * 4);
        let sp = "";
        let rr = Row {
            map: map.to_vec(),
            conditions: conditions.to_vec(),
        };
        //println!("{sp}___ {rr} ____________________");
        let is_last = conditions.len() == 1;
        let size = conditions[0];
        let tail_len = conditions.iter().sum::<usize>() + conditions.len() - 1;

        let mut start = 0;
        let mut result = 0;

        'out: loop {
            //println!("{sp}loop {start}");
            if start + tail_len > map.len() {
                //println!(" TOO BIG break");
                break;
            }

            let next_match = map.iter().skip(start).enumerate().find(|(i, _)| {
                //println!("--- {}", start + *i);

                //////////////// TODO
                if start + *i + tail_len > map.len() {
                    return false;
                }
                /////////////////////

                if start > 0 {
                    let prior_idx = start - 1;
                    if map[prior_idx] == Spring::Damaged {
                        return false;
                    }
                }

                for spring in map.iter().skip(start + *i).take(size) {
                    //println!("{spring:?}");
                    if *spring == Spring::Operational {
                        return false;
                    }
                }

                if is_last {
                    // Make sure there are no remaining damaged springs.
                    //for spring in map.iter().skip(start + *i + size) {
                    for spring in map.iter().skip(start + *i + size) {
                        if *spring == Spring::Damaged {
                            return false;
                        }
                    }

                    true
                } else {
                    let c = map[start + *i + size] != Spring::Damaged;
                    //println!(".. next damaged {c:?}");
                    c
                }
            });

            if let Some((next_match, _)) = next_match {
                //println!("{sp}!!!!!! next match {start} {next_match}");
                for i in start..next_match {
                    if map[i] == Spring::Damaged {
                        //println!("????????????????????????????????????????????????????????");
                        break 'out;
                    }
                }
                if is_last {
                    //println!("{sp}{rr} ++++++++++++++++111111111111111111111111111");
                    result += 1;
                } else {
                    result += Row::find_possibilites(
                        &map[start + next_match + size + 1..],
                        &conditions[1..],
                    );
                    // If this match contains a damaged spring, we can't progress - must use it.
                    for i in start + next_match..start + next_match + size {
                        if map[i] == Spring::Damaged {
                            break 'out;
                        }
                    }
                }
                //if map[start + next_match] == Spring::Damaged {
                //    break 'out;
                //} else {
                start += next_match + 1;
                //}
            } else {
                //println!("{sp}no next match");
                break;
            }
        }
        //println!("return result: {result}");

        result
    }
        */

    /*
    let contains_operational = map
        .iter()
        .skip(start)
        .take(size)
        .filter(|x| **x == Spring::Operational)
        .count()
        > 0;

    let is_match = if contains_operational {
        false
    } else if is_last {
        true
    } else {
        map[start + size] != Spring::Damaged
    };

    if is_match {
        println!("found match");
    }

    start += 1;
    */

    /*
    fn find_possibilites(map: &[Spring], conditions: &[usize]) -> usize {
        let space = format!("{:>1$}", " ", (3 - conditions.len()) * 4);
        //let space = "";
        let prmap = Row {
            map: map.to_vec(),
            conditions: conditions.to_vec(),
        };
        println!("{space}_________ {prmap}_______________ {conditions:?} _____");
        // Ensure room for conditions with space between.
        if conditions.iter().sum::<usize>() + conditions.len() - 1 > map.len() {
            return 0;
        }

        let mut idx = 0;
        let mut result = 0;
        let size = conditions[0];

        while let Some((i, _)) = map.iter().skip(idx).enumerate().find(|(i, _)| {
            println!("{space}{prmap} [-----{idx}-{size}------------{i}]");
            for spring in map.iter().skip(*i).take(size) {
                println!("{space} checkin.. {spring:?}");
                if ![Spring::Unknown, Spring::Damaged].contains(spring) {
                    return false;
                }
            }
            if conditions.len() == 1 {
                true
            } else {
                [Spring::Unknown, Spring::Operational].contains(&map[*i + size])
            }
        }) {
            println!("{space}00 i:{i} size:{size} result:{result}");
            result += if conditions.len() == 1 {
                println!("{space}** last condition return 1");
                1
            } else if idx + i + size < map.len() {
                Row::find_possibilites(&map[idx + i + size + 1..], &conditions[1..])
            } else {
                0
            };
            idx += i + 1;
        }
        println!("{space}_exit_ {result}");
        result
    }
    */

    /*
    fn possibilities(&self) -> usize {
        assert!(!self.conditions.is_empty());
        let size = self.conditions[0];
        assert!(size > 0);

        println!("____________\ncall possibilities on {self}");

        let result = 0;

        if self.conditions.len() == 1 && self.map.len() < size
            || self.conditions.len() > 1 && self.map.len() < size + 1
        {
            println!("doesn't fit - return 0");
            return 0;
        } else if let Some((i, _)) = self.map.iter().enumerate().find(|(i, _)| {
            println!("- loop from {} to {}", *i, *i + size);
            for j in *i..*i + size {
                if ![Spring::Unknown, Spring::Damaged].contains(&self.map[j]) {
                    return false;
                }
            }
            if self.conditions.len() == 1 {
                true
            } else {
                [Spring::Unknown, Spring::Operational].contains(&self.map[*i + size])
            }
        }) {
            println!("  found one!");
            if self.conditions.len() == 1 {
                println!("  last condition, return 1");
                return 1;
            } else {
                println!("  return recursion...");
                let result = Row {
                    map: Vec::from(&self.map[i + size..]),
                    conditions: Vec::from(&self.conditions[1..]),
                }
                .possibilities();
                println!("  got additional {result} from recursion");
                return result;
            }
        }

        0
    }
    */
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Row {
    map: Vec<Spring>,
    conditions: Vec<usize>,
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.map.iter().map(|x| x.to_char()).collect::<String>(),
            self.conditions
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn to_char(&self) -> char {
        match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        }
    }

    fn vec_from_str(s: &str) -> Vec<Self> {
        s.chars().map(Spring::from).collect()
    }
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }
}
