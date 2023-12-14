use std::collections::{HashMap, HashSet};

use crate::day14::Rock::{Fixed, Round};
use crate::util::{AdventHelper, Point};
use itertools::Itertools;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines = advent.parse_from_strings();
    let grid: HashMap<Point, Rock> = grid(&lines);

    advent.part1("load:  {}", load(&grid, lines.len() as i32));
    advent.part2("load:  {}", load2(&grid, lines.len() as i32, 1_000_000_000));
}

fn load(grid: &HashMap<Point, Rock>, n: i32) -> i32 {
    let mut grid = grid.clone();

    step(&mut grid, Point::up);
    grid.iter()
        .filter(|(_, rock)| **rock == Round)
        .map(|(p, _)| n - p.y)
        .sum()
}

fn load2(grid: &HashMap<Point, Rock>, n: i32, target: i32) -> i32 {
    let mut grid = grid.clone();
    let mut seen: HashMap<String, i32> = HashMap::new();
    let mut loads: HashMap<i32, i32> = HashMap::new();

    for i in 1.. {
        step(&mut grid, Point::up);
        step(&mut grid, Point::left);
        step(&mut grid, Point::down);
        step(&mut grid, Point::right);

        let digest = grid
            .iter()
            .filter(|(_, rock)| **rock == Round)
            .map(|(k, _v)| k)
            .sorted()
            .join(",");
        let load = grid
            .iter()
            .filter(|(_, rock)| **rock == Round)
            .map(|(p, _)| n - p.y)
            .sum();
        if let Some(idx) = seen.get(&digest).cloned() {
            let cycle_length = i - idx;
            let distance_to_target = (target - idx) % cycle_length;
            return loads[&(idx + distance_to_target)];
        }
        seen.insert(digest, i);
        loads.insert(i, load);
    }
    unreachable!()
}

fn step(grid: &mut HashMap<Point, Rock>, mv: fn(&Point) -> Point) {
    let col: HashSet<Point> = grid.keys().cloned().collect();
    let bounds = Point::bounds(&col);
    let mut changed = 1;
    while changed > 0 {
        changed = 0;
        let round_rocks = grid
            .iter()
            .filter(|(_, rock)| **rock == Round)
            .map(|(p, _)| p)
            .cloned()
            .collect_vec();
        for p in round_rocks {
            let next = mv(&p);
            if !grid.contains_key(&next) && bounds.contains(&next) {
                grid.remove(&p);
                grid.insert(next, Round);
                changed += 1;
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Rock {
    Round,
    Fixed,
}

fn grid(lines: &Vec<String>) -> HashMap<Point, Rock> {
    let mut grid = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                grid.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    if char == 'O' { Round } else { Fixed },
                );
            }
        }
    }
    grid
}
