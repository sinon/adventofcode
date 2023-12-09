use std::collections::HashMap;

use graph::prelude::{DirectedCsrGraph, DirectedNeighbors, GraphBuilder};
use nom::{bytes::complete::tag, character::complete::alpha1, sequence::separated_pair, IResult};

fn main() {
    let input = include_str!("./input.txt");
    let p1_result = p1(input);
    dbg!(p1_result);
}

fn parse_node_edges(input: &str) -> IResult<&str, ((&str, &str), (&str, &str))> {
    // AAA = (BBB, CCC)
    let (input, (parent_node, (edge_1, edge_2))) = separated_pair(
        alpha1,
        tag(" = "),
        separated_pair(alpha1, tag(", "), alpha1),
    )(input)?;
    Ok((input, ((parent_node, edge_1), (parent_node, edge_2))))
}

fn p1(input: &str) -> i32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let _ = lines.next();
    let mut nodes: Vec<String> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for ln in lines {
        let cleaned_ln = ln.replace("(", "").replace(")", "");

        let (_, ((p_node, ed_1), (_, ed_2))) = parse_node_edges(&cleaned_ln).unwrap();
        if !nodes.iter().any(|i| i.clone() == p_node.to_string()) {
            nodes.push(p_node.to_string());
        }
        if !nodes.iter().any(|i| i.clone() == ed_1.to_string()) {
            nodes.push(ed_1.to_string());
        }
        if !nodes.iter().any(|i| i.clone() == ed_2.to_string()) {
            nodes.push(ed_2.to_string());
        }
        println!("PNode:{} Edge1: {} Edge2: {}", p_node, ed_1, ed_2);
        // println!("Nodes: {:?}", nodes);
        let p_id = nodes
            .iter()
            .position(|r| r.clone() == p_node.to_string())
            .unwrap();
        let ed1_id = nodes
            .iter()
            .position(|r| r.clone() == ed_1.to_string())
            .unwrap();
        let ed2_id = nodes
            .iter()
            .position(|r| r.clone() == ed_2.to_string())
            .unwrap();
        edges.push((p_id, ed1_id));
        edges.push((p_id, ed2_id));
    }
    let mut node_name_lookup: HashMap<usize, String> = HashMap::new();
    let mut node_id_lookup: HashMap<String, usize> = HashMap::new();
    for (idx, name) in nodes.into_iter().enumerate() {
        node_id_lookup.insert(name.clone(), idx);
        node_name_lookup.insert(idx, name);
    }
    let graph: DirectedCsrGraph<usize> = GraphBuilder::new().edges(edges).build();
    let mut current_node_id = 0;
    let terminus_node_id = node_id_lookup.get("ZZZ").unwrap().clone();
    let mut count = 0;
    for ch in instructions.chars() {
        let mut n_edges = graph.out_neighbors(current_node_id);
        let l = n_edges.next().unwrap();
        let r = n_edges.next().unwrap();
        println!(
            "At Node: {} Children: Left::{} Right::{}",
            node_name_lookup.get(&current_node_id).unwrap(),
            node_name_lookup.get(l).unwrap(),
            node_name_lookup.get(r).unwrap()
        );
        count += 1;
        match ch {
            'L' => current_node_id = *l,
            'R' => current_node_id = *r,
            _ => unreachable!("Unknown direction {}", ch),
        }
        if current_node_id == terminus_node_id {
            break;
        }
    }
    count
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
