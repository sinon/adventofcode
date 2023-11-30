use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::IResult;

fn parse_move_params(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag("move ")(input)?;
    let (input, move_count) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_id) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_id) = digit1(input)?;

    Ok((
        input,
        (
            move_count.parse().unwrap(),
            from_id.parse().unwrap(),
            to_id.parse().unwrap(),
        ),
    ))
}

/*
    [C]             [L]         [T]
    [V] [R] [M]     [T]         [B]
    [F] [G] [H] [Q] [Q]         [H]
    [W] [L] [P] [V] [M] [V]     [F]
    [P] [C] [W] [S] [Z] [B] [S] [P]
[G] [R] [M] [B] [F] [J] [S] [Z] [D]
[J] [L] [P] [F] [C] [H] [F] [J] [C]
[Z] [Q] [F] [L] [G] [W] [H] [F] [M]
 1   2   3   4   5   6   7   8   9
 */

fn main() {
    let lines = include_str!("input.txt").lines();
    let mut stacks = HashMap::new();
    stacks.insert(1, "ZJG".to_string());
    stacks.insert(2, "QLRPWFVC".to_string());
    stacks.insert(3, "FPMCLGR".to_string());
    stacks.insert(4, "LFBWPHM".to_string());
    stacks.insert(5, "GCFSVQ".to_string());
    stacks.insert(6, "WHJZMQTL".to_string());
    stacks.insert(7, "HFSBV".to_string());
    stacks.insert(8, "FJZS".to_string());
    stacks.insert(9, "MCDPFHBT".to_string());
    for ln in lines.into_iter() {
        match parse_move_params(ln) {
            Ok((_, (move_count, from_id, to_id))) => {
                let s = stacks.get(&from_id).unwrap().clone();
                let mut v = String::new();
                let frm_idx: usize = s.len() - move_count;
                let new_str = (s[0..frm_idx]).to_string();
                (s[frm_idx..s.len()]).clone_into(&mut v);
                let mut dst = stacks.get(&to_id).unwrap().clone();
                dst.push_str(&v);
                stacks.insert(from_id, new_str);
                stacks.insert(to_id, dst);
            }
            Err(_) => println!("Error parsing"),
        }
    }
    // WSFTMRHPP
    // GSLCMFBRP
    print!("{:?}", stacks);
}
