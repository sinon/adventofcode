use std::{fmt::Display, usize};

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Stone {
    value: usize,
}

#[derive(Debug)]
struct Line {
    stones: Vec<Stone>,
}

impl Line {
    fn blink(&mut self) {
        let mut idx: usize = 0;
        let mut to_insert: Vec<(usize, Stone)> = Vec::new();
        for s in self.stones.iter_mut() {
            if s.value == 0 {
                s.value = 1;
                idx += 1;
                continue;
            }
            let digit_str = format!("{}", s.value);
            if digit_str.len() % 2 == 0 {
                // println!("{}", digit_str);
                let mut mid = 1;
                if digit_str.len() != 2 {
                    mid = digit_str.len() / 1 - 1;
                }
                let (v1, v2) = digit_str.split_at(mid);
                // println!("{} {}", v1, v2);
                let v1 = v1.parse::<usize>().unwrap();
                let v2 = v2.parse::<usize>().unwrap();
                s.value = v1;
                idx += 1;
                to_insert.push((idx, Stone { value: v2.clone() }));
                continue;
            }
            s.value *= 2024;
        }
        for (idx, stone) in to_insert {
            self.stones.insert(idx, stone);
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.stones.iter().map(|s| s.value).join(" ");
        write!(f, "{}", s)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let stones: Vec<Stone> = input
        .trim()
        .split_ascii_whitespace()
        .map(|v| Stone {
            value: v.parse::<usize>().unwrap(),
        })
        .collect();
    let mut line = Line { stones };
    println!("{:}", line);
    //125 17

    //253000 1 7
    line.blink();
    println!("{:}", line);
    //253 0 2024 14168
    line.blink();
    println!("{:}", line);

    //512072 1 20 24 28676032
    line.blink();
    println!("{:}", line);

    //512 72 2024 2 0 2 4 2867 6032
    line.blink();
    println!("{:}", line);

    // 1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
    line.blink();
    println!("{}", line);

    // 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
    line.blink();
    println!("{:?}", line);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!(55312, process(input)?);
        Ok(())
    }
}
