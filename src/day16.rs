use crate::util::Dir::{Down, Left, Right, Up};
use crate::util::{AdventHelper, Dir, Point};
use rayon::prelude::*;
use rustc_hash::FxHashSet;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid = advent.parse_grid();

    advent.part1(
        "number of energized:  {}",
        count_energized(&grid, 0, 0, Right),
    );
    advent.part2("number of energized:  {}", max_energized(&grid));
}

fn max_energized(grid: &Vec<Vec<char>>) -> usize {
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;

    let top_down = (0..=max_x)
        .into_par_iter()
        .map(|x| count_energized(grid, x, 0, Down));
    let bottom_up = (0..=max_x)
        .into_par_iter()
        .map(|x| count_energized(grid, x, max_y, Up));
    let left_right = (0..=max_y)
        .into_par_iter()
        .map(|y| count_energized(grid, 0, y, Right));
    let right_left = (0..=max_y)
        .into_par_iter()
        .map(|y| count_energized(grid, max_x, y, Left));
    top_down
        .chain(bottom_up)
        .chain(left_right)
        .chain(right_left)
        .max()
        .unwrap()
}

fn count_energized(grid: &Vec<Vec<char>>, x: usize, y: usize, dir: Dir) -> usize {
    let capacity = grid.len() * grid[0].len();
    let initial = (Point::new(x, y), dir);
    let mut visited = FxHashSet::with_capacity_and_hasher(capacity, Default::default());
    let mut visited_point = FxHashSet::with_capacity_and_hasher(capacity, Default::default());
    let mut front = vec![initial];
    while let Some(current @ (point @ Point { x, y }, dir)) = front.pop() {
        if out_of_bounds(grid, x, y) || visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        visited_point.insert(point);
        for new_dir in reflect(grid, x, y, dir) {
            front.push((point.mv(new_dir), new_dir))
        }
    }

    visited_point.len()
}

fn out_of_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len()
}

fn reflect(grid: &[Vec<char>], x: i32, y: i32, dir: Dir) -> Vec<Dir> {
    match (dir, grid[y as usize][x as usize]) {
        (Up, '/') | (Down, '\\') => vec![Right],
        (Down, '/') | (Up, '\\') => vec![Left],
        (Right, '/') | (Left, '\\') => vec![Up],
        (Left, '/') | (Right, '\\') => vec![Down],
        (Up, '-') | (Down, '-') => vec![Left, Right],
        (Left, '|') | (Right, '|') => vec![Up, Down],
        (d, _) => vec![d],
    }
}
