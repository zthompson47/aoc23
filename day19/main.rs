use std::collections::HashMap;

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
                let (operation, sub) = rule_src.split_once(':').unwrap();
                let operation = operation.chars().collect::<Vec<_>>();
                let variable = operation[0];
                let operator = operation[1];
                let value: u32 = operation[2..]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                let sub = match sub {
                    "A" => Outcome::Accept,
                    "R" => Outcome::Reject,
                    x => Outcome::GoSub(x),
                };
                comparisons.push(Comparison {
                    variable: Variable::from(variable),
                    ordering: match operator {
                        '<' => std::cmp::Ordering::Less,
                        '>' => std::cmp::Ordering::Greater,
                        _ => unreachable!(),
                    },
                    value,
                    outcome: sub.clone(), // Remove clone later
                });
            } else {
                let end_result = match rule_src {
                    "A" => Outcome::Accept,
                    "R" => Outcome::Reject,
                    x => Outcome::GoSub(x),
                };
                result = Some(end_result.clone());
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

struct Comparison {
    variable: Variable,
    ordering: std::cmp::Ordering,
    value: u32,
    outcome: Outcome,
}

struct Routine {
    comparisons: Vec<Comparison>,
    result: Outcome,
}

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Copy, Debug)]
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
