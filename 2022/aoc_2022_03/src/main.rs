use std::collections::{HashMap, HashSet};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn common_chars(a: &str, b: &str) -> Vec<char> {
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };
    let set: HashSet<char> = shorter.chars().collect();
    longer.chars().filter(|x| set.contains(x)).collect()
}

fn common_chars_2(a: &str, b: &str) -> String {
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };
    let set: HashSet<char> = shorter.chars().collect();
    longer.chars().filter(|x| set.contains(x)).collect()
}

fn main_1() {
    let lines = include_str!("./input.txt").lines();
    let scores: Vec<(char, i32)> = ALPHABET.chars().into_iter().zip(1..53).collect();
    let score_lookup: HashMap<char, i32> = scores.into_iter().collect();
    println!("{:?}", &score_lookup);
    let mut total = 0;
    for ln in lines.into_iter() {
        let (first, second) = ln.split_at(ln.len() / 2);
        let overlap = common_chars(first, second);
        println!("{:?}", overlap);
        // let priority = overlap
        //     .into_iter()
        //     .fold(0, |acc, x| acc + score_lookup.get(&x).unwrap());
        total += score_lookup.get(&overlap[0]).unwrap();
    }
    println!("Total: {}", total);
}

fn main() {
    main_1();
    let scores: Vec<(char, i32)> = ALPHABET.chars().into_iter().zip(1..53).collect();
    let score_lookup: HashMap<char, i32> = scores.into_iter().collect();
    let lines = include_str!("./input.txt").lines();
    let mut counter = 1;
    let mut total = 0;
    let mut common = "".to_string();
    for ln in lines.into_iter() {
        if common.is_empty() {
            common = ln.to_string();
        }
        println!("{}", ln);
        let common_str = common_chars_2(ln, &common);
        println!("Common: {}", common_str);
        if counter == 3 {
            println!("=========");
            println!("{:?}", common_str);
            let first_char = common_str.chars().nth(0).unwrap();
            total += score_lookup.get(&first_char).unwrap();
            counter = 1;
            common = "".to_string();
        } else {
            counter += 1;
            common = common_str;
        }
    }
    println!("Total: {}", total);
}

#[test]
fn test() {
    let str1 = "abcdef";
    let str2 = "the quick brown fox";
    assert_eq!(common_chars(str1, str2), vec!['e', 'c', 'b', 'f']);
}
