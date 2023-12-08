use core::panic;
use std::ops::Deref;

use itertools::{Itertools, Position};

#[derive(Debug)]
struct Hand<'a> {
    _cards: &'a str,
    bid: u32,
    score: (HandType, (u32, u32, u32, u32, u32)),
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn score_hand(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();

    let values = if let Some(joker_count) = counts.get(&'J') {
        if *joker_count == 5 {
            "5".to_string()
        } else {
            counts
                .iter()
                .filter_map(|(key, val)| (key != &'J').then_some(val))
                .sorted()
                .with_position()
                .map(|(pos, value)| match pos {
                    Position::Last | Position::Only => value + joker_count,
                    _ => *value,
                })
                .join("")
        }
    } else {
        counts.values().sorted().join("")
    };

    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    };
    let card_scores = hand
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            val => val.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();

    (hand_type, card_scores)
}

fn process(input: &str) -> String {
    let hands = input
        .lines()
        .map(|f| {
            let (cards, bid) = f.split_once(" ").unwrap();
            Hand {
                _cards: cards,
                bid: bid.parse().expect("should be a number"),
                score: score_hand(cards),
            }
        })
        // sort by hand type (x.score.1), then if matches are found, sort by hand score (x.score.1)
        .sorted_by_key(|h| (h.score.0 as u8, h.score.1))
        // .rev()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum::<u32>();

    hands.to_string()
}

fn main() {
    let input = include_str!("./input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input));
    }
}
