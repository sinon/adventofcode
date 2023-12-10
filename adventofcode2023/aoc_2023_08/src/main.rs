use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::alpha1, sequence::separated_pair, IResult};

fn main() {
    let input = include_str!("./input.txt");
    let p1_result = p1(input);
    dbg!(p1_result);
}

fn parse_node_edges(input: &str) -> IResult<&str, (&str, &str, &str)> {
    // AAA = (BBB, CCC)
    let (input, (parent_node, (edge_1, edge_2))) = separated_pair(
        alpha1,
        tag(" = "),
        separated_pair(alpha1, tag(", "), alpha1),
    )(input)?;
    Ok((input, (parent_node, edge_1, edge_2)))
}

fn p1(input: &str) -> i32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let _ = lines.next();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for ln in lines {
        let cleaned_ln = ln.replace(['(', ')'], "");

        let (_, (p_node, ed_1, ed_2)) = parse_node_edges(&cleaned_ln).unwrap();

        map.insert(p_node.to_owned(), (ed_1.to_owned(), ed_2.to_owned()));
    }

    let mut count = 0;
    let mut current_node = "AAA".to_owned();
    let end_node = "ZZZ".to_owned();
    loop {
        for c in instructions.chars() {
            if current_node == end_node {
                return count;
            }
            let node = map.get(&current_node).unwrap();
            match c {
                'L' => current_node = node.0.clone(),
                'R' => current_node = node.1.clone(),
                _ => {
                    unreachable!()
                }
            }
            count += 1;
        }
    }
}
#[test]
fn test_p1_1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(p1(input), 2);
}

#[test]
fn test_p1_2() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    println!("{}", input);
    assert_eq!(p1(input), 6);
}
