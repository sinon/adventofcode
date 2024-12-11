#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    File,
    Free,
}

#[derive(Debug)]
struct Block {
    type_: BlockType,
    data: Vec<usize>,
    free: usize,
}

impl Block {
    fn new(id: usize, size: usize) -> Self {
        if id % 2 == 0 {
            Block {
                type_: BlockType::File,
                free: 0,
                data: vec![id / 2; size],
            }
        } else {
            Block {
                type_: BlockType::Free,
                free: size,
                data: Vec::new(),
            }
        }
    }

    fn fill(&mut self, data: &Vec<usize>) {
        self.data.extend(data);
        self.free -= data.len();
        if self.free == 0 {
            self.type_ = BlockType::File;
        }
    }

    fn clear(&mut self) {
        self.free = self.data.len();
        self.data.clear();
        self.type_ = BlockType::Free;
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut disk: Vec<Block> = input
        .chars()
        .filter(|&c| c.is_digit(10)) // No whitespace.
        .filter_map(|c| c.to_digit(10)) // Convert to digit.
        .map(|c| c as usize) // We'll use this as the size of the vec.
        .enumerate()
        .map(|(i, size)| Block::new(i, size))
        .collect();
    let mut right = disk.len() - 1;
    while right > 0 {
        // We can ignore free space.
        if disk[right].type_ == BlockType::Free {
            right -= 1;
            continue;
        }

        // Find the next free space.
        let mut left = 0;
        while left < right {
            // If it's filled or there isn't enough space, move to the next.
            if disk[left].type_ == BlockType::File
                || (disk[left].type_ == BlockType::Free && disk[left].free < disk[right].data.len())
            {
                left += 1;
                continue;
            }

            // Otherwise, we found a space, so we can move the file.
            let r = disk[right].data.clone();
            disk[left].fill(&r);
            disk[right].clear();
            break;
        }
        right -= 1;
    }

    let p2 = disk
        .iter()
        .flat_map(|d| {
            // Include all the data first and then pad with 0s for all the free space.
            d.data
                .iter()
                .cloned()
                .chain(std::iter::repeat(0).take(d.free))
        })
        .enumerate()
        .map(|(i, x)| x * i)
        .sum::<usize>();
    Ok(p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!(2858, process(input)?);
        Ok(())
    }
}
