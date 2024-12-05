use nom::character::complete::{char, digit1};
use nom::sequence::separated_pair;

use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Range {
    pub x: i32,
    pub y: i32,
}

impl Into<Range> for (&str, &str) {
    fn into(self) -> Range {
        Range {
            x: self.0.parse::<i32>().unwrap(),
            y: self.1.parse::<i32>().unwrap(),
        }
    }
}
// Range { x: 58, y: 79 } - Range { x: 16, y: 59 }
impl Range {
    fn contains(&self, other: &Range) -> bool {
        if self.x >= other.x && self.y <= other.y {
            return true;
        } else if other.x >= self.x && other.y <= self.y {
            return true;
        }
        false
    }
    fn overlap(&self, other: &Range) -> bool {
        if self.x >= other.x && self.x <= other.y {
            return true;
        } else if self.y <= other.y && self.y >= other.x {
            return true;
        } else if other.x <= self.y && other.x >= self.x {
            return true;
        }
        false
    }
}

fn range(input: &str) -> IResult<&str, Range> {
    let (input, (x, y)) = separated_pair(digit1, char('-'), digit1)(input)?;

    Ok((input, (x, y).into()))
}

fn range_pair(input: &str) -> IResult<&str, (Range, Range)> {
    let (input, (r1, r2)) = separated_pair(range, char(','), range)(input)?;

    Ok((input, (r1, r2)))
}

fn main() {
    let lines = include_str!("./input.txt").lines();
    let mut contains_count = 0;
    let mut overlap_count = 0;
    for ln in lines.into_iter() {
        match range_pair(ln) {
            Ok((_, (r1, r2))) => {
                if r1.contains(&r2) {
                    contains_count += 1;
                }
                if r1.overlap(&r2) {
                    overlap_count += 1;
                }
            }
            Err(_) => println!("err"),
        }
    }
    println!("Contains Total: {}", contains_count);
    // FIX: Overlap value is too high
    println!("Overlap Total: {}", overlap_count);
}

#[test]
fn test() {
    let r1 = Range { x: 58, y: 79 };
    let r2 = Range { x: 16, y: 59 };
    assert_eq!(r1.contains(&r2), false);
}

#[test]
fn test_overlap() {
    let r1 = Range { x: 58, y: 79 };
    let r2 = Range { x: 16, y: 59 };
    assert_eq!(r1.overlap(&r2), true);
    let r1 = Range { x: 5, y: 7 };
    let r2 = Range { x: 7, y: 9 };
    assert_eq!(r1.overlap(&r2), true);

    let r1 = Range { x: 2, y: 8 };
    let r2 = Range { x: 3, y: 7 };
    assert_eq!(r1.overlap(&r2), true);

    let r1 = Range { x: 6, y: 6 };
    let r2 = Range { x: 4, y: 6 };
    assert_eq!(r1.overlap(&r2), true);

    let r1 = Range { x: 2, y: 6 };
    let r2 = Range { x: 4, y: 8 };
    assert_eq!(r1.overlap(&r2), true);
}
