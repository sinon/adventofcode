use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part_1(input);
    println!("Part 1 Answer: {}", part1);
    let input = include_str!("input.txt");
    let p2: i32 = part_2(input);
    println!("Part 2 Answer: {}", p2);
}

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"(mul\()(\d+).(\d+)\)").unwrap();
    let mut results = vec![];
    for (_, [_, int1, int2]) in re.captures_iter(input).map(|c| c.extract()) {
        let i1: i32 = int1.parse().unwrap();
        let i2: i32 = int2.parse().unwrap();
        results.push((i1, i2));
    }
    results.iter().map(|(x, y)| x * y).sum()
}

fn part_2(input: &str) -> i32 {
    let do_re = Regex::new(r"(do\(\))").unwrap();
    let dont_re = Regex::new(r"(don't\(\))").unwrap();

    let input = do_re.replace_all(input, "\ndo()");
    let input = dont_re.replace_all(&input, "\ndon't()");

    let mut is_enabled = true;
    let mut sum = 0;
    for ln in input.lines() {
        dbg!(ln, ln.contains("don't()"), ln.contains("do()"));
        if ln.contains("don't()") {
            is_enabled = false;
        }
        if ln.contains("do()") {
            is_enabled = true
        }
        if ln.contains("dont()") && ln.contains("do()") {
            panic!()
        }
        if is_enabled {
            sum += part_1(ln);
        }
    }
    sum
}

#[test]
fn test_part_1() {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part_1(test_input), 161);
}

#[test]
fn test_part_2() {
    let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part_2(test_input), 48);
}
