use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Range(usize, usize);

#[derive(Debug)]
enum Command {
    TurnOn(Range, Range),
    TurnOff(Range, Range),
    Toggle(Range, Range),
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, cmd_type) = alt((tag("turn off "), tag("turn on "), tag("toggle ")))(input)?;
    let (input, (x1, y1)) = separated_pair(digit1, char(','), digit1)(input)?;
    let (input, _x) = tag(" through ")(input)?;
    let (input, (x2, y2)) = separated_pair(digit1, char(','), digit1)(input)?;
    let cmd;
    match cmd_type {
        "turn off " => {
            cmd = Command::TurnOff(
                Range(x1.parse().unwrap(), y1.parse().unwrap()),
                Range(x2.parse().unwrap(), y2.parse().unwrap()),
            )
        }
        "turn on " => {
            cmd = Command::TurnOn(
                Range(x1.parse().unwrap(), y1.parse().unwrap()),
                Range(x2.parse().unwrap(), y2.parse().unwrap()),
            )
        }
        "toggle " => {
            cmd = Command::Toggle(
                Range(x1.parse().unwrap(), y1.parse().unwrap()),
                Range(x2.parse().unwrap(), y2.parse().unwrap()),
            )
        }
        _ => panic!("Unknown {}", cmd_type),
    }
    Ok((input, cmd))
}

fn part_1(input: &str) {
    let mut grid = [[false; 1000]; 1000];
    for ln in input.lines() {
        let (_, cmd) = parse_command(ln).unwrap();
        match cmd {
            Command::TurnOn(Range(x1, y1), Range(x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        grid[x][y] = true;
                    }
                }
            }
            Command::TurnOff(Range(x1, y1), Range(x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        grid[x][y] = false;
                    }
                }
            }
            Command::Toggle(Range(x1, y1), Range(x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        grid[x][y] = !grid[x][y];
                    }
                }
            }
        }
    }
    let mut lit_count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] {
                lit_count += 1;
            }
        }
    }
    dbg!(lit_count);
}

fn part_2(input: &str) {
    let mut grid = [[0; 1000]; 1000];
    for ln in input.lines() {
        let (_, cmd) = parse_command(ln).unwrap();
        match cmd {
            Command::TurnOn(Range(x1, y1), Range(x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        grid[x][y] += 1;
                    }
                }
            }
            Command::TurnOff(Range(x1, y1), Range(x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        grid[x][y] -= 1;
                        if grid[x][y] < 0 {
                            grid[x][y] = 0;
                        }
                    }
                }
            }
            Command::Toggle(Range(x1, y1), Range(x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        grid[x][y] += 2;
                    }
                }
            }
        }
    }
    let mut lit_count_2 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            lit_count_2 += grid[x][y];
        }
    }
    dbg!(lit_count_2);
}

fn main() {
    let input = include_str!("./input.txt");
    part_1(input);

    part_2(input);
}

#[test]
fn test_p1() {
    let x = parse_command("turn off 660,55 through 986,197").unwrap();
    assert!(false);
}
