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

fn slant_grid<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone + std::fmt::Debug,
{
    assert!(!v.is_empty());
    // 1 2 3
    // 4 5 6
    // 7 8 9
    // to
    // 1 5 9
    // 2 6
    // 3
    // 4 8
    // 7

    // (0, 0), (1, 1), (2, 2)
    // (0, 1), (1, 2)
    // (1, 0), (2, 1)
    // (0, 3)
    // (3, 0)
    let max_diag_length = v.len() + v[0].len() - 1;
    let mut slanted: Vec<Vec<T>> = Vec::new();
    for i in 0..max_diag_length {
        slanted.push(Vec::new());
    }

    for r in 0..v.len() {
        for c in 0..v[0].len() {
            let i = v[r][c].clone();
            slanted[r + c].push(v[r][c].clone());
        }
    }

    slanted
}

fn part_1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let grid2 = transpose(grid.clone());
    let grid3 = slant_grid(grid.clone());
    println!("{:?}", grid);
    println!("{:?}", grid2);

    let mut sum = 0;
    sum += count_words(grid);
    sum += count_words(grid2);
    sum += count_words(grid3);

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
