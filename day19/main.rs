fn main() {
    let mut lines = include_str!("test.txt").lines();

    let mut program: Vec<Sub> = Vec::new();
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
        println!("{rules:?}");
        program.push(Sub { name, rules });
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
    println!("{parts:?}");
}

#[derive(Debug)]
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

#[derive(Debug)]
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

struct Sub {
    name: &'static str,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
