use std::str::Lines;

fn main() {
    let lines = include_str!("input.txt").lines();
    let part1 = part_1(lines);
    println!("Part 1 Answer: {}", part1);
    let lines = include_str!("input.txt").lines();
    let p2: usize = lines.map(part_2).sum();
    println!("Part 2 Answer: {}", p2);
}

fn part_2(line: &str) -> usize {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    let d1 = line.chars().find(|c| c.is_numeric()).unwrap();
    let d2 = line.chars().rev().find(|c| c.is_numeric()).unwrap();
    format!("{d1}{d2}").parse().unwrap()
}

fn part_1(lines: Lines) -> u32 {
    let mut total = 0;
    for ln in lines {
        println!("{}", ln);
        let mut first: Option<char> = None;
        let mut second: Option<char> = None;
        for char in ln.chars() {
            if char.is_numeric() {
                if first.is_some() {
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
    let total: usize = test_input.map(part_2).sum();
    assert_eq!(total, 281 + 83 + 79 + 18 + 24 + 87 + 83 + 99 + 28);
}
