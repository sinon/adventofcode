use core::fmt;
use miette::{bail, Result};
use rayon::prelude::*;
use std::{
    collections::{hash_set, HashMap, HashSet},
    hash::Hash,
};

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Location {
    row: isize,
    column: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Location { row: x, column: y }
    }
}

type Grid = [[char; 130]; 130];

struct Guard<'a> {
    current_location: Location,
    facing: Direction,
    off_grid: bool,
    grid: &'a Grid,
    visited: HashSet<Location>,
    bumps: HashMap<Location, i64>,
}

impl<'a> Guard<'a> {
    fn new(start: Location, facing: Direction, grid: &Grid) -> Guard {
        let mut visited: HashSet<Location> = HashSet::new();
        visited.insert(start.clone());
        Guard {
            current_location: start,
            facing,
            visited,
            grid,
            off_grid: false,
            bumps: HashMap::new(),
        }
    }

    fn next_location(&mut self) -> Option<Location> {
        match self.facing {
            Direction::North => {
                let next_x = self.current_location.row - 1;
                // println!("N: {}->{}", self.current_location.row, next_x);
                if next_x < 0 {
                    self.off_grid = true;
                    return None;
                }
                Some(Location {
                    row: next_x,
                    column: self.current_location.column,
                })
            }
            Direction::East => {
                let next_y = self.current_location.column + 1;
                // println!("E: {}{}", self.current_location.column, next_y);
                if next_y >= self.grid.len() as isize {
                    self.off_grid = true;
                    return None;
                }
                Some(Location {
                    row: self.current_location.row,
                    column: next_y,
                })
            }
            Direction::South => {
                let next_x = self.current_location.row + 1;
                // println!("S: {}->{}", self.current_location.row, next_x);
                if next_x >= self.grid.len() as isize {
                    self.off_grid = true;
                    return None;
                }
                Some(Location {
                    row: next_x,
                    column: self.current_location.column,
                })
            }
            Direction::West => {
                let next_y = self.current_location.column - 1;
                // println!("W: {}->{}", self.current_location.column, next_y);
                if next_y < 0 {
                    self.off_grid = true;
                    return None;
                }
                Some(Location {
                    row: self.current_location.row,
                    column: next_y,
                })
            }
        }
    }

    fn next_space(&mut self) -> Result<char> {
        let next_location = self.next_location();
        if self.off_grid || next_location.is_none() {
            bail!("Off grid no next space")
        }
        match next_location {
            None => {
                bail!("off grid")
            }
            Some(x) => Ok(self.grid[x.row as usize][x.column as usize]),
        }
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
                // self.visited.pop();
                break;
            }
            // println!("Wall ahead");
            // let hit_obstacle = self.next_location();
            self.bumps
                .entry(self.current_location.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
            self.turn();
            // println!("Turning");

            // println!("Now facing: {:?}", self.facing);
        }
        let next = self.next_location();
        match next {
            None => (),
            Some(n) => {
                // println!("Next location: {:?}", n);
                self.visited.insert(n.clone());
                self.current_location = n;
            }
        }
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
pub fn process(_input: &str) -> miette::Result<isize> {
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
            y += 1;
        }
        x += 1;
        y = 0;
    }
    let mut guard = Guard::new(start.clone(), Direction::North, &grid);

    loop {
        if guard.off_grid {
            println!("First guard went off grid");
            break;
        }
        guard.move_forward();
    }
    let original_visits = guard.visited.clone();
    let mut loops_found = HashSet::new();

    for (g_count, visit) in original_visits.into_iter().enumerate() {
        // println!("Insert Object at: {:?}", visit);
        let mut g = grid.clone();
        g[(visit.row) as usize][(visit.column) as usize] = '#';
        let mut new_g = Guard::new(start.clone(), Direction::North, &g);
        loop {
            if new_g.off_grid {
                println!("{} guard left the grid", g_count + 1);
                break;
            }
            new_g.move_forward();
            if new_g.bumps.values().any(|&x| x > 100) {
                loops_found.insert(visit.clone());
                println!("Loop found at {:?}", &visit);
                break;
            }
        }
    }
    // (6,3) (7,6) (7,7) (8,1) (8, 3) (9,7)
    dbg!(&loops_found);

    Ok(loops_found.len() as isize)
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
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
