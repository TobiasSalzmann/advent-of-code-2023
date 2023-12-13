use crate::util::{AdventHelper, Point};
use itertools::Itertools;
use std::cmp::max;

use num::abs;

use std::collections::HashSet;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines: Vec<String> = advent.parse_from_strings();
    let grid = to_grid(&lines);

    advent.part1("sum of distances: {}", sum_of_distances(&grid, 2));
    advent.part2("sum of distances: {}", sum_of_distances(&grid, 1_000_000));
}

fn sum_of_distances(galaxies: &HashSet<Point>, factor: i32) -> i64 {
    let galaxies = expand(galaxies, factor);
    let mut sum = 0;
    for a in &galaxies {
        for b in &galaxies {
            sum += (abs(a.x - b.x) + abs(a.y - b.y)) as i64;
        }
    }
    sum / 2
}

fn expand(galaxies: &HashSet<Point>, factor: i32) -> HashSet<Point> {
    let mut x_empty_space = 0;
    let mut x_expanded_galaxies = HashSet::new();
    let mut x_previous_galaxy = Point { x: 0, y: 0 };
    for galaxy in galaxies.iter().sorted_by_key(|g| g.x) {
        x_empty_space += max(0, (galaxy.x - x_previous_galaxy.x - 1) * (factor - 1));
        x_expanded_galaxies.insert(Point {
            x: galaxy.x + x_empty_space,
            y: galaxy.y,
        });
        x_previous_galaxy = galaxy.clone()
    }
    let mut y_empty_space = 0;
    let mut y_expanded_galaxies = HashSet::new();
    let mut y_previous_galaxy = Point { x: 0, y: 0 };
    for galaxy in x_expanded_galaxies.iter().sorted_by_key(|g| g.y) {
        y_empty_space += max(0, (galaxy.y - y_previous_galaxy.y - 1) * (factor - 1));
        y_expanded_galaxies.insert(Point {
            x: galaxy.x,
            y: galaxy.y + y_empty_space,
        });
        y_previous_galaxy = galaxy.clone()
    }
    y_expanded_galaxies
}

// fn sum_of_distances(galaxies: &HashSet<Point>) -> i32 {
//
// }

fn to_grid(lines: &Vec<String>) -> HashSet<Point> {
    let mut m = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                m.insert(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    m
}
