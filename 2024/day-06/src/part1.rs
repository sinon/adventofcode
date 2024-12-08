use anyhow::{bail, Result};
use nom::combinator::rest;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Location {
    row: i32,
    column: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Location { row: x, column: y }
    }
}

type Grid = [[char; 130]; 130];

struct Guard<'a> {
    current_location: Location,
    facing: Direction,
    visited: HashSet<Location>,
    grid: &'a Grid,
    off_grid: bool,
}

impl<'a> Guard<'a> {
    fn new(start: Location, facing: Direction, grid: &Grid) -> Guard {
        let mut visited = HashSet::new();
        visited.insert(start.clone());
        Guard {
            current_location: start,
            facing,
            visited,
            grid,
            off_grid: false,
        }
    }

    fn next_location(&mut self) -> Location {
        match self.facing {
            Direction::North => {
                let next_x = self.current_location.row - 1;
                if next_x < 0 {
                    self.off_grid = true;
                }
                Location {
                    row: next_x,
                    column: self.current_location.column,
                }
            }
            Direction::East => {
                let next_y = self.current_location.column + 1;
                if next_y < 0 {
                    self.off_grid = true;
                }
                Location {
                    row: self.current_location.row,
                    column: next_y,
                }
            }
            Direction::South => {
                let next_x = self.current_location.row + 1;
                if next_x >= self.grid.len() as i32 {
                    self.off_grid = true;
                }
                Location {
                    row: next_x,
                    column: self.current_location.column,
                }
            }
            Direction::West => {
                let next_y = self.current_location.column - 1;
                if next_y >= self.grid.len() as i32 {
                    self.off_grid = true;
                }
                Location {
                    row: self.current_location.row,
                    column: next_y,
                }
            }
        }
    }

    fn next_space(&mut self) -> Result<char> {
        let next_location = self.next_location();
        if self.off_grid {
            bail!("Off grid no next space")
        }
        println!(
            "off grid: {} new_location: {:?}",
            self.off_grid, next_location
        );
        Ok(self.grid[next_location.row as usize][next_location.column as usize])
    }

    fn is_wall_ahead(&mut self) -> bool {
        let next_space = self.next_space();
        match next_space {
            Ok(c) => return !(c == '.'),
            Err(_) => false,
        }
    }

    fn move_forward(&mut self) -> () {
        loop {
            if !self.is_wall_ahead() || self.off_grid {
                break;
            }
            println!("Wall ahead");
            self.turn();
            println!("Turning");
            println!("Now facing: {:?}", self.facing);
            self.move_forward();
        }
        let next = self.next_location();
        println!("Next location: {:?}", next);
        self.visited.insert(next.clone());
        self.current_location = next;
    }

    fn turn(&mut self) {
        let new_direction = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self.facing = new_direction;
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut grid: Grid = [['.'; 130]; 130];
    let mut x = 0;
    let mut y = 0;
    let mut start = Location::new(0, 0);
    for ln in _input.lines() {
        for c in ln.chars() {
            if c == '^' {
                start = Location::new(x, y);
                grid[x as usize][y as usize] = '.';
            } else {
                grid[x as usize][y as usize] = c;
            }
            // println!("{} {}", x, y);
            y += 1;
        }
        x += 1;
        y = 0;
    }
    // for g in grid {
    //     println!("{:?}", g);
    // }
    // println!("{:?}", start);

    let mut guard = Guard::new(start, Direction::North, &grid);

    loop {
        if guard.off_grid {
            break;
        }
        guard.move_forward();
    }
    let result = guard.visited.len() - 1;
    Ok(result as i32)
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
