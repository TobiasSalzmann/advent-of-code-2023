use crate::util::AdventHelper;
use array2d::Array2D;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid = advent.parse_grid_2d();

    advent.part1("Longest walk: {}", longest_walk(&grid, false));
    advent.part2("Longest walk: {}", longest_walk_2(&grid));
}

fn longest_walk(grid: &Array2D<char>, climb_slope: bool) -> usize {
    let start: (usize, usize) = (0, 1);
    let end = (grid.column_len() - 1, grid.row_len() - 2);

    dijkstra(
        &vec![start],
        |s| successors(s, grid, end, climb_slope),
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
    climb_slope: bool,
) -> Vec<(Vec<(usize, usize)>, usize)> {
    let current @ (y, x) = *visited.last().unwrap();
    let c = grid[current];
    let right = (y, x + 1);
    let left = (y, x - 1);
    let up = (y - 1, x);
    let down = (y + 1, x);

    let mut succ = vec![];
    if c == '.' || c == '>' || climb_slope {
        succ.push(right)
    }
    if c == '.' || c == '<' || climb_slope {
        succ.push(left)
    }
    if c == '.' || c == '^' || climb_slope {
        succ.push(up)
    }
    if c == '.' || c == 'v' || climb_slope {
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
                Some((x1, cost))
            } else {
                None
            }
        })
        .collect_vec();
    successors
}

fn direct_successors(node: &(usize, usize), grid: &Array2D<char>) -> Vec<(usize, usize)> {
    let (y, x) = node;
    let right = (*y, x + 1);
    let left = (*y, x - 1);
    let up = (y - 1, *x);
    let down = (y + 1, *x);

    let is_open = |(y, x)| grid.get(y, x).map(|c| *c != '#').unwrap_or(false);

    [up, down, left, right]
        .iter()
        .filter(|s| is_open(**s))
        .cloned()
        .collect_vec()
}

fn simplify(grid: &Array2D<char>) -> FxHashMap<(usize, usize), Vec<((usize, usize), usize)>> {
    let interesting_nodes: FxHashSet<(usize, usize)> = grid
        .enumerate_row_major()
        .filter(|(p, c)| **c != '#' && direct_successors(p, grid).len() != 2)
        .map(|(p, _)| p)
        .collect();
    let mut connections = FxHashMap::default();
    for node in &interesting_nodes {
        let mut conns = vec![];
        for mut n in direct_successors(&node, grid) {
            let mut visited = FxHashSet::default();
            visited.insert(*node);
            while !interesting_nodes.contains(&n) {
                visited.insert(n);
                let two_next = direct_successors(&n, grid);
                n = if visited.contains(&two_next[0]) {
                    two_next[1]
                } else {
                    two_next[0]
                }
            }
            conns.push((n, visited.len()));
        }
        connections.insert(*node, conns);
    }
    connections
}

fn longest_walk_2(grid: &Array2D<char>) -> usize {
    let start: (usize, usize) = (0, 1);
    let end = (grid.column_len() - 1, grid.row_len() - 2);
    let map = simplify(grid);

    let (path, _): (Vec<(Vec<(usize, usize)>, usize)>, usize) = dijkstra(
        &(vec![start], 0),
        |s| successors_2(s, &map, end),
        |x| *x.0.last().unwrap() == end,
    )
    .unwrap();
    path.last().unwrap().1
}

fn successors_2(
    visited: &(Vec<(usize, usize)>, usize),
    map: &FxHashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    end: (usize, usize),
) -> Vec<((Vec<(usize, usize)>, usize), usize)> {
    let (history, depth) = visited.clone();
    let current = history.last().unwrap();
    let mut succ = vec![];
    for (next, d) in &map[current] {
        if history.contains(&next) {
            continue;
        }
        let mut new_history = history.clone();
        new_history.push(*next);
        let new_depth = depth + d;
        let cost = if *next == end { 1000000 - new_depth } else { 0 };
        succ.push(((new_history, new_depth), cost))
    }
    succ
}
