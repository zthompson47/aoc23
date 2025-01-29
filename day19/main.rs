use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut program: HashMap<&'static str, Routine> = HashMap::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (name, rules_src) = line.split_once('{').unwrap();
        let rules_src = rules_src.strip_suffix('}').unwrap().split(',');
        let mut comparisons: Vec<Comparison> = Vec::new();
        let mut result: Option<Outcome> = None;

        for rule_src in rules_src {
            if rule_src.contains(':') {
                let (operation, outcome) = rule_src.split_once(':').unwrap();
                let operation = operation.chars().collect::<Vec<_>>();
                comparisons.push(Comparison {
                    variable: Variable::from(operation[0]),
                    ordering: match operation[1] {
                        '<' => Less,
                        '>' => Greater,
                        _ => unreachable!(),
                    },
                    value: operation[2..]
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                    outcome: match outcome {
                        "A" => Outcome::Accept,
                        "R" => Outcome::Reject,
                        x => Outcome::GoSub(x),
                    },
                });
            } else {
                let end_result = match rule_src {
                    "A" => Outcome::Accept,
                    "R" => Outcome::Reject,
                    x => Outcome::GoSub(x),
                };
                result = Some(end_result);
            };
        }

        program.insert(
            name,
            Routine {
                comparisons,
                result: result.unwrap(),
            },
        );
    }

    let mut parts: Vec<Part> = Vec::new();
    for line in lines {
        let variables = line
            .trim_matches(['{', '}'])
            .split(',')
            .map(|x| {
                x.chars().collect::<Vec<_>>()[2..]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        parts.push(Part {
            x: variables[0],
            m: variables[1],
            a: variables[2],
            s: variables[3],
        });
    }

    let part1: u32 = parts
        .iter()
        .filter(|part| run_routine("in", &program, part))
        .map(|part| part.rating())
        .sum();

    println!("Part 1: {part1}");

    /*
    let zero = Part::new(0, 0, 0, 0);
    dbg!(zero.run(&program));
    for i in 1..=4000 {
        println!("{} {}", i, Part::new(i, 0, 0, 0).run(&program));
    }
    */

    /*
    use rayon::prelude::*;
    let count: u32 = (0..217).into_par_iter().fold(|| 0, |count, x| {
    //for x in 0..204 {
        let mut inner_count = 0;
        for m in 0..217 {
            for a in 0..242 {
                for s in 0..244 {
                    if Part::new(x, m, a, s).run(&program) {
                        inner_count += 1;
                    }
                }
            }
        }
        count + inner_count
    }).sum();
    dbg!(count);
    */

    let mut inflections: HashMap<Variable, Vec<u32>> = program
        .values()
        .map(|x| &x.comparisons)
        .fold(HashMap::new(), |mut acc, comparisons| {
            for c in comparisons {
                let inflection = match c.ordering {
                    Less => c.value,
                    Equal => unreachable!(),
                    Greater => c.value + 1,
                };
                acc.entry(c.variable)
                    .and_modify(|x| x.push(inflection))
                    .or_insert(vec![inflection]);
            }
            acc
        });

    for val in inflections.values_mut() {
        val.sort();
        val.insert(0, 1);
    }

    //dbg!(&inflections);

    let mut inflection_blocks: HashMap<Variable, HashMap<u32, u32>> = HashMap::new();
    for (k, v) in inflections {
        let mut blocks: HashMap<u32, u32> = HashMap::new();
        for i in 0..v.len() {
            let sample = v[i];
            let size = match i + 1 == v.len() {
                true => 4001,
                false => v[i + 1],
            } - sample;
            blocks.insert(sample, size);
        }
        inflection_blocks.insert(k, blocks);
    }

    //dbg!(&inflection_blocks);

    use rayon::prelude::*;
    let part2: u128 = inflection_blocks
        .get(&Variable::X)
        .unwrap()
        .into_par_iter()
        .fold(
            || 0,
            |mut part2, x| {
                //for x in inflection_blocks.get(&Variable::X).unwrap() {
                for m in inflection_blocks.get(&Variable::M).unwrap() {
                    for a in inflection_blocks.get(&Variable::A).unwrap() {
                        for s in inflection_blocks.get(&Variable::S).unwrap() {
                            if Part::new(*x.0, *m.0, *a.0, *s.0).run(&program) {
                                part2 += *x.1 as u128 * *m.1 as u128 * *a.1 as u128 * *s.1 as u128;
                            }
                        }
                    }
                }
                part2
            },
        )
        .sum();

    println!("Part 2: {part2}");
}

fn run_routine(name: &'static str, program: &HashMap<&'static str, Routine>, part: &Part) -> bool {
    let routine = program.get(name).unwrap();
    let process_result = |rule: &Outcome| -> bool {
        match rule {
            Outcome::GoSub(name) => run_routine(name, program, part),
            Outcome::Accept => true,
            Outcome::Reject => false,
        }
    };

    for comparison in routine.comparisons.iter() {
        if part.variable(comparison.variable).cmp(&comparison.value) == comparison.ordering {
            return process_result(&comparison.outcome);
        }
    }

    process_result(&routine.result)
}

#[derive(Clone, Copy, Debug)]
struct Comparison {
    variable: Variable,
    ordering: std::cmp::Ordering,
    value: u32,
    outcome: Outcome,
}

#[derive(Debug)]
struct Routine {
    comparisons: Vec<Comparison>,
    result: Outcome,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    GoSub(&'static str),
    Reject,
    Accept,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        Part { x, m, a, s }
    }

    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn variable(&self, v: Variable) -> u32 {
        match v {
            Variable::X => self.x,
            Variable::M => self.m,
            Variable::A => self.a,
            Variable::S => self.s,
        }
    }

    fn run(&self, program: &HashMap<&'static str, Routine>) -> bool {
        run_routine("in", program, self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Variable {
    X,
    M,
    A,
    S,
}

impl From<char> for Variable {
    fn from(value: char) -> Self {
        match value {
            'x' => Variable::X,
            'm' => Variable::M,
            'a' => Variable::A,
            's' => Variable::S,
            _ => unreachable!(),
        }
    }
}
