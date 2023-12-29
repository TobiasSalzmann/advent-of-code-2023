use crate::day14::Rock::{Fixed, Round};
use crate::util::{AdventHelper, BitSetGrid, Point};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines = advent.parse_from_strings();
    let grid: FxHashMap<Point, Rock> = grid(&lines);

    advent.part1("load:  {}", load(&grid, lines.len() as i32));
    advent.part2("load:  {}", load2(&grid, lines.len() as i32, 1_000_000_000));
}

fn load(grid: &FxHashMap<Point, Rock>, n: i32) -> i32 {
    let mut grid = grid.clone();

    let fixed_rocks: FxHashSet<Point> = grid
        .iter()
        .filter(|(_, rock)| **rock == Fixed)
        .map(|(p, _)| p)
        .cloned()
        .collect();

    let fixed_rocks = BitSetGrid::from_hashset(&fixed_rocks);
    let mut round_rocks: FxHashSet<Point> = grid
        .iter()
        .filter(|(_, rock)| **rock == Round)
        .map(|(p, _)| p)
        .cloned()
        .collect();

    let mut round_rocks = BitSetGrid::from_hashset(&round_rocks);

    step(&mut round_rocks, Point::up, true, &fixed_rocks);
    round_rocks.into_iter().map(|p| n - p.y).sum()
}

fn load2(grid: &FxHashMap<Point, Rock>, n: i32, target: i32) -> i32 {
    let mut grid = grid.clone();
    let mut seen: FxHashMap<String, i32> = FxHashMap::default();
    let mut loads: FxHashMap<i32, i32> = FxHashMap::default();
    let fixed_rocks: FxHashSet<Point> = grid
        .iter()
        .filter(|(_, rock)| **rock == Fixed)
        .map(|(p, _)| p)
        .cloned()
        .collect();
    let fixed_rocks = BitSetGrid::from_hashset(&fixed_rocks);
    let round_rocks: FxHashSet<Point> = grid
        .iter()
        .filter(|(_, rock)| **rock == Round)
        .map(|(p, _)| p)
        .cloned()
        .collect();

    let mut round_rocks = BitSetGrid::from_hashset(&round_rocks);

    for i in 1.. {
        step(&mut round_rocks, Point::up, true, &fixed_rocks);
        step(&mut round_rocks, Point::left, false, &fixed_rocks);
        step(&mut round_rocks, Point::down, true, &fixed_rocks);
        step(&mut round_rocks, Point::right, false, &fixed_rocks);

        let digest = round_rocks.into_iter().sorted().join(",");
        let load = round_rocks.into_iter().map(|p| n - p.y).sum();
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

fn step(
    mut round_rocks: &mut BitSetGrid,
    mv: fn(&Point) -> Point,
    vertical: bool,
    fixed_rocks: &BitSetGrid,
) {
    let bounds = fixed_rocks.bounds();
    let mut changed = 1;
    while changed > 0 {
        changed = 0;
        let map = round_rocks.into_iter().collect_vec();
        for p in map {
            let mut good = p;
            loop {
                let next = mv(&good);
                if fixed_rocks.contains(&next)
                    || round_rocks.contains(&next)
                    || !bounds.contains(&next)
                {
                    break;
                }
                good = next;
            }
            if good != p {
                round_rocks.remove(&p);
                round_rocks.insert(&good);
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

fn grid(lines: &Vec<String>) -> FxHashMap<Point, Rock> {
    let capacity = lines.len() * lines[0].len();
    let mut grid = FxHashMap::with_capacity_and_hasher(capacity, Default::default());

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
