use std::{
    collections::{BTreeMap, HashSet},
    str::Lines,
};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{digit1, multispace1},
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

#[derive(PartialEq, Debug, Clone)]
struct ScratchCard {
    id: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
}

impl ScratchCard {
    fn winning_matches(&self) -> Vec<i32> {
        let mut intersect_result: Vec<i32> = self.card_numbers.clone();

        let winning: HashSet<i32> = self.winning_numbers.clone().into_iter().collect();
        intersect_result = winning
            .intersection(&intersect_result.into_iter().collect())
            .copied()
            .collect::<Vec<_>>();
        intersect_result
    }
    fn winning_score(&self) -> i32 {
        let matches = self.winning_matches();
        if matches.is_empty() {
            return 0;
        }
        2i32.pow((matches.len() - 1) as u32)
    }
}

fn take_till_num(input: &str) -> IResult<&str, &str> {
    let parser = take_till(|c: char| c.is_ascii_digit());
    parser(input)
}

fn parse_space_seperated_integer_lists(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let mut parser = pair(take_till_num, separated_list1(multispace1, digit1));
    parser(input)
}

fn into_num_vec(vec: Vec<&str>) -> Vec<i32> {
    vec.iter().map(|c| c.parse::<i32>().unwrap()).collect()
}

fn parse_scatchcard(input: &str) -> IResult<&str, ScratchCard> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = take_till_num(input)?;
    let (input, card_id) = digit1(input)?;
    let (input, _) = take_till_num(input)?;
    let (input, ((_, winning_numbers), (_, card_numbers))) = separated_pair(
        parse_space_seperated_integer_lists,
        tag(" | "),
        parse_space_seperated_integer_lists,
    )(input)?;
    let winning_numbers = into_num_vec(winning_numbers);
    let card_numbers = into_num_vec(card_numbers);
    Ok((
        input,
        ScratchCard {
            id: card_id.parse().unwrap(),
            winning_numbers,
            card_numbers,
        },
    ))
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let (total, total_cards) = part_2(lines);
    println!("Part 1 Score: {}", total);
    println!("Part 2 Score: {}", total_cards);
}

fn part_2(input: Lines) -> (i32, i32) {
    let mut total = 0;
    let mut sc_lookup: BTreeMap<i32, ScratchCard> = BTreeMap::new();
    let mut winners_lookup: BTreeMap<i32, Vec<ScratchCard>> = BTreeMap::new();

    // Parse the input into a scratchcards
    for ln in input {
        let (_, sc) = parse_scatchcard(ln).unwrap();
        sc_lookup.insert(sc.id, sc.clone());
        winners_lookup.insert(sc.id, vec![]);
        total += sc.winning_score();
    }
    for (sc_id, sc) in &sc_lookup {
        let winning_match_count = sc.winning_matches().len() as i32;
        // println!("{} has {} winning matches", sc_id, winning_match_count);
        let extra_cards = winners_lookup.get(sc_id).unwrap().len();
        // println!(
        //     "{} has {} extra cards from past winning cards to score",
        //     sc_id, extra_cards
        // );
        for _ in 0..extra_cards + 1 {
            // println!("{}", i);
            for winner_id in (sc_id + 1)..(sc_id + 1 + winning_match_count) {
                // println!("Winner ID: {}", winner_id);
                let bonus = sc_lookup.get(&winner_id).unwrap();
                winners_lookup
                    .entry(winner_id)
                    .and_modify(|i| i.push(bonus.clone()));
            }
        }
    }
    let mut total_cards = sc_lookup.len();
    for (_, extra_winning) in winners_lookup {
        total_cards += extra_winning.len();
    }
    (total, total_cards as i32)
}

#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .lines();
    let (total, total_cards) = part_2(input);
    assert_eq!(total, 13);
    assert_eq!(total_cards, 30);
}
