use std::{collections::HashMap, ops::Range};

fn main() {
    let mut grid = include_str!("input.txt").lines().enumerate().fold(
        Grid::default(),
        |mut acc, (line_number, line)| {
            let line: Vec<char> = line.chars().collect();

            let mut part_numbers = Vec::new();
            let mut symbols = Vec::new();

            let mut i = 0;
            while i < line.len() {
                if line[i] == '.' {
                    i += 1;
                    continue;
                } else if line[i].is_ascii_digit() {
                    let digits = line[i..]
                        .iter()
                        .take_while(|x| x.is_ascii_digit())
                        .collect::<String>();

                    part_numbers.push(Number::new(
                        i..i + digits.len(),
                        digits.parse::<u32>().unwrap(),
                    ));

                    i += digits.len();
                } else {
                    symbols.push(i);
                    if line[i] == '*' {
                        acc.gears.insert((line_number, i), TwoOrMore::None);
                    }
                    i += 1;
                }
            }

            acc.part_numbers.push(part_numbers);
            acc.symbols.push(symbols);

            acc
        },
    );

    println!("{}", grid.sum_part_numbers());
    println!("{}", grid.sum_gears());
}

#[derive(Default, Debug)]
struct Grid {
    part_numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<usize>>,
    gears: HashMap<(usize, usize), TwoOrMore>,
}

impl Grid {
    fn sum_part_numbers(&mut self) -> u32 {
        self.part_numbers
            .iter()
            .enumerate()
            .fold(0, |mut acc, (i, line)| {
                let start = if i == 0 { 0 } else { i - 1 };
                let end = if i == self.part_numbers.len() - 1 {
                    self.part_numbers.len()
                } else {
                    i + 2
                };

                for number in line {
                    let horizontal_window = window_range(number.span.start, number.span.len());

                    for j in start..end {
                        for symbol in self.symbols[j].iter() {
                            if horizontal_window.contains(symbol) {
                                if self.gears.contains_key(&(j, *symbol)) {
                                    let new_gear =
                                        self.gears.get(&(j, *symbol)).unwrap().push(number.value);
                                    self.gears.insert((j, *symbol), new_gear);
                                }
                                acc += number.value;
                            }
                        }
                    }
                }

                acc
            })
    }

    fn sum_gears(&self) -> u32 {
        self.gears
            .values()
            .filter_map(|x| {
                if let TwoOrMore::More(val) = x {
                    Some(val)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn window_range(index: usize, length: usize) -> Range<usize> {
    let index = index as i32;
    let length = length as i32;

    let start = index - 1;
    let end = index + length + 1;

    let start = if start < 0 { 0 } else { start };

    start as usize..end as usize
}

#[derive(Debug)]
struct Number {
    value: u32,
    span: Range<usize>,
}

impl Number {
    fn new(span: Range<usize>, value: u32) -> Self {
        Number { value, span }
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum TwoOrMore {
    #[default]
    None,
    One(u32),
    More(u32),
}

impl TwoOrMore {
    fn push(self, val: u32) -> Self {
        match self {
            TwoOrMore::None => TwoOrMore::One(val),
            TwoOrMore::One(x) => TwoOrMore::More(x * val),
            TwoOrMore::More(x) => TwoOrMore::More(x * val),
        }
    }
}
