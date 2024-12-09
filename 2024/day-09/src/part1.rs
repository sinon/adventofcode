use std::fmt;

use itertools::Itertools;

#[derive(Debug)]
struct File {
    id: i32,
    block_size: i32,
    free_space: i32,
}

impl File {
    fn from_input(id: usize, block_size: char, free_space: char) -> Self {
        File {
            id: id as i32,
            block_size: block_size.to_digit(10).unwrap() as i32,
            free_space: free_space.to_digit(10).unwrap() as i32,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = (0..self.block_size)
            .map(|_| format!("{}", self.id))
            .collect::<String>();
        let y = (0..self.free_space)
            .map(|_| format!("{}", "."))
            .collect::<String>();
        write!(f, "{x}{y}")
    }
}

#[tracing::instrument]
pub fn process(mut _input: &str) -> miette::Result<u64> {
    let mut input = _input.to_owned();
    if !(input.len() % 2 == 0) {
        input += "0";
    }
    // println!("{}", input);
    let files: Vec<File> = input
        .chars()
        .tuples()
        .enumerate()
        .map(|(id, (block_size, free_space))| File::from_input(id, block_size, free_space))
        .collect();
    let to_compact: String = files.into_iter().map(|f| format!("{}", f)).collect();
    // println!("{}", to_compact);
    let mut to_compact: Vec<char> = to_compact.chars().collect();

    let mut rev_idx = to_compact.len() - 1;
    println!("size of memory to re-organize: {}", to_compact.len());
    for idx in 0..rev_idx {
        if to_compact[idx] == '.' {
            loop {
                if to_compact[rev_idx] == '.' {
                    rev_idx -= 1;
                }
                break;
            }
            to_compact.swap(idx, rev_idx);
            rev_idx -= 1;
        }
        let x = &to_compact[idx..to_compact.len()]
            .iter()
            .filter(|c| **c == '.')
            .count();
        // println!(
        //     "idx:{}..-1{}: {}  len_to_end:{}",
        //     idx,
        //     to_compact.len(),
        //     x,
        //     (to_compact.len() - idx)
        // );
        if *x == (to_compact.len() - idx - 1) {
            // println!("BALANCED");
            break;
        }
        // let compacted: String = to_compact.iter().map(|c| format!("{}", c)).collect();
        // println!("{}", compacted);
        println!("{}/{}", idx, to_compact.len());
    }
    let compacted: String = to_compact.iter().map(|c| format!("{}", c)).collect();
    println!("{}", compacted);
    // 00...111...2...333.44.5555.6666.777.888899
    // 00...111...2...333.44.5555.6666.777.888899
    // 0099811188827773336446555566..............
    let r = compacted
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            if c == '.' {
                return 0;
            }
            idx as u64 * c.to_digit(10).unwrap() as u64
        })
        .sum();
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!(1928, process(input)?);
        Ok(())
    }
}
