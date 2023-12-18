use crate::util::Dir::{Down, Left, Right, Up};
use crate::util::{AdventHelper, Bounds, Dir, Point};
use itertools::Itertools;
use pathfinding::prelude::dfs_reach;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::collections::HashSet;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid: Vec<DigInstruction> = advent.parse_from_strings();

    advent.part1("Inside Area: {}", measure_inside_area(&grid));
    let real_instructions = grid.iter().map(|x| x.real()).collect_vec();
    advent.part1("Inside Area: {}", measure_inside_area(&real_instructions))
}

fn measure_inside_area(instructions: &Vec<DigInstruction>) -> u64 {
    let corners = corner_points(instructions);

    let x_converter = converters(&corners, |p| p.x);
    let y_converter = converters(&corners, |p| p.y);

    let border: FxHashSet<Point> = find_compressed_border(instructions, &x_converter, &y_converter);

    let bounds = Point::bounds(&border);

    let outside = flood_fill_outside(&border, &bounds);

    inside_area(x_converter, y_converter, bounds, outside)
}

fn corner_points(instructions: &Vec<DigInstruction>) -> Vec<Point> {
    let mut current = Point::new(0, 0);
    let mut corners = vec![current];
    for instruction in instructions {
        current = current.mv_mulitple(instruction.dir, instruction.length);
        corners.push(current);
    }
    corners
}

fn inside_area(
    x_converter: Converter,
    y_converter: Converter,
    bounds: Bounds,
    outside: HashSet<Point>,
) -> u64 {
    let mut sum = 0;
    for p @ Point { x, y } in bounds {
        let x_size = x_converter.compression_factor(&x);
        let y_size = y_converter.compression_factor(&y);

        if !outside.contains(&p) {
            sum += (x_size as u64) * (y_size as u64);
        }
    }
    sum
}

fn find_compressed_border(
    instructions: &Vec<DigInstruction>,
    x_converter: &Converter,
    y_converter: &Converter,
) -> HashSet<Point, BuildHasherDefault<FxHasher>> {
    let mut current = Point::new(0, 0);
    let mut border = FxHashSet::default();
    for instruction in instructions {
        let mut current_mapped = Point {
            x: x_converter.compress(&current.x),
            y: y_converter.compress(&current.y),
        };
        current = current.mv_mulitple(instruction.dir, instruction.length);
        let next_mapped = Point {
            x: x_converter.compress(&current.x),
            y: y_converter.compress(&current.y),
        };
        while current_mapped != next_mapped {
            border.insert(current_mapped);
            current_mapped = current_mapped.mv(instruction.dir)
        }
        border.insert(next_mapped);
    }
    border
}

fn flood_fill_outside<H: BuildHasher>(
    border: &HashSet<Point, H>,
    bounds1: &Bounds,
) -> HashSet<Point> {
    let bounds: Bounds = bounds1.expand(1);
    let outside_point = Point::new(bounds.min_x, bounds.min_y);

    let outside: HashSet<Point> = dfs_reach(outside_point, |p| {
        p.neighbours()
            .into_iter()
            .filter(|n| !border.contains(n) && bounds.contains(n))
            .collect_vec()
    })
    .collect();
    outside
}

fn converters(corners: &Vec<Point>, f: fn(&Point) -> i32) -> Converter {
    let mut compress_map = FxHashMap::default();
    let mut decompress_map = FxHashMap::default();

    for (ix, x) in corners.iter().map(f).sorted().unique().enumerate() {
        compress_map.insert(x, 2 * ix as i32);
        decompress_map.insert(2 * ix as i32, x);
    }
    Converter {
        compress_map,
        decompress_map,
    }
}

struct Converter {
    compress_map: FxHashMap<i32, i32>,
    decompress_map: FxHashMap<i32, i32>,
}

impl Converter {
    fn compress(&self, value: &i32) -> i32 {
        self.compress_map[value]
    }

    fn decompress(&self, value: &i32) -> i32 {
        self.decompress_map[value]
    }

    fn compression_factor(&self, &value: &i32) -> i32 {
        if value % 2 == 0 {
            return 1;
        }
        self.decompress(&(value + 1)) - self.decompress(&(value - 1)) - 1
    }
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
