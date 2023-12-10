use crate::util::{AdventHelper, Point};
use itertools::Itertools;

use pathfinding::prelude::dfs_reach;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines: Vec<String> = advent.parse_from_strings();
    let grid = to_grid(&lines);

    advent.part1("farthest distance: {}", farthest(&grid));
    advent.part2("number of inside points: {}", count_inside_points(&grid));
}

fn count_inside_points(grid: &HashMap<Point, char>) -> usize {
    let zoomed_in_loop = zoomed_in_loop(grid);
    let bounds = Point::bounds(&zoomed_in_loop).expand(1);
    let outside_point = Point {
        x: bounds.min_x,
        y: bounds.min_y,
    };

    let outside: HashSet<Point> = dfs_reach(outside_point, |p| {
        p.neighbours()
            .into_iter()
            .filter(|n| !zoomed_in_loop.contains(n) && bounds.contains(n))
            .collect_vec()
    })
    .into_iter()
    .collect();

    let mut inside_count = 0;
    for y in bounds.min_y..=bounds.max_y {
        for x in bounds.min_x..=bounds.max_x {
            let is_outside = outside.contains(&Point { x, y });
            let is_loop = zoomed_in_loop.contains(&Point { x, y });
            let is_original_grid_point = x % 3 == 1 && y % 3 == 1;
            let is_inside = !is_outside && !is_loop && is_original_grid_point;
            if is_inside {
                inside_count += 1;
            }
        }
    }
    inside_count
}

fn zoomed_in_loop(grid: &HashMap<Point, char>) -> HashSet<Point> {
    let das_loop: HashSet<Point> = HashSet::from_iter(find_loop(grid));
    let mut zoomed_in = HashSet::new();
    for p in das_loop {
        zoomed_in.insert(Point {
            x: 3 * p.x + 1,
            y: 3 * p.y + 1,
        });
        let c = grid.get(&p).unwrap();
        if let '|' | 'L' | 'J' | 'S' = c {
            zoomed_in.insert(Point {
                x: 3 * p.x + 1,
                y: 3 * p.y,
            });
        }
        if let '|' | '7' | 'F' | 'S' = c {
            zoomed_in.insert(Point {
                x: 3 * p.x + 1,
                y: 3 * p.y + 2,
            });
        }
        if let '-' | '7' | 'J' | 'S' = c {
            zoomed_in.insert(Point {
                x: 3 * p.x,
                y: 3 * p.y + 1,
            });
        }
        if let '-' | 'F' | 'L' | 'S' = c {
            zoomed_in.insert(Point {
                x: 3 * p.x + 2,
                y: 3 * p.y + 1,
            });
        }
    }
    zoomed_in
}

fn farthest(grid: &HashMap<Point, char>) -> usize {
    find_loop(grid).len() / 2
}

fn find_loop(grid: &HashMap<Point, char>) -> Vec<Point> {
    let (start, _) = grid.iter().find(|(_, c)| **c == 'S').unwrap();
    let mut prev = start.clone();
    let mut current = Point {
        x: start.x,
        y: start.y + 1,
    };
    let mut points = vec![start.clone()];
    while current != *start {
        points.push(current);
        let (a, b) = match grid.get(&current).unwrap() {
            '|' => (
                Point {
                    x: current.x,
                    y: current.y - 1,
                },
                Point {
                    x: current.x,
                    y: current.y + 1,
                },
            ),
            '-' => (
                Point {
                    x: current.x - 1,
                    y: current.y,
                },
                Point {
                    x: current.x + 1,
                    y: current.y,
                },
            ),
            'L' => (
                Point {
                    x: current.x,
                    y: current.y - 1,
                },
                Point {
                    x: current.x + 1,
                    y: current.y,
                },
            ),
            'J' => (
                Point {
                    x: current.x,
                    y: current.y - 1,
                },
                Point {
                    x: current.x - 1,
                    y: current.y,
                },
            ),
            '7' => (
                Point {
                    x: current.x - 1,
                    y: current.y,
                },
                Point {
                    x: current.x,
                    y: current.y + 1,
                },
            ),
            'F' => (
                Point {
                    x: current.x + 1,
                    y: current.y,
                },
                Point {
                    x: current.x,
                    y: current.y + 1,
                },
            ),
            _ => panic!(),
        };
        let next = if a == prev { b } else { a };
        prev = current;
        current = next;
    }
    points
}

fn to_grid(lines: &Vec<String>) -> HashMap<Point, char> {
    let mut m = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            m.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                char,
            );
        }
    }
    m
}
