use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", run_game(Game::Jacks));
    println!("Part 2: {}", run_game(Game::Jokers));
}

fn run_game(game: Game) -> usize {
    let mut hands = hands(game);

    for hand_type in hands.values_mut() {
        hand_type.sort_by_key(|x| x.as_number());
    }

    let five_of_a_kind = hands[&HandType::FiveOfAKind].as_slice();
    let four_of_a_kind = hands[&HandType::FourOfAKind].as_slice();
    let full_house = hands[&HandType::FullHouse].as_slice();
    let three_of_a_kind = hands[&HandType::ThreeOfAKind].as_slice();
    let two_pair = hands[&HandType::TwoPair].as_slice();
    let one_pair = hands[&HandType::OnePair].as_slice();
    let high_card = hands[&HandType::HighCard].as_slice();
    let all = [
        high_card,
        one_pair,
        two_pair,
        three_of_a_kind,
        full_house,
        four_of_a_kind,
        five_of_a_kind,
    ]
    .concat();

    all.iter().enumerate().map(|(i, x)| x.bid * (i + 1)).sum()
}

fn hands(game: Game) -> HashMap<HandType, Vec<Hand<'static>>> {
    let lines = include_str!("input.txt").lines();

    lines.fold(HashMap::new(), |mut acc, line| {
        let mut line = line.split_ascii_whitespace();
        let hand = Hand {
            cards: line.next().unwrap(),
            bid: line.next().unwrap().parse::<usize>().unwrap(),
            game,
        };

        acc.entry(hand.hand_type())
            .and_modify(|x: &mut Vec<Hand>| x.push(hand.clone()))
            .or_insert(vec![hand]);

        acc
    })
}

#[derive(Clone, Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
    game: Game,
}

impl Hand<'_> {
    fn as_number(&self) -> usize {
        self.cards
            .chars()
            .enumerate()
            .fold(0, |mut acc, (i, card)| {
                acc += 13usize.pow(4 - i as u32) * self.card_order(&card);

                acc
            })
    }

    fn hand_type(&self) -> HandType {
        let mut cards = self.cards.to_string();

        let joker_count = if let Game::Jokers = self.game {
            let old_len = cards.len();
            cards = cards.replace('J', "");
            old_len - cards.len()
        } else {
            0
        };

        let cards = cards.chars().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let mut ranks = cards.values().copied().collect::<Vec<usize>>();
        ranks.sort_by(|a, b| b.cmp(a));

        let mut hand_type = if ranks.is_empty() {
            HandType::None
        } else if ranks[0] == 5 {
            HandType::FiveOfAKind
        } else if ranks[0] == 4 {
            HandType::FourOfAKind
        } else if ranks[0] == 3 && ranks.len() > 1 && ranks[1] == 2 {
            HandType::FullHouse
        } else if ranks[0] == 3 {
            HandType::ThreeOfAKind
        } else if ranks[0] == 2 && ranks.len() > 1 && ranks[1] == 2 {
            HandType::TwoPair
        } else if ranks[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        for _ in 0..joker_count {
            hand_type = match hand_type {
                HandType::None => HandType::HighCard,
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FullHouse => panic!(),
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FiveOfAKind => panic!(),
            }
        }

        hand_type
    }

    fn card_order(&self, card: &char) -> usize {
        let inc = match self.game {
            Game::Jacks => 0,
            Game::Jokers => 1,
        };

        match card {
            '2' => inc,
            '3' => 1 + inc,
            '4' => 2 + inc,
            '5' => 3 + inc,
            '6' => 4 + inc,
            '7' => 5 + inc,
            '8' => 6 + inc,
            '9' => 7 + inc,
            'T' => 8 + inc,
            'J' => match self.game {
                Game::Jacks => 9,
                Game::Jokers => 0,
            },
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Game {
    Jacks,
    Jokers,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_correct_answer() {
        assert_eq!(252295678, run_game(Game::Jacks));
    }

    #[test]
    fn part2_correct_answer() {
        assert_eq!(250577259, run_game(Game::Jokers));
    }
}
