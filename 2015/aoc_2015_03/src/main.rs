use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let p1 = p1(input);
    dbg!(p1);
    let input = include_str!("./input.txt");
    let p2 = p2(input);
    dbg!(p2);
}

fn p1(input: &str) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut visit_counts: HashMap<(i32, i32), i32> = HashMap::new();
    visit_counts.insert((x, y), 1);
    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => unreachable!(),
        }
        visit_counts
            .entry((x, y))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    visit_counts.len() as i32
}

fn p2(input: &str) -> i32 {
    let (mut x, mut y) = (0, 0);
    let (mut rob_x, mut rob_y) = (0, 0);
    let mut visit_counts: HashMap<(i32, i32), i32> = HashMap::new();
    visit_counts.insert((x, y), 2);
    let mut is_santa: bool = true;
    for c in input.chars() {
        match (c, is_santa) {
            ('^', true) => y += 1,
            ('v', true) => y -= 1,
            ('>', true) => x += 1,
            ('<', true) => x -= 1,
            ('^', false) => rob_y += 1,
            ('v', false) => rob_y -= 1,
            ('>', false) => rob_x += 1,
            ('<', false) => rob_x -= 1,
            _ => unreachable!(),
        }
        if is_santa {
            visit_counts
                .entry((x, y))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        } else {
            visit_counts
                .entry((rob_x, rob_y))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        is_santa = !is_santa;
    }
    visit_counts.len() as i32
}

#[test]
fn test_p1() {
    assert_eq!(p1(">"), 2);
    assert_eq!(p1("^>v<"), 4);
    assert_eq!(p1("^v^v^v^v^v"), 2);
}
