#![deny(elided_lifetimes_in_paths)]

fn main() {
    println!("pat 1 : {:?}", day12::run(day12::Part::One).iter().sum::<usize>());
    /*
    let v1 = day12::run(day12::Part::One);
    let v2 = part2();
    v1.iter()
        .zip(v2.iter())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if *a as u32 != *b {
                println!("MISMATCH: {i} {a} {b}");
            }
        });
    println!("Part 1: {}", v1.iter().sum::<usize>());
    println!("Part 2: {}", v2.iter().sum::<u32>());
    */
}

fn part2() -> Vec<u32> {
    input(day12::Part::One, None)
        .iter()
        .fold(Vec::new(), |mut result, row| {
            println!("{} {} {:?}", row.string, row.string.len(), row.groups);

            let st = row.string.chars().collect::<Vec<char>>();

            let possible_slots: Vec<Vec<usize>> = row
                .groups
                .iter()
                .enumerate()
                .map(|(group_idx, group)| {
                    // Get range based on lengths of surrounding groups.
                    let pre = &row.groups[0..group_idx];
                    let pre_sum = pre.iter().sum::<u32>() as usize;
                    let pre_len = pre.len();
                    let start = pre_sum + pre_len;

                    let post_0 = if group_idx < row.groups.len() {
                        group_idx + 1
                    } else {
                        group_idx
                    };
                    let post = &row.groups[post_0..];
                    let post_sum = post.iter().sum::<u32>() as usize;
                    let post_len = post.len();
                    let end = st.len() - (post_sum + post_len);

                    //println!();
                    //println!("pre_sum {pre_sum}, pre_len {pre_len}");
                    //println!("post_sum {post_sum}, post_len {post_len}");
                    //println!("group_idx {} with range: {:?}", group_idx, (start..end));

                    (start..end)
                        .filter(|i| {
                            // Group has no preceeding pipe.
                            if *i > 0 && st[*i - 1] == '#' {
                                return false;
                            }
                            // Group has no following pipe.
                            if (*i + *group as usize) < st.len() && st[*i + *group as usize] == '#'
                            {
                                return false;
                            }
                            // Group contains no ground.
                            *i < st.len()
                                && (*i + *group as usize) <= st.len()
                                && !st[*i..*i + *group as usize].contains(&'.')
                        })
                        .collect()
                })
                .collect();

            println!("{:?}", possible_slots);

            /*println!(
                "     \"{}\" {} {:?}",
                row.string,
                row.string.len(),
                row.groups
            );*/

            let mut new_results = 0;
            let regex = groups_regex(row.groups.as_slice());
            for (count, solution) in
                Solutions::new(row.string.clone(), &row.groups, possible_slots).enumerate()
            {
                {
                    //if count > 5 {
                    //    break;
                    //}
                }
                //println!("{}", solution);
                if check_solution(&row.string, &solution) {
                    if is_solution(&solution, &regex) {
                        //println!("YES: {:?}", solution);
                        new_results += 1;
                    } else {
                        println!("_n_: {:?}", solution);
                    }
                } else {
                    println!("NO_: {:?}", solution);
                    println!("     {:?}", row.string);
                }
            }

            result.push(new_results);
            result
        })
}

fn check_solution(left: &str, right: &str) -> bool {
    for (l, r) in left.chars().zip(right.chars()) {
        match l {
            '.' => {
                if r != '.' {
                    return false;
                }
            }
            '#' => {
                if r != '#' {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn groups_regex(groups: &[u32]) -> regex::Regex {
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
    //println!("regex: {regex:?}");
    if let Some(m) = regex.find(solution) {
        //println!("match: {m:?}");
        !m.is_empty()
    } else {
        false
    }
}

struct Solutions {
    string: String,
    possible_slots: Vec<Vec<usize>>,
    groups: Vec<u32>,
    index: Option<Vec<usize>>,
    count: u32,
}

impl Solutions {
    fn new(string: String, groups: &[u32], possible_slots: Vec<Vec<usize>>) -> Self {
        assert_eq!(possible_slots.len(), groups.len());

        // Determine first possible index where groups don't overwrite each other.
        let mut index = vec![0];
        for i in 1..possible_slots.len() {
            let start_idx = possible_slots[i - 1][index[i - 1]];
            //println!("start_idx {start_idx}, {index:?}");
            for (slot_idx, slot) in possible_slots[i].iter().enumerate() {
                if start_idx + (groups[i - 1] as usize) < *slot {
                    index.push(slot_idx);
                    break;
                }
            }
        }
        let index = Some(index);
        //println!("{index:?}");

        Solutions {
            possible_slots,
            groups: groups.to_vec(),
            index,
            string,
            count: 0,
        }
    }
}

impl Iterator for Solutions {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = &mut self.index {
            // Write resulting solution string for this permutation.
            let mut result = vec!['.'; self.string.len()];
            for (count, i) in index.iter().enumerate() {
                for g in 0..self.groups[count] {
                    result[self.possible_slots[count][*i] + g as usize] = '#';
                }
            }

            // DEBUG
            //println!("trying: {:?}", result.iter().collect::<String>());
            self.count += 1;
            //if self.count % 10000000 == 0 {
            //println!("{}", self.count);
            //print!(".");
            //}
            // DEBUG

            println!("{index:?} {}", result.iter().collect::<String>());

            // Update the index.
            for i in 0..index.len() {
                index[i] += 1;
                if index[i] == self.possible_slots[i].len() {
                    // Index slot exhausted, reset and continue to next index slot.
                    index[i] = 0;
                    if i == self.possible_slots.len() - 1 {
                        // Last index slot exhausted, close iterator.
                        self.index = None;
                        break;
                    }
                } else if i < index.len() - 1 {
                    // Quit incrementing index slot when group overlaps next slot.
                    let start_idx = self.possible_slots[i][index[i]];
                    let group_len = self.groups[i] as usize;
                    let next_idx = self.possible_slots[i + 1][index[i + 1]];

                    print!("start_idx {start_idx}, group_len {group_len}, next_idx {next_idx} ");

                    if start_idx + group_len >= next_idx {
                        println!("0000000000000000000000000000000");
                        // Overlap, reset slot.
                        index[i] = 0;
                    } else if self.string[start_idx + group_len..next_idx].contains('#') {
                        // Find first slot index that covers the required pipe.
                        println!("------------>>>>>>>> GOT_ONE!!!!!!!!!!!!!!!!!");
                        break;
                    } else {
                        println!("1111111111111111111111111111111");
                        // Normal flow, stop after each slot increments.
                        break;
                    }
                } else {
                    // Normal flow, stop after last slot increment.
                    break;
                }
            }

            //println!("\n{index:?}");

            return Some(result.iter().collect());
        }
        println!(" =====>>>  {}", self.count);
        None
    }
}

#[derive(Debug)]
struct Row {
    string: String,
    groups: Vec<u32>,
}

fn input(part: day12::Part, take: Option<usize>) -> Vec<Row> {
    if let Some(count) = take {
        Box::new(include_str!("input.txt").lines().take(count)) as Box<dyn Iterator<Item = &str>>
    } else {
        Box::new(include_str!("input.txt").lines()) as Box<dyn Iterator<Item = &str>>
    }
    .fold(Vec::new(), |mut result, row| {
        let mut split = row.split_ascii_whitespace();
        let mut string = split.next().unwrap().to_string();
        let mut groups = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        if let day12::Part::Two = part {
            let a = string.as_str();
            string = [a, a, a, a, a].join("?");
            groups = [
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
                groups.as_slice(),
            ]
            .concat();
        }
        result.push(Row { string, groups });
        result
    })
}
