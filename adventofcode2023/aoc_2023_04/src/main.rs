use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{digit1, multispace1},
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

#[derive(PartialEq, Debug)]
struct ScratchCard {
    id: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
}

impl ScratchCard {
    fn winning_matches(&self) -> Vec<i32> {
        let mut intersect_result: Vec<i32> = self.card_numbers.clone();

        let unique_a: HashSet<i32> = self.winning_numbers.clone().into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .copied()
            .collect::<Vec<_>>();
        intersect_result
    }
    fn winning_score(&self) -> i32 {
        let matches = self.winning_matches();
        let mut total = 0;
        for _ in matches.iter() {
            if total == 0 {
                total = 1;
            } else {
                total *= 2;
            }
        }
        total
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
    let winning_numbers = winning_numbers
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let card_numbers = card_numbers
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
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
    let mut total = 0;
    for ln in lines {
        let (_, sc) = parse_scatchcard(ln).unwrap();
        let score = sc.winning_score();
        total += score;
    }
    println!("Part 1 Score: {}", total);
}
