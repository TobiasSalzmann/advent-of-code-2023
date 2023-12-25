use crate::util::AdventHelper;
use gomez::algo::trust_region::TrustRegionOptions;
use gomez::algo::{Lipo, NelderMead, TrustRegion};
use gomez::{nalgebra as na, OptimizerDriver};
use gomez::{Domain, Problem, SolverDriver, System};
use itertools::Itertools;
use na::{Dyn, IsContiguous};
use num::complex::ComplexFloat;
use num::{signum, Num};
use plotters::prelude::*;
use std::fmt::Display;
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

fn intersecting_paths(
    hailstones: &Vec<Hailstone<i128>>,
    lower_bound: f64,
    upper_bound: f64,
) -> usize {
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
#[derive(Debug, Clone)]
struct Hailstone<T> {
    position: Vec3<T>,
    velocity: Vec3<T>,
}

#[derive(Copy, Clone, Debug)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl Vec3<f64> {
    fn rotate(&self, alpha: f64, beta: f64) -> Vec3<f64> {
        Vec3 {
            x: alpha.cos() * beta.cos() * self.x - alpha.sin() * self.y
                + alpha.cos() * beta.sin() * self.z,
            y: alpha.sin() * beta.cos() * self.x
                + alpha.cos() * self.y
                + alpha.sin() * beta.sin() * self.z,
            z: -beta.sin() * self.x + beta.cos() * self.z,
        }
    }
}

impl<T: Add> Add for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl FromStr for Hailstone<i128> {
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

fn projected_intersections(hailstones: &Vec<Hailstone<f64>>) -> Vec<Vec2> {
    let mut intersections = vec![];
    for (a, b) in hailstones.iter().tuple_combinations() {
        let Vec3 { x: x1, y: y1, z: _ } = a.position;
        let Vec3 { x: x2, y: y2, z: _ } = a.position + a.velocity;
        let Vec3 { x: x3, y: y3, z: _ } = b.position;
        let Vec3 { x: x4, y: y4, z: _ } = b.position + b.velocity;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        if denom == 0.0 {
            continue;
        }
        let x = x_num as f64 / denom as f64;
        let y = y_num as f64 / denom as f64;

        intersections.push(Vec2 { x, y })
    }
    intersections
}
