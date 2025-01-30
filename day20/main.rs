#![allow(unused)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    // Part 1
    let mut network = build_network(input);
    let mut counter = Counter::default();
    for _ in 0..1000 {
        let mut messages = network
            .get_mut("broadcaster")
            .unwrap()
            .send_signal(Signal::Low, "broadcaster");
        counter.low += 1;
        while !messages.is_empty() {
            let mut new_messages = vec![];
            for message in messages.iter() {
                counter.count(message.signal);
                if let Some(node) = network.get_mut(message.to) {
                    new_messages.extend(node.send_signal(message.signal, message.from));
                }
            }
            messages = new_messages;
        }
    }

    println!("Part 1: {}", counter.high * counter.low);

    /*
    dbg!(&network);

    // Part 2
    let mut network = build_network(input);
    let mut count = 0;
    let count = 'out: loop {
        let mut messages = network
            .get_mut("broadcaster")
            .unwrap()
            .send_signal(Signal::Low, "broadcaster");
        count += 1;
        while !messages.is_empty() {
            let mut new_messages = vec![];
            for message in messages.iter() {
                if message.to == "cl" && message.signal == Signal::High {
                    println!("{count} {} {:?}", message.from, message.signal);
                }

                if message.to == "rx" && message.signal == Signal::Low {
                    break 'out count;
                }
                if let Some(node) = network.get_mut(message.to) {
                    new_messages.extend(node.send_signal(message.signal, message.from));
                }
            }
            messages = new_messages;
        }
        /*
        //if count % 10000000 == 0 {
            //println!("{count}");
            for node in network.values()
            //.filter(|x| matches!(x.module, Module::Conjunction { .. }))
            {
                if let Module::Conjunction(signals) = &node.module {
                    for signal in signals {
                        print!(
                            "{}",
                            match signal.1 {
                                Signal::Low => 0,
                                Signal::High => 1,
                            }
                        );
                    }
                } else if let Module::FlipFlop(on) = &node.module {
                    print!("{}", if *on { "1" } else { "0" });
                }
            }
            println!();
        //}
        */
    };

    println!("Part 2: {count}");
    */
}

struct Network {
    nodes: HashMap<&'static str, Node>,
    counter: Counter,
    cycles: u32,
}

fn build_network(input: &'static str) -> HashMap<&'static str, Node> {
    let mut module_inputs: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let mut network = input.lines().fold(HashMap::new(), |mut network, line| {
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

        network.insert(
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

        network
    });

    // Store inputs for all `Module::Conjunction`.
    for (name, inputs) in module_inputs {
        if let Some(Node {
            module: Module::Conjunction(input_signals),
            ..
        }) = network.get_mut(name)
        {
            for input in inputs {
                input_signals.insert(input, Signal::Low);
            }
        }
    }

    network
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

/*
trait Pulse {
    fn pulse(&mut self, signal: Signal, source: &'static str) -> Option<Signal>;
}

struct Conjunction {
    inputs: HashMap<&'static str, Signal>,
}

impl Pulse for Conjunction {
    fn pulse(&mut self, signal: Signal, source: &'static str) -> Option<Signal> {
        *self.inputs.get_mut(source).unwrap() = signal;
        if self.inputs.values().any(|f| *f == Signal::Low) {
            Some(Signal::High)
        } else {
            Some(Signal::Low)
        }
    }
}

struct FlipFlop {
    on: bool,
}

impl Pulse for FlipFlop {
    fn pulse(&mut self, signal: Signal, _: &'static str) -> Option<Signal> {
        if signal == Signal::Low {
            match self.on {
                true => {
                    self.on = false;
                    Some(Signal::Low)
                }
                false => {
                    self.on = true;
                    Some(Signal::High)
                }
            }
        } else {
            None
        }
    }
}

struct Broadcaster;

impl Pulse for Broadcaster {
    fn pulse(&mut self, signal: Signal, _: &'static str) -> Option<Signal> {
        Some(signal)
    }
}
*/
