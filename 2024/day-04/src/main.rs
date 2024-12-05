use std::collections::HashMap;

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

fn extract_diagonals_top_right<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
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

fn extract_diagonals_top_left<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
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

fn extract_diagonals_top_right_2<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<(T, i32, i32)>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let max_diag_length = rows + cols - 1;

    let mut diagonals: Vec<Vec<(T, i32, i32)>> = (0..max_diag_length).map(|_| Vec::new()).collect();

    for r in 0..rows {
        for c in 0..cols {
            let v = (grid[r][c].clone(), r as i32, c as i32);
            diagonals[r + c].push(v);
        }
    }

    diagonals
}

fn extract_diagonals_top_left_2<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<(T, i32, i32)>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let max_diag_length = rows + cols - 1;

    let mut diagonals: Vec<Vec<(T, i32, i32)>> = (0..max_diag_length).map(|_| Vec::new()).collect();

    for r in 0..rows as i32 {
        for c in 0..cols as i32 {
            let idx: usize = (c - r + (rows as i32 - 1)).try_into().unwrap();
            let v = (grid[r as usize][c as usize].clone(), r, c);

            diagonals[idx].push(v);
        }
    }

    diagonals
}

fn part_1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let grid2 = transpose(grid.clone());
    let grid3 = extract_diagonals_top_right(&grid);
    let grid4 = extract_diagonals_top_left(&grid);

    let mut sum = 0;
    for g in [grid, grid2, grid3, grid4] {
        sum += count_words(g);
    }
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

fn count_words_part_2(grid: Vec<Vec<(char, i32, i32)>>) -> Vec<(i32, i32)> {
    let mas_str = "MAS";
    let rev_mas_str = "SAM";
    let mut results: Vec<(i32, i32)> = Vec::new();
    for row in grid {
        let s: String = row.iter().map(|x| x.0).collect();
        let coords_lookup: HashMap<usize, (i32, i32)> = row
            .iter()
            .enumerate()
            .map(|(idx, (_, x, y))| (idx, (*x, *y)))
            .collect();

        let mut center = 1;
        for i in s.as_bytes().windows(mas_str.len()) {
            if i == mas_str.as_bytes() || i == rev_mas_str.as_bytes() {
                results.push(*coords_lookup.get(&center).unwrap())
            }
            center += 1;
        }
    }
    results
}

fn part_2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let grid_d1 = extract_diagonals_top_right_2(&grid);
    let grid_d2 = extract_diagonals_top_left_2(&grid);

    // Find all MAS appearing a diagonal in grid_d1, grid_d2
    // If The 'A' location for each of grid_d1, grid_d2 are the same then it's an X-MAS
    // Problem: co-ordinate system used for both grids will be different.
    // Solution: Each generated grid needs to reain a reference to the original co-ordinate system
    let mas_matches_1 = count_words_part_2(grid_d1);
    println!("{:?}", mas_matches_1);
    let mas_matches_2 = count_words_part_2(grid_d2);
    println!("{:?}", mas_matches_2);

    mas_matches_1
        .iter()
        .filter(|&x| mas_matches_2.contains(x))
        .cloned()
        .count() as i32
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
    let test_input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
    assert_eq!(part_2(test_input), 9);
}

#[test]

fn test_part_2_simple() {
    let test_input = "S-M
-A-
S-M";
    assert_eq!(part_2(test_input), 1);
}
