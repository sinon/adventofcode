#![feature(iter_map_windows)]
use std::str::Lines;

fn main() {
    let lines = include_str!("input.txt").lines();
    let part1 = part_1(lines);
    println!("Part 1 Answer: {}", part1);
    let lines = include_str!("input.txt").lines();
    let p2 = part_2(lines);
    println!("Part 2 Answer: {}", p2);
}
fn parse_ln(line: &str) -> Vec<i32> {
    line.split(" ").map(|x| x.parse().unwrap()).collect()
}

fn is_safe(values: Vec<i32>) -> bool {
    let all_increasing_in_threshold = values
        .iter()
        .map_windows(|[x, y]| x > y && (**x - **y) < 4)
        .all(|x| x);
    let all_decreasing_in_threshold = values
        .iter()
        .map_windows(|[x, y]| x < y && (**y - **x) < 4)
        .all(|x| x);
    all_increasing_in_threshold || all_decreasing_in_threshold
}

fn is_safe_2(values: Vec<i32>) -> bool {
    let length = values.len();
    for j in 0..length {
        let (tail, rest) = (&values[0..j], &values[j + 1..length]);
        let new = [tail, rest].concat();
        if is_safe(new) {
            return true;
        }
    }
    false
}

fn part_1(lines: Lines) -> usize {
    lines
        .into_iter()
        .map(|x| is_safe(parse_ln(x)))
        .filter(|x| *x)
        .count()
}
fn part_2(lines: Lines) -> usize {
    lines
        .into_iter()
        .map(|x| parse_ln(x))
        .filter(|x| is_safe_2(x.to_vec()))
        .count()
}

#[test]
fn test_part_1() {
    let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .lines();
    assert_eq!(part_1(test_input), 2);
}

#[test]
fn test_part_2() {
    let test_input = "7 6 4 2 1
1 2 7 8 9
1 2 7 8 9 10
9 7 6 2 1
10 9 7 6 2 1
1 3 2 4 5
1 2 3 4 5 100
1 3 2 4 5 6
8 6 4 4 1
9 8 6 4 4 1
1 3 6 7 9"
        .lines();
    assert_eq!(part_2(test_input), 7);
}
