fn main() {
    let input = include_str!("./input.txt");
    let p1 = calc_floor(input);
    dbg!(p1);
    let p2 = basement_index(input);
    dbg!(p2);
}

fn calc_floor(input: &str) -> i32 {
    input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

fn basement_index(input: &str) -> usize {
    let mut current_floor = 0;
    for (idx, dir) in input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .enumerate()
    {
        current_floor += dir;
        if current_floor == -1 {
            return idx + 1;
        }
    }

    0
}

#[test]
fn test_part1() {
    assert_eq!(calc_floor("(())"), 0);
    assert_eq!(calc_floor("()()"), 0);
    assert_eq!(calc_floor("((("), 3);
    assert_eq!(calc_floor("(()(()("), 3);
    assert_eq!(calc_floor("))((((("), 3);
    assert_eq!(calc_floor("())"), -1);
    assert_eq!(calc_floor("))("), -1);
    assert_eq!(calc_floor(")))"), -3);
    assert_eq!(calc_floor(")())())"), -3);
}

#[test]
fn test_part2() {
    assert_eq!(basement_index(")"), 1);
    assert_eq!(basement_index("()())"), 5);
}
