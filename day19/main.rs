use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut program: HashMap<&'static str, Vec<Rule>> = HashMap::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (name, rules_src) = line.split_once('{').unwrap();
        let mut rules: Vec<Rule> = Vec::new();
        let rules_src = rules_src.strip_suffix('}').unwrap().split(',');
        for rule_src in rules_src {
            rules.push(if rule_src.contains(':') {
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
                    "A" => Rule::Accept,
                    "R" => Rule::Reject,
                    x => Rule::GoSub(x),
                };
                Rule::Compare {
                    variable: Variable::from(variable),
                    operator: match operator {
                        '<' => std::cmp::Ordering::Less,
                        '>' => std::cmp::Ordering::Greater,
                        _ => unreachable!(),
                    },
                    value,
                    result: Box::new(sub),
                }
            } else {
                match rule_src {
                    "A" => Rule::Accept,
                    "R" => Rule::Reject,
                    x => Rule::GoSub(x),
                }
            });
        }
        //dbg!(&rules);
        program.insert(name, rules);
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
    //dbg!(&parts);

    let part1: u32 = parts
        .iter()
        .filter(|part| run_routine("in", &program, part))
        .map(|part| part.rating())
        .sum();

    println!("Part 1: {part1}");
}

fn run_routine(
    name: &'static str,
    program: &HashMap<&'static str, Vec<Rule>>,
    part: &Part,
) -> bool {
    let routine = program.get(name).unwrap();
    let process_result = |rule: &Rule| -> bool {
        match rule {
            Rule::Compare { .. } => unreachable!(),
            Rule::GoSub(name) => run_routine(name, program, part),
            Rule::Accept => true,
            Rule::Reject => false,
        }
    };

    for rule in routine.iter().take(routine.len() - 1) {
        if let Rule::Compare {
            variable,
            operator,
            value,
            result,
        } = &rule
        {
            if part.variable(*variable).cmp(value) == *operator {
                return process_result(result);
            }
        } else {
            unreachable!()
        }
    }

    match routine.last() {
        Some(rule) => process_result(rule),
        None => unreachable!(),
    }
}

/*
fn __run_routine(
    name: &'static str,
    program: &HashMap<&'static str, Vec<Rule>>,
    part: &Part,
) -> bool {
    let mut routine = program.get(name).unwrap().iter();
    while let Some(Rule::Compare {
        variable,
        operator,
        value,
        result,
    }) = routine.next()
    {
        if part.variable(*variable).cmp(value) == *operator {
            return match **result {
                Rule::Compare { .. } => unreachable!(),
                Rule::GoSub(name) => return __run_routine(name, program, part),
                Rule::Accept => true,
                Rule::Reject => false,
            };
        }
    }
    match routine.next() {
        Some(rule) => match rule {
            Rule::Compare { .. } => unreachable!(),
            Rule::GoSub(name) => __run_routine(name, program, part),
            Rule::Accept => true,
            Rule::Reject => false,
        },
        None => unreachable!(),
    }
}

fn _run_routine(
    name: &'static str,
    program: &HashMap<&'static str, Vec<Rule>>,
    part: &Part,
) -> Rule {
    for rule in program.get(name).unwrap() {
        match rule {
            Rule::Compare {
                variable,
                operator,
                value,
                result,
            } => {
                let variable = part.variable(*variable);
                if variable.cmp(value) == *operator {
                    return match **result {
                        Rule::Compare { .. } => unreachable!(),
                        Rule::GoSub(name) => return _run_routine(name, program, part),
                        ref rule @ (Rule::Reject | Rule::Accept) => return rule.clone(),
                    };
                } else {
                    continue;
                }
            }
            Rule::GoSub(name) => return _run_routine(name, program, part),
            rule @ (Rule::Reject | Rule::Accept) => return rule.clone(),
        }
    }
    unreachable!()
}
*/

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

#[derive(Clone, Debug)]
enum Rule {
    Compare {
        variable: Variable,
        operator: std::cmp::Ordering,
        value: u32,
        result: Box<Rule>,
    },
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
