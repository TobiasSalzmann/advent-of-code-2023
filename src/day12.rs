use crate::util::AdventHelper;
use itertools::{repeat_n, Itertools};

use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let springs: Vec<Spring> = advent.parse_from_strings();

    advent.part1(
        "number of arrangements: {}",
        count_total_combinations(&springs, 1),
    );
    advent.part2(
        "number of arrangements: {}",
        count_total_combinations(&springs, 5),
    );
}

fn count_total_combinations(springs: &[Spring], multiplicity: usize) -> u64 {
    springs
        .iter()
        .map(|x| {
            count_combinations(&Spring {
                row: repeat_n(&x.row, multiplicity).join("?") + ".",
                constraint: x.constraint.repeat(multiplicity),
            })
        })
        .sum()
}

fn count_combinations(spring: &Spring) -> u64 {
    let total_length = spring.row.len();
    let total_blocks = spring.constraint.len();
    let mut combinations: FxHashMap<(usize, usize), u64> = (0..=total_blocks)
        .map(|number_of_blocks| {
            (
                (0, number_of_blocks),
                if number_of_blocks == 0 { 1 } else { 0 },
            )
        })
        .collect();

    for prefix_length in 1..=total_length {
        let new_character = spring.row.chars().nth(prefix_length - 1).unwrap();

        let prev = combinations[&(prefix_length - 1, 0)];
        combinations.insert(
            (prefix_length, 0),
            if new_character == '#' { 0 } else { prev },
        );

        for num_blocks in 1..=total_blocks {
            let new_block = spring.constraint[num_blocks - 1] as usize;
            let mut count = 0;
            if new_character != '#' {
                count += combinations[&(prefix_length - 1, num_blocks)];
                if prefix_length > new_block {
                    let block_location =
                        &spring.row[prefix_length - new_block - 1..prefix_length - 1];
                    if block_location.chars().all(|c| c != '.') {
                        count += combinations[&(prefix_length - new_block - 1, num_blocks - 1)]
                    }
                }
            }

            combinations.insert((prefix_length, num_blocks), count);
        }
    }
    combinations[&(total_length, total_blocks)]
}

#[derive(Debug)]
struct Spring {
    row: String,
    constraint: Vec<i32>,
}

impl FromStr for Spring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, raw_constraint) = s.split(" ").collect_tuple().unwrap();
        let constraint = raw_constraint
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect_vec();
        Ok(Spring {
            row: row.to_string(),
            constraint,
        })
    }
}
