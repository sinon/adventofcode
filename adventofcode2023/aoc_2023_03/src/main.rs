use std::str::Lines;

fn main() {
    let lines = include_str!("input.txt").lines();
    let answer1 = part_1(lines);
    println!("Part 1: {}", answer1);
}

#[derive(Debug)]
struct Point {
    row: usize,
    column: usize,
}
#[derive(Debug)]
struct NumberRange {
    start: Point,
    end: Point,
    value: usize,
}

impl NumberRange {
    fn has_nearby_symbol(&self, grid: &[Vec<char>]) -> bool {
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...
        let col_boundary = grid[0].len() - 1;
        let top_l = Point {
            row: self.start.row.saturating_sub(1),
            column: self.start.column.saturating_sub(1),
        };
        let bottom_l = Point {
            row: (self.start.row + 1).clamp(0, grid[1].len() - 1),
            column: self.start.column.saturating_sub(1),
        };
        let top_r = Point {
            row: self.start.row.saturating_sub(1),
            column: self.end.column + 1,
        };
        let bottom_r = Point {
            row: (self.start.row + 1).clamp(0, grid[1].len() - 1),
            column: self.end.column + 1,
        };

        // Above Horizontal
        for col_idx in top_l.column..(top_r.column + 1).clamp(0, col_boundary) {
            if is_symbol(grid[top_r.row][col_idx]) {
                return true;
            }
        }
        // Below Horizontal
        for col_idx in bottom_l.column..(bottom_r.column + 1).clamp(0, col_boundary) {
            if is_symbol(grid[bottom_l.row][col_idx]) {
                return true;
            }
        }

        // Left Vertical
        if is_symbol(grid[self.start.row][self.start.column.saturating_sub(1)]) {
            return true;
        }
        // Right Vertical
        let end_c = (self.end.column + 1).clamp(0, col_boundary);
        if is_symbol(grid[self.end.row][end_c]) {
            return true;
        }
        false
    }
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

#[test]
fn test_has_nearby_symbol() {
    let lines = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .lines();
    let grid = grid_from_lines(lines);
    let number_ranges = get_number_ranges_from_grid(&grid);
    // let num_range = &number_ranges[0];
    // assert_eq!(num_range.has_nearby_symbol(&grid), true);
    let num_range = &number_ranges[1];
    assert_eq!(num_range.has_nearby_symbol(&grid), false);
}

fn get_number_ranges_from_grid(grid: &[Vec<char>]) -> Vec<NumberRange> {
    let mut number_ranges: Vec<NumberRange> = Vec::new();
    let mut in_num = false;
    let mut num_start = 0;
    let mut num = "".to_string();
    // Iterate over the rows
    for (row_idx, row) in grid.iter().enumerate() {
        // Iterate over the columns of the row
        for (col_idx, col) in row.iter().enumerate() {
            // Identify number, isdigit till not is digit, capture range [(x,y), (x,y)]
            if col.is_numeric() {
                if !in_num {
                    in_num = true;
                    num_start = col_idx;
                    num.push(*col);
                } else {
                    num.push(*col);
                }

                if col_idx == row.len() - 1 {
                    in_num = false;
                    let num_end = row.len() - 1;
                    assert!((num_end - num_start) == num.len() - 1);
                    number_ranges.push(NumberRange {
                        start: Point {
                            row: row_idx,
                            column: num_start,
                        },
                        end: Point {
                            row: row_idx,
                            column: num_end,
                        },
                        value: num.parse().unwrap(),
                    });
                    num = "".to_string();
                }
            } else if in_num {
                in_num = false;
                let num_end = col_idx - 1;

                assert!((num_end - num_start) == num.len() - 1);
                number_ranges.push(NumberRange {
                    start: Point {
                        row: row_idx,
                        column: num_start,
                    },
                    end: Point {
                        row: row_idx,
                        column: num_end,
                    },
                    value: num.parse().unwrap(),
                });
                num = "".to_string();
            }
        }
    }
    number_ranges
}

fn part_1(lines: Lines) -> usize {
    // Load lines in to a 2D Array i.e x,y row/column
    let grid = grid_from_lines(lines);
    let number_ranges = get_number_ranges_from_grid(&grid);

    // Search the space around the range for a symbol
    let mut total = 0;
    for num_range in number_ranges {
        if num_range.has_nearby_symbol(&grid) {
            total += num_range.value;
        }
    }
    total
}

fn grid_from_lines(lines: Lines) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (_, ln) in lines.enumerate() {
        grid.push(ln.chars().collect());
    }
    grid
}

#[test]
fn test_schematic() {
    let lines = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .lines();
    assert_eq!(part_1(lines), 4361);
}
