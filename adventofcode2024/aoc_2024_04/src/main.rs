use std::process::id;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part_1(input);
    println!("Part 1 Answer: {}", part1);
    let input = include_str!("input.txt");
    let p2: i32 = part_2(input);
    println!("Part 2 Answer: {}", p2);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn extract_diagonals_top_right<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let max_diag_length = rows + cols - 1;

    let mut diagonals: Vec<Vec<T>> = (0..max_diag_length).map(|_| Vec::new()).collect();

    for r in 0..rows {
        for c in 0..cols {
            diagonals[r + c].push(grid[r][c].clone());
        }
    }

    diagonals
}

fn extract_diagonals_top_left<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let max_diag_length = rows + cols - 1;

    let mut diagonals: Vec<Vec<T>> = (0..max_diag_length).map(|_| Vec::new()).collect();

    for r in 0..rows as i32 {
        for c in 0..cols as i32 {
            let idx: usize = (c - r + (rows as i32 - 1)).try_into().unwrap();
            diagonals[idx].push(grid[r as usize][c as usize].clone());
        }
    }

    diagonals
}

fn part_1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let grid2 = transpose(grid.clone());
    let grid3 = extract_diagonals_top_right(&grid);
    let grid4 = extract_diagonals_top_left(&grid);
    println!("Grid1");
    for g in &grid {
        println!("{:?}", g)
    }
    println!("Grid2");
    for g in &grid2 {
        println!("{:?}", g)
    }
    println!("Grid3");
    for g in &grid3 {
        println!("{:?}", g)
    }
    println!("Grid4");
    for g in &grid4 {
        println!("{:?}", g)
    }

    let mut sum = 0;
    sum += count_words(grid); // -->, <---
    sum += count_words(grid2); // Up, Down
    sum += count_words(grid3); // Diagonal Top Left to Bottom Right
    sum += count_words(grid4); // Diagonal Top Left to Bottom Right
    sum as i32
}

fn count_words(grid: Vec<Vec<char>>) -> usize {
    let xmas_str = "XMAS";
    let rev_xmas_str = "SAMX";
    let mut sum = 0;
    for row in grid {
        let s: String = row.iter().collect();
        sum += s
            .as_bytes()
            .windows(xmas_str.len())
            .filter(|&w| w == xmas_str.as_bytes())
            .count();
        sum += s
            .as_bytes()
            .windows(rev_xmas_str.len())
            .filter(|&w| w == rev_xmas_str.as_bytes())
            .count();
    }
    sum
}

fn part_2(input: &str) -> i32 {
    0
}

#[test]
fn test_part_1() {
    let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part_1(test_input), 18);
}
#[test]
fn test_part_1_1() {
    let test_input = "XMAS
M---
A---
S---";
    assert_eq!(part_1(test_input), 2);
}

#[test]
fn test_part_1_2() {
    let test_input = "SAMX
A---
M---
X---";
    assert_eq!(part_1(test_input), 2);
}

#[test]
fn test_part_1_3() {
    let test_input = "X---
-M--
--A-
---S";
    assert_eq!(part_1(test_input), 1);
}

#[test]
fn test_part_1_4() {
    let test_input = "---X
-MM-
-AA-
S--S";
    assert_eq!(part_1(test_input), 1);
}

#[test]
fn test_transpose() {
    let t = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
    assert_eq!(transpose(t), expected);
}

#[test]
fn test_part_2() {
    let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part_2(test_input), 0);
}
