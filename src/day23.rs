use crate::util::{AdventHelper, BitSetGrid};
use array2d::Array2D;
use bit_set::BitSet;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::iter;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid = advent.parse_grid_2d();

    advent.part1("Longest walk: {}", longest_walk(&grid, false));
    advent.part2("Longest walk: {}", longest_walk(&grid, true));
}

fn direct_successors(
    node: &(usize, usize),
    grid: &Array2D<char>,
    ignore_slopes: bool,
) -> Vec<(usize, usize)> {
    let (y, x) = node;
    let right = (*y, x + 1);
    let left = (*y, x - 1);
    let up = (y - 1, *x);
    let down = (y + 1, *x);

    let is_open = |(y, x)| grid.get(y, x).map(|c| *c != '#').unwrap_or(false);

    let c = grid[(*y, *x)];
    let mut succ = vec![];
    if c == '.' || c == '>' || ignore_slopes {
        succ.push(right)
    }
    if c == '.' || c == '<' || ignore_slopes {
        succ.push(left)
    }
    if c == '.' || c == '^' || ignore_slopes {
        succ.push(up)
    }
    if c == '.' || c == 'v' || ignore_slopes {
        succ.push(down)
    }

    succ.iter().filter(|s| is_open(**s)).cloned().collect_vec()
}

fn simplify(
    grid: &Array2D<char>,
    ignore_slopes: bool,
) -> FxHashMap<(usize, usize), Vec<((usize, usize), usize)>> {
    let interesting_nodes: FxHashSet<(usize, usize)> = grid
        .enumerate_row_major()
        .filter(|(p, c)| {
            ((ignore_slopes && **c != '#') || (!ignore_slopes && **c == '.'))
                && direct_successors(p, grid, ignore_slopes).len() != 2
        })
        .map(|(p, _)| p)
        .collect();
    let mut connections = FxHashMap::default();
    for node in &interesting_nodes {
        let mut conns = vec![];
        'middle: for mut n in direct_successors(&node, grid, ignore_slopes) {
            let mut visited = FxHashSet::default();
            visited.insert(*node);
            while !interesting_nodes.contains(&n) {
                visited.insert(n);
                let two_next = direct_successors(&n, grid, ignore_slopes);
                n = if visited.contains(&two_next[0]) {
                    if two_next.len() < 2 {
                        continue 'middle;
                    }
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

fn longest_walk(grid: &Array2D<char>, ignore_slopes: bool) -> usize {
    let start: (usize, usize) = (0, 1);
    let end = (grid.column_len() - 1, grid.row_len() - 2);
    let map: FxHashMap<(usize, usize), Vec<((usize, usize), usize)>> =
        simplify(grid, ignore_slopes);

    let to_nodes: FxHashMap<(usize, usize), usize> =
        map.keys().enumerate().map(|(i, n)| (*n, i)).collect();

    let start = to_nodes[&start];
    let end = to_nodes[&end];
    let cap = to_nodes.len();
    let mut neighbours: Vec<Vec<usize>> = vec![vec![]; cap];
    let mut costs = Array2D::filled_with(0, cap, cap);
    for (a, bs) in map {
        for (b, cost) in bs {
            let a = to_nodes[&a];
            let b = to_nodes[&b];
            costs[(a, b)] = cost;
            neighbours[a].push(b);
        }
    }

    longest_path(start, end, 0, &neighbours, &costs).unwrap()
}

fn longest_path(
    start: usize,
    end: usize,
    visited: u64,
    neighbours: &Vec<Vec<usize>>,
    costs: &Array2D<usize>,
) -> Option<usize> {
    if start == end {
        return Some(0);
    }
    let mut longest = None;
    for next in &neighbours[start] {
        if (visited & (1 << next)) > 0 {
            continue;
        }
        let next_visited = visited | (1 << next);
        if let Some(length) = longest_path(*next, end, next_visited, neighbours, costs) {
            let new_length = costs[(start, *next)] + length;
            if longest == None || longest.unwrap() < new_length {
                longest = Some(new_length)
            }
        };
    }
    longest
}
