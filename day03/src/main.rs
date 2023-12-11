use std::ops::Range;

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .fold(Grid::default(), |mut acc, line| {
            let line = line.chars().collect::<Vec<_>>();
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
                    i += 1;
                }
            }

            acc.part_numbers.push(part_numbers);
            acc.symbols.push(symbols);

            acc
        });

    println!("{}", grid.sum_part_numbers());
}

#[derive(Default, Debug)]
struct Grid {
    part_numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<usize>>,
}

impl Grid {
    fn sum_part_numbers(&self) -> u32 {
        self.part_numbers
            .iter()
            .enumerate()
            .fold(0, |mut acc, (i, line)| {
                let start = if i == 0 { 0 } else { i - 1 };
                let end = if i == self.part_numbers.len() - 1 {
                    self.part_numbers.len() - 1
                } else {
                    i + 2
                };

                for number in line {
                    let horizontal_window = window_range(number.span.start, number.span.len());

                    for i in start..end {
                        for symbol in &self.symbols[i] {
                            if horizontal_window.contains(symbol) {
                                acc += number.value;
                            }
                        }
                    }
                }

                acc
            })
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
