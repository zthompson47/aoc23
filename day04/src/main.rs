use std::collections::BTreeSet;

fn main() {
    let result: u32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let card = line
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|x| {
                    x.split_ascii_whitespace()
                        .map(|y| y.parse::<u32>().unwrap())
                        .collect::<BTreeSet<u32>>()
                })
                .collect::<Vec<BTreeSet<u32>>>();
            let winners = &card[0];
            let picks = &card[1];

            let result = picks.iter().fold(0, |game_score, x| {
                if winners.contains(x) {
                    if game_score == 0 {
                        1
                    } else {
                        game_score * 2
                    }
                } else {
                    game_score
                }
            });

            result
        })
        .sum();

    println!("{result:?}");
}
