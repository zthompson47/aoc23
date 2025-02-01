use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part2(input: &'static str) -> u128 {
    let mut network = Network::new(input);
    let mut count = 0;

    //let first_nodes = ["ct", "hr", "ft", "qm"];
    //let initial_state = network.node_state(&first_nodes);

    let (mut kd, mut zz, mut mh, mut cm): (Option<u128>, Option<u128>, Option<u128>, Option<u128>) =
        (None, None, None, None);
    'out: loop {
        count += 1;

        //let to_cm = ["bd", "lg", "qd", "rp", "pc", "xl", "qn", "bk"];
        //let to_mh = ["mj", "fr", "tp", "fl", "tk", "jg", "xg", "lz", "ct"];
        //let to_zz = ["lj", "lh", "mp", "pz", "zq", "hr", "cx", "mf", "fs"];
        //let to_kd = ["ks", "gk", "vq", "xk", "kb", "qm", "hv", "nv", "qj"];

        //let zz_lineup = [
        //    "mp", "mf", "pz", "lh", "fs", "jk", "cx", "lj", "bh", "th", "zq", "hr",
        //];

        //println!("{} {}", network.node_state(&zz_lineup), network.node_state(&["zz"]));

        let mut messages = vec![Message {
            from: "button",
            to: "broadcaster",
            signal: Signal::Low,
        }];

        while !messages.is_empty() {
            let mut new_messages = vec![];
            for message in messages.iter() {
                //if message.to == "rx" && message.signal == Signal::Low {
                //    break 'out count;
                //}
                // Find length of cycle for each counter feeding final four conjunctions.
                if message.signal == Signal::Low {
                    if message.to == "hr" && message.from == "zz" {
                        zz = Some(count);
                    }
                    if message.to == "ct" && message.from == "mh" {
                        mh = Some(count);
                    }
                    if message.to == "ft" && message.from == "cm" {
                        cm = Some(count);
                    }
                    if message.to == "qm" && message.from == "kd" {
                        kd = Some(count);
                    }
                    if let (Some(zz), Some(mh), Some(cm), Some(kd)) = (zz, mh, cm, kd) {
                        use num::integer::lcm;
                        break 'out lcm(zz, lcm(mh, lcm(cm, kd)));
                    }
                }

                new_messages.extend(network.send_message(message));
            }
            messages = new_messages;
        }
    }
}

fn part1(input: &'static str) -> u32 {
    let mut network = Network::new(input);
    for _ in 0..1000 {
        let mut messages = vec![Message {
            from: "button",
            to: "broadcaster",
            signal: Signal::Low,
        }];
        while !messages.is_empty() {
            let mut new_messages = vec![];
            for message in messages.iter() {
                new_messages.extend(network.send_message(message));
            }
            messages = new_messages;
        }
    }
    network.counter.high * network.counter.low
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<&'static str, Node>,
    counter: Counter,
}

impl Network {
    #[allow(unused)]
    fn flip_flop_state(&self) -> String {
        let mut result = String::new();
        for node in self.nodes.values() {
            if let Module::FlipFlop(on) = &node.module {
                result.push_str(if *on { "1" } else { "0" });
            }
        }
        result
    }

    #[allow(unused)]
    fn conjunction_state(&self) -> String {
        let mut result = String::new();
        for node in self.nodes.values() {
            if let Module::Conjunction(signals) = &node.module {
                for signal in signals {
                    result.push_str(match signal.1 {
                        Signal::Low => "0",
                        Signal::High => "1",
                    });
                }
            }
        }
        result
    }

    #[allow(unused)]
    fn state(&self) -> String {
        let mut result = String::new();
        for node in self.nodes.values() {
            if let Module::Conjunction(signals) = &node.module {
                for signal in signals {
                    result.push_str(match signal.1 {
                        Signal::Low => "0",
                        Signal::High => "1",
                    });
                }
            } else if let Module::FlipFlop(on) = &node.module {
                result.push_str(if *on { "1" } else { "0" });
            }
        }
        result
    }

    #[allow(unused)]
    fn node_state(&self, nodes: &[&'static str]) -> String {
        let mut result = String::new();
        for name in nodes {
            if let Some(node) = self.nodes.get(name) {
                if let Module::Conjunction(signals) = &node.module {
                    for signal in signals {
                        result.push_str(match signal.1 {
                            Signal::Low => "0",
                            Signal::High => "1",
                        });
                    }
                } else if let Module::FlipFlop(on) = &node.module {
                    result.push_str(if *on { "1" } else { "0" });
                }
            }
        }
        result
    }

    fn send_message(&mut self, message: &Message) -> Vec<Message> {
        self.counter.count(message.signal);
        if let Some(node) = self.nodes.get_mut(message.to) {
            node.send_signal(message.signal, message.from)
        } else {
            vec![]
        }
    }

    fn new(input: &'static str) -> Self {
        let mut module_inputs: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        let mut nodes = input.lines().fold(HashMap::new(), |mut nodes, line| {
            let (module, outputs) = line.split_once(" -> ").unwrap();
            let outputs: Vec<&'static str> = outputs.split(", ").collect();
            let (module_type, mut name) = module.split_at(1);
            if module_type == "b" {
                name = "broadcaster";
            }

            for output in &outputs {
                module_inputs
                    .entry(output)
                    .and_modify(|inputs| inputs.push(name))
                    .or_insert(vec![name]);
            }

            nodes.insert(
                name,
                Node {
                    name,
                    module: match module_type {
                        "b" => Module::Broadcaster,
                        "%" => Module::FlipFlop(false),
                        "&" => Module::Conjunction(HashMap::new()),
                        _ => unreachable!(),
                    },
                    outputs,
                },
            );

            nodes
        });

        // Store inputs for all `Module::Conjunction`.
        for (name, inputs) in module_inputs {
            if let Some(Node {
                module: Module::Conjunction(input_signals),
                ..
            }) = nodes.get_mut(name)
            {
                for input in inputs {
                    input_signals.insert(input, Signal::Low);
                }
            }
        }

        Network {
            nodes,
            counter: Counter::default(),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: &'static str,
    module: Module,
    outputs: Vec<&'static str>,
}

#[derive(Debug)]
struct Message {
    from: &'static str,
    to: &'static str,
    signal: Signal,
}

impl Node {
    fn send_signal(&mut self, signal: Signal, from: &'static str) -> Vec<Message> {
        let mut result = vec![];
        if let Some(output_signal) = self.module.pulse(signal, from) {
            for output in self.outputs.iter() {
                result.push(Message {
                    from: self.name,
                    to: output,
                    signal: output_signal,
                });
            }
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Default)]
struct Counter {
    high: u32,
    low: u32,
}

impl Counter {
    fn count(&mut self, signal: Signal) {
        match signal {
            Signal::Low => self.low += 1,
            Signal::High => self.high += 1,
        }
    }
}

#[derive(Debug)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'static str, Signal>),
}

impl Module {
    fn pulse(&mut self, signal: Signal, source: &'static str) -> Option<Signal> {
        match self {
            Module::Broadcaster => Some(signal),
            Module::FlipFlop(on) => {
                if signal == Signal::Low {
                    match on {
                        true => {
                            *on = false;
                            Some(Signal::Low)
                        }
                        false => {
                            *on = true;
                            Some(Signal::High)
                        }
                    }
                } else {
                    None
                }
            }
            Module::Conjunction(inputs) => {
                *inputs.get_mut(source).unwrap() = signal;
                if inputs.values().any(|signal| *signal == Signal::Low) {
                    Some(Signal::High)
                } else {
                    Some(Signal::Low)
                }
            }
        }
    }
}
