#![feature(iter_map_windows)]

use std::collections::HashMap;

fn is_nice(ln: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_found: Vec<char> = Vec::new();
    // contains at least vowels
    let mut vowel_count = 0;
    for c in ln.chars() {
        if VOWELS.contains(&c.to_ascii_lowercase()) {
            if !vowels_found.contains(&c.to_ascii_lowercase()) {
                vowels_found.push(c);
            }
            vowel_count += 1;
        }
    }
    // aa
    let dupe_chars = ln.chars().map_windows(|[x, y]| x == y).any(|c| c);

    // !(ab, cd, pq, xy)
    let bad_pairs = ["ab", "cd", "pq", "xy"];
    let no_bad_strs = bad_pairs
        .iter()
        .map(|x| ln.find(*x).is_some())
        .all(|x| x == false);
    // let x: Vec<bool> = bad_pairs.iter().map(|x| ln.find(x).is_some()).collect();
    if vowel_count > 2 && no_bad_strs && dupe_chars {
        return true;
    }
    false
}

fn filter_overlapping(coord_list: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut prev_end = 0;

    for &(start, end) in coord_list.iter() {
        if prev_end == 0 {
            prev_end = end;
            result.push((start, end));
            continue;
        }
        if start > prev_end {
            result.push((start, end));
            prev_end = end;
        }
    }

    result
}

fn is_nice_2(ln: &str) -> bool {
    let mut char_pairs_counter: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    for idx in 0..(ln.len() - 1) {
        let substring = &ln[idx..(idx + 2)];
        char_pairs_counter
            .entry(substring)
            .and_modify(|c| c.push((idx, idx + 1)))
            .or_insert(vec![(idx, idx + 1)]);
    }

    let has_multiple_pairs = char_pairs_counter.values().any(|x| {
        if x.len() > 1 {
            let y = filter_overlapping(x.clone());
            if y.len() > 1 {
                return true;
            }
            return false;
        }
        false
    });
    let dupe_chars = ln.chars().map_windows(|[x, _y, z]| x == z).any(|c| c);
    if has_multiple_pairs && dupe_chars {
        return true;
    }

    false
}

fn main() {
    let input = include_str!("./input.txt");
    let mut total_nice = 0;
    let mut total_nice_2 = 0;
    for ln in input.lines() {
        if is_nice(ln) {
            total_nice += 1;
        }
        if is_nice_2(ln) {
            total_nice_2 += 1;
        }
    }
    dbg!(total_nice);
    dbg!(total_nice_2);
}

#[test]
fn test_p1() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
}

#[test]
fn test_p2() {
    assert!(!is_nice_2("aaa"));
    dbg!("----");
    assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
    dbg!("----");
    assert!(is_nice_2("xxyxx"));
    dbg!("----");
    assert!(!is_nice_2("uurcxstgmygtbstg"));
    dbg!("----");
    assert!(!is_nice_2("ieodomkazucvgmuy"));
    dbg!("----");
    assert!(!is_nice_2("isaljhemltsdzlum"));
    dbg!("----");
}
