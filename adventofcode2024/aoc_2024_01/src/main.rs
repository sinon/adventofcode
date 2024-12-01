use std::{
    collections::{BTreeSet, HashMap},
    iter::zip,
    str::Lines,
};

use nom::{IResult, bytes::complete::tag, character::complete::digit1, sequence::tuple};

fn main() {
    let lines = include_str!("input.txt").lines();
    let part1 = part_1(lines);
    println!("Part 1 Answer: {}", part1);
    let lines = include_str!("input.txt").lines();
    let p2: i32 = part_2(lines);
    println!("Part 2 Answer: {}", p2);
}

fn parse_ln(line: &str) -> IResult<&str, (i32, i32)> {
    // 3 4
    let (line, (int1, _, int2)) = tuple((digit1, tag("   "), digit1))(line)?;
    let int1 = int1.parse().unwrap();
    let int2 = int2.parse().unwrap();
    Ok((line, (int1, int2)))
}

fn part_1(lines: Lines) -> i32 {
    let (mut r1, mut r2) = (BTreeSet::new(), BTreeSet::new());
    for ln in lines {
        let (_, (i1, i2)) = parse_ln(ln).unwrap();
        r1.insert(i1);
        r2.insert(i2);
    }
    zip(r1, r2).map(|(x, y)| (x - y).abs()).sum()
}

fn part_2(lines: Lines) -> i32 {
    let mut r1 = Vec::new();
    let mut r2: HashMap<i32, i32> = HashMap::new();
    for ln in lines {
        let (_, (i1, i2)) = parse_ln(ln).unwrap();
        r1.push(i1);
        *r2.entry(i2).or_insert(0) += 1;
    }
    r1.iter().map(|i| *r2.entry(*i).or_default() * i).sum()
}

#[test]
fn test_part_1() {
    let test_input = "3   4
4   3
2   5
1   3
3   9
3   3"
        .lines();
    assert_eq!(part_1(test_input), 11);
}

#[test]
fn test_part_2() {
    let test_input = "3   4
4   3
2   5
1   3
3   9
3   3"
        .lines();
    assert_eq!(part_2(test_input), 31);
}
