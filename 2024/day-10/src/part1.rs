use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct SearchPath<N>(N, Option<Rc<SearchPath<N>>>);

impl<N: Debug> Debug for SearchPath<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.1 {
            Some(v) => write!(f, "{:?} -> {:?}", v, &self.0),
            None => write!(f, "{:?}", &self.0),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    value: u8,
}

impl Point {
    fn get_neighbours(&self, grid: &Vec<Vec<Point>>) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();

        // UP
        if self.x > 0 {
            neighbours.push(grid[self.x - 1][self.y].clone());
        }
        // LEFT
        if self.y > 0 {
            neighbours.push(grid[self.x][self.y - 1].clone());
        }
        // RIGHT
        if self.y < grid[0].len() - 1 {
            neighbours.push(grid[self.x][self.y + 1].clone());
        }
        // DOWN
        if self.x < grid.len() - 1 {
            neighbours.push(grid[self.x + 1][self.y].clone())
        }

        neighbours
    }
}

// pub fn dfs(root: TreeNodeRef, target: i32) -> Option<TreeNodeRef> {
//     let mut queue = VecDeque::new();
//     queue.push_back(root);

//     while let Some(node) = queue.pop_front() {
//         // Check if this is the node we are looking for
//         if node.borrow().val == target {
//             return Some(node);
//         }

//         // Add left and write to the back of the queue for DFS
//         let items = node.borrow();
//         if let Some(left) = &items.left {
//             queue.push_front(left.clone());
//         }

//         if let Some(right) = &items.right {
//             queue.push_front(right.clone());
//         }
//     }

//     // Search completed and node was not found
//     None
// }

fn bfs<N, F, R>(start: N, end: N, expand: F) -> Option<SearchPath<N>>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> R,
    R: IntoIterator<Item = N>,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(SearchPath(start, None));
    visited.insert(start);

    while let Some(SearchPath(node, path)) = queue.pop_front() {
        if node == end {
            return Some(SearchPath(node, path));
        }

        let path = Rc::new(SearchPath(node, path.clone()));

        for edge in expand(node) {
            if !visited.contains(&edge) {
                visited.insert(edge);
                queue.push_back(SearchPath(edge, Some(path.clone())));
            }
        }
    }

    None
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut associated_list: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut grid: Vec<Vec<Point>> = Vec::new();
    let mut trailheads = Vec::new();
    for (x, ln) in _input.lines().enumerate() {
        let mut row = Vec::new();
        for (y, c) in ln.chars().enumerate() {
            let value = c.to_digit(10).unwrap() as u8;
            let p = Point { x, y, value };
            // let ni = graph.add_node(p.clone());
            // associated_list.insert(p.clone(), p.get_neighbours(&grid));
            row.push(p.clone());
            if p.value == 0 {
                trailheads.push(p.clone());
            }
        }
        grid.push(row);
    }
    // println!("{:?}", grid);
    // println!("{:?}", associated_list);
    let mut path_count = 0;
    for t in trailheads {
        let path = bfs((t.x, t.y), (0, 9), |(x, y)| {
            let mut edges = Vec::new();
            for (dy, dx) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let edge_y = (y as isize + dy) as usize;
                let edge_x = (x as isize + dx) as usize;

                // Check if offset is within bounds
                if edge_y >= grid.len() || edge_x >= grid[edge_y].len() {
                    continue;
                }

                // Check if offset points to valid location
                if grid[edge_y][edge_x].value == 3 {
                    continue;
                }

                edges.push((edge_y, edge_x));
            }

            edges
        });
        match path {
            Some(x) => {
                path_count += 1;
            }
            None => {}
        }
    }
    let path = bfs((0, 0), (0, 9), |(x, y)| {
        let mut edges = Vec::new();
        for (dy, dx) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let edge_y = (y as isize + dy) as usize;
            let edge_x = (x as isize + dx) as usize;

            // Check if offset is within bounds
            if edge_y >= grid.len() || edge_x >= grid[edge_y].len() {
                continue;
            }

            // Check if offset points to valid location
            if grid[edge_y][edge_x].value == 3 {
                continue;
            }

            edges.push((edge_y, edge_x));
        }

        edges
    });
    println!("{:?}", path);
    // for (x, row) in grid.iter().enumerate() {
    //     for (y, (p, n1)) in row.iter().enumerate() {
    //         let neighbours = p.get_neighbours(&grid);
    //         // let n = graph.node_weight(*p);
    //         for n2 in neighbours {
    //             graph.add_edge(*n1, n2, ());
    //         }
    //     }
    // }
    // println!("{:?}", graph);
    // todo!("Part 1 process todo");
    Ok(path_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(36, process(input)?);
        Ok(())
    }
}
