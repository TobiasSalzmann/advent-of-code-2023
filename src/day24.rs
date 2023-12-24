use crate::util::AdventHelper;
use itertools::Itertools;
use num::signum;
use std::ops::Add;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let hailstones = advent.parse_from_strings();

    advent.part1(
        "Longest walk: {}",
        intersecting_paths(&hailstones, 200000000000000.0, 400000000000000.0),
        // intersecting_paths(&hailstones, 7.0, 27.0),
    );
}

fn intersecting_paths(hailstones: &Vec<Hailstone>, lower_bound: f64, upper_bound: f64) -> usize {
    let mut count = 0;
    for (a, b) in hailstones.iter().tuple_combinations() {
        let Vec3 { x: x1, y: y1, z: _ } = a.position;
        let Vec3 { x: x2, y: y2, z: _ } = a.position + a.velocity;
        let Vec3 { x: x3, y: y3, z: _ } = b.position;
        let Vec3 { x: x4, y: y4, z: _ } = b.position + b.velocity;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        if denom == 0 {
            continue;
        }
        let x = x_num as f64 / denom as f64;
        let y = y_num as f64 / denom as f64;

        let is_in_bounds =
            lower_bound <= x && x <= upper_bound && lower_bound <= y && y <= upper_bound;
        let is_in_positive_directions = signum(x - x1 as f64) == signum(x2 as f64 - x1 as f64)
            && signum(x - x3 as f64) == signum(x4 as f64 - x3 as f64);

        if is_in_bounds && is_in_positive_directions {
            count += 1;
            // println!("{a:?} {b:?} {denom} {x_num} {y_num}");
        };
    }
    count
}
#[derive(Debug)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s
            .split(" @ ")
            .map(|v| {
                let (x, y, z) = v
                    .split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Vec3 { x, y, z }
            })
            .collect_tuple()
            .unwrap();
        Ok(Hailstone { position, velocity })
    }
}
