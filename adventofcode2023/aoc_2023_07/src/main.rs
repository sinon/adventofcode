use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[repr(u8)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<char> for Rank {
    fn from(item: char) -> Rank {
        match item {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' | '1' => Rank::Ace,
            _ => unreachable!("Invalid rank"),
        }
    }
}

#[derive(Debug, Eq)]
struct CamelHand {
    cards: Vec<Rank>,
    bet: i64,
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ht1 = self.classify();
        let ht2 = other.classify();
        if ht1 != ht2 {
            return ht1.cmp(&ht2);
        } else {
            for zip in self.cards.iter().zip(other.cards.iter()) {
                if zip.0 != zip.1 {
                    return zip.0.cmp(zip.1);
                }
            }
        }
        unreachable!("");
    }
}

impl CamelHand {
    fn new(line: &str) -> CamelHand {
        let mut split = line.split(' ');
        let cards = split
            .next()
            .unwrap()
            .chars()
            .map(Rank::from)
            .collect::<Vec<_>>();
        let bet = split.next().unwrap().parse::<i64>().unwrap();
        CamelHand { cards, bet }
    }

    fn classify(&self) -> HandType {
        let counter: BTreeMap<Rank, i64> = BTreeMap::new();
        let counter = self
            .cards
            .iter()
            .fold(counter, |mut acc, &item| match acc.get(&item) {
                Some(x) => {
                    acc.insert(item, x + 1);
                    acc
                }
                None => {
                    acc.insert(item, 1);
                    acc
                }
            });
        let mut x: Vec<&i64> = counter.values().collect();
        x.sort();
        match x.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::Pair,
            [_, 2, 2] => HandType::TwoPair,
            [_, _, 3] => HandType::ThreeOfAKind,
            [_, 3] => HandType::FullHouse,
            [_, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => {
                unreachable!("Invalid hand: {:?}", x);
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let p1_answer = p1(input);
    println!("Part 1: {}", p1_answer);
}

fn p1(input: &str) -> i64 {
    let mut hands = input.lines().map(CamelHand::new).collect::<Vec<_>>();
    hands.sort();
    let mut total = 0;
    for (i, h) in hands.iter().enumerate() {
        println!(
            "Rank: {} {:?} {:?} {:?}",
            i + 1,
            h.cards,
            h.bet,
            h.classify()
        );
        total += h.bet * (i as i64 + 1);
    }
    total
}

#[test]
fn test_p1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(p1(input), 6440);
}
