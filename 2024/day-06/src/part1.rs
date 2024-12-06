#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut grid = [['.'; 10]; 10];
    let mut x = 0;
    let mut y = 0;
    let mut start = (0, 0);
    for ln in _input.lines() {
        for c in ln.chars() {
            if c == '^' {
                start = (x, y);
            }
            println!("{} {}", x, y);
            grid[x][y] = c;
            y += 1;
        }
        x += 1;
        y = 0;
    }
    for g in grid {
        println!("{:?}", g);
    }
    println!("{:?}", start);

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, process(input)?);
        Ok(())
    }
}
