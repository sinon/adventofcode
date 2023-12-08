use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Rank {
    fn ordinal(&self) -> usize {
        match *self {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        let ord1 = self.ordinal();
        let ord2 = other.ordinal();
        if ord1 < ord2 {
            return Ordering::Less;
        } else if ord1 > ord2 {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    ThreeOfAKind,
    TwoPair,
    FullHouse,
    Pair,
    HighCard,
}

impl From<char> for Rank {
    fn from(item: char) -> Rank {
        let rank = match item {
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
        };
        rank
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
        let ht1 = self.classify();
        let ht2 = other.classify();
        if ht1 != ht2 {
            return ht1.partial_cmp(&ht2);
        } else {
            for zip in self.cards.iter().zip(other.cards.iter()) {
                if zip.0 != zip.1 {
                    return zip.0.partial_cmp(zip.1);
                }
            }
        }
        self.classify().partial_cmp(&other.classify())
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
        self.classify().cmp(&other.classify())
    }
}

impl CamelHand {
    fn new(line: &str) -> CamelHand {
        let mut split = line.split(" ");
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
        x.reverse();
        match x.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] => HandType::Pair,
            [2, 2, 1] => HandType::TwoPair,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [3, 2] => HandType::FullHouse,
            [4, 1] => HandType::FourOfAKind,
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
    hands.reverse();
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
