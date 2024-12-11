#[tracing::instrument]
pub fn process(mut _input: &str) -> miette::Result<usize> {
    let input = _input.to_owned();
    let mut disk: Vec<Option<usize>> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10)) // Convert to digit.
        .map(|c| c as usize) // We'll use this as the size of the vec.
        .enumerate()
        .flat_map(|(i, capacity)| {
            if i % 2 == 0 {
                vec![Some(i / 2); capacity]
            } else {
                vec![None; capacity]
            }
        })
        .collect();

    let mut idx = 0;
    let mut rev_idx = disk.len() - 1;
    while idx < rev_idx {
        // Find the next free space.
        while idx < rev_idx && disk[idx].is_some() {
            idx += 1;
        }

        // Find the last filled space.
        while idx < rev_idx && disk[rev_idx].is_none() {
            rev_idx -= 1;
        }

        // Swap the two.
        disk.swap(idx, rev_idx);

        // Increment the pointers.
        idx += 1;
        rev_idx -= 1;
    }
    let r: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|x| x * i))
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
