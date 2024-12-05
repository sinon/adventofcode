use std::str::Lines;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::IResult;

fn parse_command(input: &str) -> IResult<&str, usize> {
    println!("{}", input);
    let mut cwd: Vec<&str> = Vec::new();
    let (input, command_prefix) = tag("$ ")(input)?;
    println!("{}", 1);
    let is_command = command_prefix.len() > 0;
    println!("{}", 2);
    let (mut is_ls, mut is_dir) = (false, false);
    println!("{}", 3);
    if is_command {
        let (input, ls_cmd) = tag("ls ")(input)?;
        println!("{}", 4);
        is_ls = ls_cmd.len() > 0;
        let (input, dir_cmd) = tag("dir ")(input)?;
        println!("{}", 5);
        is_dir = dir_cmd.len() > 0;
        if is_dir {
            println!("{}", 6);
            if input == ".." {
                println!("{}", 7);
                cwd.pop();
            } else {
                println!("{}", 8);
                cwd.push(input);
            }
        }
    }
    println!(
        "cwd:{:?} is_command:{:?} is_ls:{:?} input:{:?}",
        cwd, is_command, is_ls, input
    );

    Ok((input, (0)))
}

fn main() {
    let lines = include_str!("input.txt").lines();
    process_command_list(lines);
}

fn process_command_list(input: Lines) -> usize {
    for ln in input.into_iter() {
        let Ok((rem, x)) = parse_command(ln) else {
            unreachable!()
        };
        println!("{:?}", rem);
    }
    0
}

#[test]
fn test_sample_inputs() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        .lines();

    let expected_total = 95437;
    assert_eq!(expected_total, process_command_list(input));
}
