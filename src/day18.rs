use crate::util::Dir::{Down, Left, Right, Up};
use crate::util::{AdventHelper, Bounds, Dir, Point};
use itertools::Itertools;
use pathfinding::prelude::dfs_reach;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid: Vec<DigInstruction> = advent.parse_from_strings();

    advent.part1("Inside Area: {}", measure_inside_area(&grid))
}

fn measure_inside_area(instructions: &Vec<DigInstruction>) -> usize {
    let mut border = HashSet::new();
    let mut current = Point::new(0, 0);
    border.insert(current);
    for instruction in instructions {
        let instruction = instruction.real();
        println!("{instruction:?}");
        for _ in 0..instruction.length {
            current = current.mv(instruction.dir);
            border.insert(current);
        }
    }
    println!("{:?}", border.len());
    let sorted = border.iter().sorted().collect_vec();
    println!("{:?}", border.len());

    let bounds: Bounds = Point::bounds(&border).expand(1);
    let outside_point = Point::new(bounds.min_x, bounds.min_y);

    let outside: HashSet<Point> = dfs_reach(outside_point, |p| {
        p.neighbours()
            .into_iter()
            .filter(|n| !border.contains(n) && bounds.contains(n))
            .collect_vec()
    })
    .collect();
    println!("{outside:?}");

    bounds.to_set().difference(&outside).count()
}

#[derive(Debug)]
struct DigInstruction {
    dir: Dir,
    length: usize,
    color: String,
}

impl DigInstruction {
    fn real(&self) -> DigInstruction {
        let dir = match self.color.chars().last().unwrap() {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => unreachable!(),
        };
        let length = usize::from_str_radix(&self.color[..5], 16).unwrap();
        let color = "".to_string();
        DigInstruction { dir, length, color }
    }
}

impl FromStr for DigInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_dir, raw_length, raw_color) = s.split(" ").collect_tuple().unwrap();
        Ok(DigInstruction {
            dir: match raw_dir {
                "L" => Left,
                "D" => Down,
                "R" => Right,
                "U" => Up,
                _ => unreachable!(),
            },
            length: raw_length.parse().unwrap(),
            color: raw_color
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .to_string(),
        })
    }
}
