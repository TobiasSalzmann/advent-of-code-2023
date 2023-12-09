use crate::util::AdventHelper;
use itertools::Itertools;

use std::collections::HashMap;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines: Vec<String> = advent.parse_from_strings();
    let network = parse(&lines);

    advent.part1(
        "steps: {}",
        count_steps(&network, "AAA".to_string(), |s| s == "ZZZ"),
    );
    advent.part2("steps: {}", count_ghost_steps(&network));
}
fn count_steps(network: &Network, begin: String, end: fn(&String) -> bool) -> i32 {
    let instructions = network.instructions.chars().cycle();
    let mut steps = 0;
    let mut current = begin;
    for instruction in instructions {
        current = network
            .connections
            .get(&(current.clone(), instruction))
            .unwrap()
            .clone();
        steps += 1;
        if end(&current) {
            break;
        }
    }
    steps
}

fn count_ghost_steps(network: &Network) -> u64 {
    let current: Vec<String> = network
        .connections
        .keys()
        .map(|(s, _)| s.clone())
        .filter(|x| x.ends_with("A"))
        .unique()
        .collect_vec();
    let mut lcm = 1;
    for x in current {
        let y = count_steps(network, x.clone(), |x| x.ends_with("Z")) as u64;
        lcm = num::integer::lcm(lcm, y);
    }
    lcm
}

fn parse(lines: &Vec<String>) -> Network {
    let instructions = lines[0].clone();
    let connections: HashMap<(String, char), String> = lines[2..]
        .iter()
        .cloned()
        .flat_map(|line| {
            vec![
                ((line[0..=2].to_string(), 'L'), line[7..=9].to_string()),
                ((line[0..=2].to_string(), 'R'), line[12..=14].to_string()),
            ]
        })
        .collect();

    Network {
        instructions,
        connections,
    }
}

struct Network {
    instructions: String,
    connections: HashMap<(String, char), String>,
}
