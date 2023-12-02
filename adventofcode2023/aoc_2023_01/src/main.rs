use fancy_regex::Regex;
use std::{collections::BTreeMap, str::Lines};

fn main() {
    let lines = include_str!("input.txt").lines();
    let part1 = part_1(lines);
    println!("Part 1 Answer: {}", part1);
    let lines = include_str!("input.txt").lines();
    let part2 = part_2(lines);
    println!("Part 2 Answer: {}", part2);
}

fn part_1(lines: Lines) -> u32 {
    let mut total = 0;
    for ln in lines {
        println!("{}", ln);
        let mut first: Option<char> = None;
        let mut second: Option<char> = None;
        for char in ln.chars() {
            if char.is_numeric() {
                if let Some(_) = first {
                    second = Some(char);
                } else {
                    first = Some(char);
                }
            }
        }
        println!("{:?} {:?}", first, second);
        if let Some(value) = first {
            if let Some(v2) = second {
                total += format!("{}{}", value, v2).parse::<u32>().unwrap();
            } else {
                total += format!("{}{}", value, value).parse::<u32>().unwrap();
            }
        }
    }
    total
}

fn get_nums_with_regex(ln: &str) -> (&str, &str) {
    println!("{}", ln);
    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|\\d))").unwrap();
    println!("{:?}", re);
    let mut results = vec![];
    for result in re.find_iter(ln) {
        match result {
            Ok(m) => {
                println!("{:?}", m);
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
        // if result.is_ok() {
        //     let x = result.unwrap().as_str();
        //     println!("{:?}", x);
        //     // println!("{:?}", result.unwrap());
        //     // results.push(result.unwrap().as_str());
        // }
    }
    println!("{:?}", results);
    if results.len() > 1 {
        (results[0], results[results.len() - 1])
    } else {
        (results[0], results[0])
    }
    //
}

fn convert_num_words_to_digits(ln: &str) -> String {
    let mut num_words = BTreeMap::new();
    num_words.insert("one", "1");
    num_words.insert("two", "2");
    num_words.insert("three", "3");
    num_words.insert("four", "4");
    num_words.insert("five", "5");
    num_words.insert("six", "6");
    num_words.insert("seven", "7");
    num_words.insert("eight", "8");
    num_words.insert("nine", "9");
    let mut s = ln.to_string();
    let mut insertions = BTreeMap::new();
    for (to_find, to_replace) in num_words {
        let idx = s.find(to_find);
        if let Some(idx) = idx {
            insertions.insert(idx, to_replace);
            let idx2 = &s[idx + to_find.len()..s.len()].find(to_find);
            if let Some(idx2) = idx2 {
                insertions.insert(idx + idx2, to_replace);
                let idx3 = &s[idx2 + to_find.len()..s.len()].find(to_find);
                if let Some(idx3) = idx3 {
                    insertions.insert(idx + idx2 + idx3, to_replace);
                    let idx4 = &s[idx3 + to_find.len()..s.len()].find(to_find);
                    if let Some(idx4) = idx4 {
                        insertions.insert(idx + idx2 + idx3 + idx4, to_replace);
                        let idx5 = &s[idx4 + to_find.len()..s.len()].find(to_find);
                        if let Some(idx5) = idx5 {
                            insertions.insert(idx + idx2 + idx3 + idx4 + idx5, to_replace);
                            let idx6 = &s[idx5 + to_find.len()..s.len()].find(to_find);
                            if let Some(idx6) = idx6 {
                                insertions
                                    .insert(idx + idx2 + idx3 + idx4 + idx5 + idx6, to_replace);
                            }
                        }
                    }
                }
            }
            // s.insert_str(idx, to_replace)
        }
    }
    for (offset, (idx, to_replace)) in insertions.iter().enumerate() {
        println!("========");
        println!(
            "idx: {} to_replace:{} offset:{} s:{}",
            idx, to_replace, offset, s
        );
        s.insert_str((*idx + offset).clamp(0, s.len()), to_replace);
        println!("{}", s);
        println!("========");
    }
    s
}

fn part_2(lines: Lines) -> u32 {
    let mut total = 0;
    for ln in lines {
        println!("{}", ln);
        let cleaned_ln = convert_num_words_to_digits(ln);
        println!("{}", cleaned_ln);
        let mut first: Option<char> = None;
        let mut second: Option<char> = None;
        for char in cleaned_ln.chars() {
            if char.is_numeric() {
                if let Some(_) = first {
                    second = Some(char);
                } else {
                    first = Some(char);
                }
            }
        }
        println!("{:?} {:?}", first, second);
        if let Some(value) = first {
            if let Some(v2) = second {
                total += format!("{}{}", value, v2).parse::<u32>().unwrap();
            } else {
                total += format!("{}{}", value, value).parse::<u32>().unwrap();
            }
        }
    }
    total
}

fn part2_with_regex(lines: Lines) -> u32 {
    let mut total = 0;
    for ln in lines {
        let (num1, num2) = get_nums_with_regex(ln);
        println!("{} {}", num1, num2);
        total += format!("{}{}", num1, num2).parse::<u32>().unwrap();
    }
    total
}

#[test]
fn test_part_1() {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .lines();
    assert_eq!(part_1(test_input), 142);
}

#[test]
fn test_part2() {
    let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eighthree
sevenine
oneight
xtwone3four
pxreightwo7
eightmkmdtvkctkvptsbckzpnkhpskdmp3
tgppgp9
twoneight"
        .lines();
    assert_eq!(
        part_2(test_input),
        281 + 83 + 79 + 18 + 24 + 87 + 83 + 99 + 28
    );
}

#[test]
fn test_part_2_2() {
    let test_input = "three2fiveonexrllxsvfive".lines();
    assert_eq!(part_2(test_input), 35);

    let test_input = "xthzlbsjvz4dlg9fiveseven7seven".lines();
    assert_eq!(part_2(test_input), 47);

    let test_input = "xthzlbsjvz4dlg9fiveseven7sevenseven".lines();
    assert_eq!(part_2(test_input), 47);

    let test_input = "xthzlbsjvzdlgseven".lines();
    assert_eq!(part_2(test_input), 77);

    let test_input = "seightninepjr3mjkgq3ckxzlqkkxpxdpkk".lines();
    assert_eq!(part_2(test_input), 83);
    let test_input = "nsmlqsixfiveng65jjblflfone".lines();
    assert_eq!(part_2(test_input), 61);
    let test_input = "mtthreeclxhfivep8threelh".lines();
    assert_eq!(part_2(test_input), 33);
}

#[test]
fn test_part2_regex() {
    let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eighthree
sevenine
oneight
xtwone3four
pxreightwo7
eightmkmdtvkctkvptsbckzpnkhpskdmp3
tgppgp9
twoneight"
        .lines();
    assert_eq!(
        part2_with_regex(test_input),
        281 + 83 + 79 + 18 + 24 + 87 + 83 + 99 + 28
    );
}

#[test]
fn test_part_2_2_regex() {
    let test_input = "three2fiveonexrllxsvfive".lines();
    assert_eq!(part2_with_regex(test_input), 35);

    let test_input = "xthzlbsjvz4dlg9fiveseven7seven".lines();
    assert_eq!(part2_with_regex(test_input), 47);

    let test_input = "xthzlbsjvz4dlg9fiveseven7sevenseven".lines();
    assert_eq!(part2_with_regex(test_input), 47);

    let test_input = "xthzlbsjvzdlgseven".lines();
    assert_eq!(part2_with_regex(test_input), 77);

    let test_input = "seightninepjr3mjkgq3ckxzlqkkxpxdpkk".lines();
    assert_eq!(part2_with_regex(test_input), 83);
    let test_input = "nsmlqsixfiveng65jjblflfone".lines();
    assert_eq!(part2_with_regex(test_input), 61);
}
