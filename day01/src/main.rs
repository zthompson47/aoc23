fn main() {
    let result = include_str!("input.txt").lines().fold(0, |acc, line| {
        let mut digits = line.chars().filter(char::is_ascii_digit);

        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        let number = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();

        acc + number
    });

    println!("{result}");
}
