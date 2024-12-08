use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd)]
struct Location {
    x: i64,
    y: i64,
}

impl Sub for Location {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Location {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Location {
    fn calculate_antinodes(&self, other: &Location) -> Vec<Location> {
        let dist = if self > other {
            self.sub(*other)
        } else {
            other.sub(*self)
        };
        let mut results = Vec::new();
        for i in 0..100 {
            let x = self.sub(Location {
                x: dist.x * i,
                y: dist.y * i,
            });
            let y = other.add(Location {
                x: dist.x * i,
                y: dist.y * i,
            });
            results.push(x);
            results.push(y);
        }
        results
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();
    let mut grid_size: i64 = 0;
    for (x, row) in _input.lines().enumerate() {
        grid_size = row.len() as i64;
        for (y, c) in row.chars().enumerate() {
            if c != '.' {
                let loc = Location {
                    x: x as i64,
                    y: y as i64,
                };
                antennas
                    .entry(c)
                    .and_modify(|e| e.push(loc))
                    .or_insert_with(|| vec![loc]);
            }
        }
    }
    let mut antinodes: HashSet<Location> = HashSet::new();
    for (_, locations) in antennas {
        for ls in locations.into_iter().combinations(2) {
            let l1 = &ls[0];
            let l2 = &ls[1];
            antinodes.extend(l1.calculate_antinodes(l2));
        }
    }
    let result = antinodes
        .into_iter()
        .filter(|x| x.x < grid_size && x.y < grid_size && x.x >= 0 && x.y >= 0)
        .count();
    Ok(result as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(34, process(input)?);
        Ok(())
    }
}
