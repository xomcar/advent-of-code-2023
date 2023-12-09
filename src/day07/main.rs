use std::cmp::Ordering;
use std::collections::HashMap;
use std::{fs, process};

const N_CARDS: usize = 5;

fn main() {
    let file = fs::read_to_string("data/day07.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
    let answer = solution(&file);
    println!("{}", answer.0);
    println!("{}", answer.1);
}

fn solution(s: &str) -> (u64, u64) {
    // parse cards
    let mut hands: Vec<_> = s.lines().map(|line| Hand::from_str(line)).collect();
    hands.sort();
    let mut index = 1;
    let winnings = hands
        .iter()
        .map(|hand| {
            index += 1;
            hand.bet * (index - 1)
        })
        .sum();
    return (winnings, 0);
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialOrd, PartialEq, Eq, Debug)]
struct Hand {
    kind: HandType,
    cards: [u8; N_CARDS],
    bet: u64,
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        let values = s.split_whitespace().collect::<Vec<&str>>();
        let hand = values[0];
        let bet = values[1].parse::<u64>().unwrap();
        let mut cards = [0, 0, 0, 0, 0];
        // populate cards
        let mut seeds = HashMap::new();
        for (i, c) in hand.chars().enumerate() {
            let value;
            if c.is_digit(10) {
                value = c.to_digit(10).unwrap().try_into().unwrap()
            } else {
                value = match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => 0,
                };
            }
            cards[i] = value;
            seeds.entry(value).and_modify(|e| *e += 1).or_insert(1);
        }

        let kind: HandType;
        kind = compute_kind(seeds);

        Hand { kind, cards, bet }
    }
}

fn compute_kind(seeds: HashMap<u8, i32>) -> HandType {
    let mut amounts = seeds.values().collect::<Vec<_>>();
    amounts.sort_by(|a, b| b.cmp(a));
    let kind: HandType;
    if *amounts[0] == 5 {
        kind = HandType::FiveOfAKind
    } else if *amounts[0] == 4 {
        kind = HandType::FourOfAKind
    } else if *amounts[0] == 3 {
        if *amounts[1] == 2 {
            kind = HandType::FullHouse
        } else {
            kind = HandType::ThreeOfAKind
        }
    } else if *amounts[0] == 2 {
        if *amounts[1] == 2 {
            kind = HandType::TwoPair
        } else {
            kind = HandType::OnePair
        }
    } else {
        kind = HandType::HighCard
    }
    kind
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind > other.kind {
            Ordering::Greater
        } else if self.kind < other.kind {
            Ordering::Less
        } else {
            for i in 0..N_CARDS {
                if self.cards[i] == other.cards[i] {
                    continue;
                } else if self.cards[i] > other.cards[i] {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{solution, Hand, HandType};

    #[test]
    fn test() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = 6440;
        assert_eq!(solution(input).0, output);
    }

    #[test]
    fn test_kind() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        let h1 = Hand {
            kind: HandType::FiveOfAKind,
            cards: [1, 2, 3, 4, 5],
            bet: 0,
        };
        let h2 = Hand {
            kind: HandType::FourOfAKind,
            cards: [1, 2, 3, 4, 5],
            bet: 0,
        };
        let h3 = Hand {
            kind: HandType::FiveOfAKind,
            cards: [2, 2, 3, 4, 5],
            bet: 0,
        };
        assert!(h1 > h2);
        assert!(h1 < h3);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Hand::from_str("32T3K 20", true),
            Hand {
                bet: 20,
                kind: HandType::OnePair,
                cards: [3, 2, 10, 3, 13]
            }
        )
    }
}
