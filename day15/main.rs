fn main() {
    println!(
        "Part 1: {}",
        include_str!("input.txt")
            .split(',')
            .map(|seq| {
                let result = seq.trim().as_bytes().iter().fold(0, |mut acc, c| {
                    let c = *c as u64;
                    acc += c;
                    acc *= 17;
                    acc %= 256;
                    acc
                });
                result
            })
            .sum::<u64>()
    );
}
