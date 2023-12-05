const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    // Part 1
    let result = include_str!("input.txt")
        .lines()
        .fold(0, |acc, line| acc + number_from_line(line));

    println!("Part 1: {result}");

    // Part 2
    let result = include_str!("input.txt").lines().fold(0, |acc, line| {
        let mut first = None;
        let mut last = None;

        for i in 0..line.len() {
            if let (Some(digit), _) = parse_number(&line[i..]) {
                first = Some(digit);
                break;
            }
        }

        for i in 0..line.len() {
            if let (Some(digit), _) = parse_number(&line[(line.len() - 1 - i)..]) {
                last = Some(digit);
                break;
            }
        }

        if let (Some(first), Some(last)) = (first, last) {
            acc + first * 10 + last
        } else {
            acc
        }
    });

    println!("Part 2: {result:?}");
}

fn number_from_line(line: &str) -> u32 {
    let mut digits = line.chars().filter(char::is_ascii_digit);

    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn parse_number(input: &str) -> (Option<u32>, &str) {
    for (i, num) in NUMBERS.iter().enumerate() {
        if let Some(stripped) = input.strip_prefix(num) {
            return (Some(i as u32 + 1), stripped);
        }
    }

    if let Some(digit) = input.chars().next() {
        if digit.is_ascii_digit() {
            (Some(digit.to_digit(10).unwrap()), &input[1..])
        } else {
            (None, &input[1..])
        }
    } else {
        (None, &input[1..])
    }
}
