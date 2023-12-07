use std::{cmp::Reverse, fmt::Display, fs, marker::PhantomData, str::FromStr};

struct NormalHand<'a>(&'a str);
struct JokerHand<'a>(&'a str);

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|c| match c {
                14 => 'A',
                13 => 'K',
                12 => 'Q',
                11 => 'J',
                10 => 'T',
                0 => 'j',
                x => std::char::from_digit(*x as u32, 10).unwrap(),
            })
            .collect::<String>();
        write!(f, "{} {}", cards, self.bid)
    }
}

impl<'a> From<NormalHand<'a>> for Hand {
    fn from(NormalHand(s): NormalHand<'a>) -> Self {
        let (s, bid) = s.split_once(' ').unwrap();
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                x => x.to_digit(10).unwrap() as u8,
            })
            .collect::<Vec<u8>>();
        Hand {
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}

impl<'a> From<JokerHand<'a>> for Hand {
    fn from(JokerHand(s): JokerHand<'a>) -> Self {
        let (s, bid) = s.split_once(' ').unwrap();
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                x => x.to_digit(10).unwrap() as u8,
            })
            .collect::<Vec<u8>>();
        Hand {
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
    fn of(hand: &Hand) -> HandType {
        let mut counts = [0; 15];
        let mut joker_count = 0;
        for card in &hand.cards {
            if *card == 0 {
                joker_count += 1;
                continue;
            }
            counts[*card as usize] += 1;
        }
        let mut counts = counts.iter().enumerate().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|(_, count)| Reverse(**count));
        let count = counts[0].1;
        match count + joker_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if *counts[1].1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if *counts[1].1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = HandType::of(self);
        let other_type = HandType::of(other);
        if self_type != other_type {
            return Some(self_type.cmp(&other_type));
        }

        Some(self.cards.cmp(&other.cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn calculate_score(mut hands: Vec<Hand>) -> u64 {
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid as u64)
        .sum()
}

fn part_one(lines: &[&str]) -> u64 {
    let hands = lines
        .iter()
        .map(|l| Hand::from(NormalHand(l)))
        .collect::<Vec<Hand>>();

    calculate_score(hands)
}

fn part_two(lines: &[&str]) -> u64 {
    let hands = lines
        .iter()
        .map(|l| Hand::from(JokerHand(l)))
        .collect::<Vec<Hand>>();

    calculate_score(hands)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[&str; 5] = &[
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    #[test]
    fn test_day_one() {
        assert_eq!(part_one(TEST_INPUT), 6440)
    }

    #[test]
    fn test_day_two() {
        assert_eq!(part_two(TEST_INPUT), 5905)
    }
}
