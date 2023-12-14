use std::collections::BTreeSet;

fn main() {
    let mut scores = Vec::new();
    let result_pt1: u32 = include_str!("input.txt")
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

            let mut win_count = 0;
            let score = picks.iter().fold(0, |game_score, x| {
                if winners.contains(x) {
                    win_count += 1;
                    if game_score == 0 {
                        1
                    } else {
                        game_score * 2
                    }
                } else {
                    game_score
                }
            });

            scores.push(win_count);

            score
        })
        .sum();

    println!("{result_pt1:?}");

    let mut aggregate_scores = vec![1; scores.len()];

    for i in 0..scores.len() {
        for j in 0..scores[i] {
            aggregate_scores[i + j + 1] += aggregate_scores[i];
        }
    }
    println!("{}", aggregate_scores.iter().sum::<u32>());
}
