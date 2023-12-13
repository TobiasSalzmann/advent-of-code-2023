use crate::util::{AdventHelper, Point};
use itertools::{Itertools, repeat_n};
use std::cmp::max;

use num::abs;

use std::collections::HashSet;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let springs: Vec<Spring> = advent.parse_from_strings();

    advent.part1("sum of distances: {}", count_total_combinations(&springs, 1));
    advent.part2("sum of distances: {}", count_total_combinations(&springs, 5));
}

fn count_total_combinations(springs: &Vec<Spring>, multiplicity: usize) -> i32 {
    springs.iter().map(|x| {
        count_combinations(&Spring{
            row: repeat_n(&x.row, multiplicity).join("?"),
            constraint: x.constraint.repeat(multiplicity),
        })
    }).sum()
}

fn count_combinations(spring: &Spring) -> i32 {
    let colored_sum: i32 = spring.constraint.iter().sum();
    let length = spring.row.len() as i32;
    let number_of_empty = length - colored_sum;
    let number_of_blocks = spring.constraint.len();
    let mut count_valid_combinations = 0;
    let combinations = (0..=number_of_empty).combinations(number_of_blocks);
    for combination in combinations {
        let mut aggregate_sum = 0;
        let mut colored = HashSet::new();
        for (shift, block_length) in combination.iter().zip(&spring.constraint) {
            let start = (aggregate_sum + shift) as usize;
            let end = start + (*block_length as usize);
            for i in start..end {
                colored.insert(i);
            }
            aggregate_sum += block_length
        }
        let is_valid = spring.row.chars().enumerate().all(|(i, c)| {
            if colored.contains(&i) {
                c != '.'
            } else {
                c != '#'
            }
        });
        if is_valid {
            count_valid_combinations += 1
        }

    }

    count_valid_combinations
}

#[derive(Debug)]
struct Spring {
    row: String,
    constraint: Vec<i32>
}

impl FromStr for Spring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, raw_constraint) = s.split(" ").collect_tuple().unwrap();
        let constraint = raw_constraint.split(",").map(|x| x.parse().unwrap()).collect_vec();
        Ok(Spring { row: row.to_string(), constraint })
    }
}


