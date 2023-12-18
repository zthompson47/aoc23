use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let lines = include_str!("input.txt").lines();
    let mut hands = lines.fold(HashMap::new(), |mut acc, line| {
        let mut line = line.split_ascii_whitespace();
        let hand = Hand {
            cards: line.next().unwrap(),
            bid: line.next().unwrap().parse::<usize>().unwrap(),
        };

        acc.entry(hand.hand_type())
            .and_modify(|x: &mut Vec<Hand>| x.push(hand.clone()))
            .or_insert(vec![hand]);

        acc
    });

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

#[derive(Clone, Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
}

impl Hand<'_> {
    fn as_number(&self) -> usize {
        self.cards
            .chars()
            .enumerate()
            .fold(0, |mut acc, (i, card)| {
                acc += 13usize.pow(4 - i as u32) * Self::card_order(&card);

                acc
            })
    }

    fn hand_type(&self) -> HandType {
        let cards = self.cards.to_string();

        let cards = cards.chars().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let mut ranks = cards.values().copied().collect::<Vec<usize>>();
        ranks.sort_by(|a, b| b.cmp(a));

        if ranks[0] == 5 {
            HandType::FiveOfAKind
        } else if ranks[0] == 4 {
            HandType::FourOfAKind
        } else if ranks[0] == 3 && ranks[1] == 2 {
            HandType::FullHouse
        } else if ranks[0] == 3 {
            HandType::ThreeOfAKind
        } else if ranks[0] == 2 && ranks[1] == 2 {
            HandType::TwoPair
        } else if ranks[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn card_order(card: &char) -> usize {
        match card {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
