use crate::util::AdventHelper;
use array2d::Array2D;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid = advent.parse_grid_2d();

    advent.part1("Desintegratable blocks: {}", longest_walk(&grid));
}

fn longest_walk(grid: &Array2D<char>) -> usize {
    let start: (usize, usize) = (0, 1);
    let end = (grid.column_len() - 1, grid.row_len() - 2);
    println!("{start:?}");
    println!("{end:?}");

    dijkstra(
        &vec![start],
        |s| successors(s, grid, end),
        |x| *x.last().unwrap() == end,
    )
    .unwrap()
    .0
    .len()
        - 1
}

fn successors(
    visited: &Vec<(usize, usize)>,
    grid: &Array2D<char>,
    end: (usize, usize),
) -> Vec<(Vec<(usize, usize)>, usize)> {
    let current @ (y, x) = *visited.last().unwrap();
    let c = grid[current];
    let right = (y, x + 1);
    let left = (y, x - 1);
    let up = (y - 1, x);
    let down = (y + 1, x);

    let mut succ = vec![];
    if c == '.' || c == '>' {
        succ.push(right)
    }
    if c == '.' || c == '<' {
        succ.push(left)
    }
    if c == '.' || c == '^' {
        succ.push(up)
    }
    if c == '.' || c == 'v' {
        succ.push(down)
    }

    let is_open = |(y, x)| grid.get(y, x).map(|c| *c != '#').unwrap_or(false);

    let successors = succ
        .iter()
        .filter_map(|s| {
            if is_open(*s) && !visited.contains(s) {
                let cost = if *s == end {
                    1000000 - visited.len()
                } else {
                    0
                };
                let mut x1 = visited.clone();
                x1.push(*s);
                // println!("{current:?} -> {s:?}");
                Some((x1, cost))
            } else {
                None
            }
        })
        .collect_vec();
    // println!("{current:?} -> {successors:?}");
    successors
}
