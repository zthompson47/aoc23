use std::ops::Range;

fn main() {
    let grid = include_str!("input.txt").lines().enumerate().fold(
        Grid::default(),
        |mut acc, (line_num, line)| {
            let line = line.chars().collect::<Vec<_>>();
            let len = line.len();
            let mut i = 0;
            let mut part_numbers = Vec::new();
            let mut symbols = Vec::new();

            while i < len {
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
        },
    );

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
            .fold(0, |acc, (i, line)| acc)
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    span: Range<usize>,
    is_part_number: bool,
}

impl Number {
    fn new(span: Range<usize>, value: u32) -> Self {
        Number {
            value,
            span,
            is_part_number: false,
        }
    }
}
