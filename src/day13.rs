use crate::util::AdventHelper;
use itertools::Itertools;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grids: Vec<Vec<Vec<char>>> = grids(&advent.parse_from_strings());

    advent.part1("summarize grids: {}", summarize_grids(&grids, 0));
    advent.part2("summarize grids: {}", summarize_grids(&grids, 1));
}

fn summarize_grids(grids: &Vec<Vec<Vec<char>>>, expected_smudges: i32) -> usize {
    grids
        .iter()
        .map(|grid| summarize_grid(grid, expected_smudges))
        .sum()
}

fn summarize_grid(grid: &Vec<Vec<char>>, expected_smudges: i32) -> usize {
    for i in 1..grid.len() {
        if horizontal(grid, i as i32) == expected_smudges {
            return 100 * i;
        }
    }
    for i in 1..grid[0].len() {
        if vertical(grid, i as i32) == expected_smudges {
            return i;
        }
    }
    0
}

fn horizontal(grid: &Vec<Vec<char>>, axis: i32) -> i32 {
    let mut mismatches = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let y2 = axis + axis - (y as i32) - 1;
            if y2 >= 0 && y2 < (grid.len() as i32) && grid[y][x] != grid[y2 as usize][x] {
                mismatches += 1
            }
        }
    }
    mismatches / 2
}

fn vertical(grid: &Vec<Vec<char>>, axis: i32) -> i32 {
    let mut mismatches = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let x2 = axis + axis - (x as i32) - 1;
            if x2 >= 0 && x2 < (grid[y].len() as i32) && grid[y][x] != grid[y][x2 as usize] {
                mismatches += 1
            }
        }
    }
    mismatches / 2
}

fn grids(lines: &Vec<String>) -> Vec<Vec<Vec<char>>> {
    lines
        .iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec()
        .split(|line| line.is_empty())
        .map(|x| x.to_vec())
        .collect_vec()
}
