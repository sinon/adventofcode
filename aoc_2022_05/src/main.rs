use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::IResult;

struct Stack {
    contents: Vec<char>,
}

impl Stack {
    fn move_crates(&mut self, count: i32, mut dest: Stack) {
        for i in 0..count {
            dest.contents.push(self.contents.pop().unwrap());
        }
    }
}

fn parse_move_params(input: &str) -> IResult<&str, (i32, i32, i32)> {
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
    // let mut stack_1 = vec!['Z', 'J', 'G'];
    // let mut stack_2 = vec!['Q', 'L', 'R', 'P', 'W', 'F', 'V', 'C'];
    // let mut stack_3 = vec!['F', 'P', 'M', 'C', 'L', 'G', 'R'];
    // let mut stack_4 = vec!['L', 'F', 'B', 'W', 'P', 'H', 'M'];
    // let mut stack_5 = vec!['G', 'C', 'F', 'S', 'V', 'Q'];
    // let mut stack_6 = vec!['W', 'H', 'J', 'Z', 'M', 'Q', 'T', 'L'];
    // let mut stack_7 = vec!['H', 'F', 'S', 'B', 'V'];
    // let mut stack_8 = vec!['F', 'J', 'Z', 'S'];
    // let mut stack_9 = vec!['M', 'C', 'D', 'P', 'F', 'H', 'B', 'T'];
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
    // move 1 from 5 to 6
    for ln in lines.into_iter() {
        match parse_move_params(ln) {
            Ok((_, (move_count, from_id, to_id))) => {
                println!("{} {} {}", move_count, from_id, to_id);
                for i in 0..move_count {
                    let mut s = stacks.get(&from_id).unwrap().clone();
                    let v = s.pop().unwrap();
                    let mut dst = stacks.get(&to_id).unwrap().clone();
                    dst.push(v);
                    stacks.insert(from_id, s);
                    stacks.insert(to_id, dst);
                }
            }
            Err(_) => println!("Error parsing"),
        }
    }
    // WSFTMRHPP
    print!("{:?}", stacks);
}
