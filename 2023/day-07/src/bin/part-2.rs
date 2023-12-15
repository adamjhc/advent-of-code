use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

#[derive(Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn parse(input: &str) -> HandType {
        let mut character_counts = input.chars().fold(HashMap::new(), |mut counts, char| {
            *counts.entry(char).or_insert(0) += 1;
            counts
        });

        let j_count = character_counts.remove(&'J');
        let mut counts: Vec<usize> = character_counts.into_values().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        if let Some(j_count) = j_count {
            match counts.get_mut(0) {
                Some(x) => *x += j_count,
                None => counts.push(j_count),
            }
        }

        match counts[0] {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 if counts[1] == 2 => Self::FullHouse,
            3 => Self::ThreeOfAKind,
            2 if counts[1] == 2 => Self::TwoPair,
            2 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(Clone, Copy)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn parse(char: char) -> Card {
        match char {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

struct Hand {
    hand: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

fn solve(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            Hand {
                hand: hand.chars().map(Card::parse).collect(),
                bid: bid.parse().unwrap(),
                hand_type: HandType::parse(hand),
            }
        })
        .collect();

    hands.sort_unstable_by(|a, b| {
        let type_ordering = (a.hand_type as isize).cmp(&(b.hand_type as isize));
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for (&card_a, &card_b) in a.hand.iter().zip(b.hand.iter()) {
            let card_ordering = (card_a as isize).cmp(&(card_b as isize));
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }

        unreachable!()
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 5905)
    }
}
