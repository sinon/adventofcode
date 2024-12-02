#![feature(iter_map_windows)]
use std::str::Lines;

fn main() {
    let lines = include_str!("input.txt").lines();
    let part1 = part_1(lines);
    println!("Part 1 Answer: {}", part1);
    let lines = include_str!("input.txt").lines();
    let p2: i32 = part_2(lines);
    println!("Part 2 Answer: {}", p2);
}

fn parse_ln(line: &str) -> Vec<i32> {
    line.split(" ").map(|x| x.parse().unwrap()).collect()
}

fn is_safe(values: Vec<i32>) -> bool {
    let all_increasing = values
        .iter()
        .map_windows(|[x, y]| x > y && (**x - **y) < 4)
        .all(|x| x);
    let all_decreasing = values
        .iter()
        .map_windows(|[x, y]| x < y && (**y - **x) < 4)
        .all(|x| x);
    all_increasing || all_decreasing
}

fn is_safe_2(values: Vec<i32>) -> bool {
    // Retain a copy of last value
    let mut last = 0;
    if let Some(x) = values.last().copied() {
        last = x;
    }
    let all_increasing: Vec<(i32, bool)> = values
        .iter()
        .map_windows(|[x, y]| (**x, x > y && (**x - **y) < 4))
        .collect();
    let inc_bad_levels_count = all_increasing
        .iter()
        .map(|(_, good)| !good)
        .filter(|is_bad| *is_bad)
        .count();
    if inc_bad_levels_count == 0 {
        return true;
    }

    let all_decreasing: Vec<(i32, bool)> = values
        .iter()
        .map_windows(|[x, y]| (**x, x < y && (**y - **x) < 4))
        .collect();

    let dec_bad_levels_count = all_decreasing
        .iter()
        .map(|(_, good)| !good)
        .filter(|is_bad| *is_bad)
        .count();
    if dec_bad_levels_count == 0 {
        return true;
    }
    dbg!(inc_bad_levels_count, dec_bad_levels_count);
    if inc_bad_levels_count > 1 && dec_bad_levels_count > 1 {
        return false;
    }
    // dbg!(
    //     &values,
    //     inc_bad_levels_count,
    //     dec_bad_levels_count,
    //     values.len(),
    //     all_increasing.len(),
    //     &all_increasing,
    //     all_decreasing.len(),
    //     &all_decreasing
    // );
    if inc_bad_levels_count == 1 {
        let add_last = all_increasing
            .last()
            .is_some_and(|(val, is_good)| (last - val).abs() < 4);
        let mut last_vec = vec![];
        if add_last {
            last_vec.push(last);
        }
        dbg!(add_last, &last_vec);
        let next_test: Vec<i32> = all_increasing
            .iter()
            .filter(|(_, is_good)| *is_good)
            .map(|(val, _)| *val)
            .chain(last_vec)
            .collect();
        let inc_2nd_level = is_safe(next_test);
        if inc_2nd_level == true {
            return true;
        }
    }
    if dec_bad_levels_count == 1 {
        dbg!(all_decreasing.last());
        let add_last = all_decreasing
            .last()
            .is_some_and(|(val, is_good)| (last - val).abs() < 4);
        let mut last_vec = vec![];
        if add_last {
            last_vec.push(last);
        }
        dbg!(&last_vec);
        let next_test_2: Vec<i32> = all_decreasing
            .iter()
            .filter(|(val, is_good)| *is_good)
            .map(|(val, _)| *val)
            .chain(last_vec)
            .collect();
        dbg!(&next_test_2);
        let dec_2nd_level = is_safe(next_test_2);
        if dec_2nd_level == true {
            return true;
        }
    }
    false
}

fn part_1(lines: Lines) -> i32 {
    lines
        .into_iter()
        .map(|x| is_safe(parse_ln(x)))
        .filter(|x| *x)
        .count() as i32
}
fn part_2(lines: Lines) -> i32 {
    lines
        .into_iter()
        .map(|x| is_safe_2(parse_ln(x)))
        .filter(|x| *x)
        .count() as i32
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

#[test]
fn test_bug() {
    assert!(is_safe_2(vec![1, 2, 3, 4, 5, 100]));
    assert!(is_safe_2(vec![100, 10, 9, 8]));
}
