use crate::day19::Action::{Accept, Call, Reject};
use crate::day19::Case::{Any, GreaterThan, LessThan};
use crate::util::Dir::{Down, Left, Right, Up};
use crate::util::{AdventHelper, Bounds, Dir, Point};
use itertools::Itertools;
use pathfinding::prelude::dfs_reach;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::alloc::System;
use std::collections::HashSet;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let system: Syst = parse_system(&advent.parse_from_strings());

    advent.part1("Count accepted: {}", count_accepted(&system));
    advent.part2("Part 2: {}", part2(&system))
}

fn part2(system: &Syst) -> usize {
    system
        .functions
        .values()
        .flat_map(|f| &f.cases)
        .flat_map(|c| match c {
            GreaterThan('x', x, _) => vec![x],
            LessThan('x', x, _) => vec![x],
            _ => vec![],
        })
        .sorted()
        .unique()
        .count()
}

fn count_accepted(system: &Syst) -> i32 {
    system
        .parts
        .iter()
        .filter(|p| is_accepted(p, &system.functions))
        .map(|p| {
            let x: i32 = p.values().sum();
            x
        })
        .sum()
}

fn is_accepted(part: &Part, map: &FxHashMap<String, Function>) -> bool {
    let mut current_function = &map["in"];
    'outer: loop {
        for case in &current_function.cases {
            let action = match case {
                GreaterThan(key, check, action) if part[key] > *check => Some(action),
                LessThan(key, check, action) if part[key] < *check => Some(action),
                Any(action) => Some(action),
                _ => None,
            };
            match action {
                Some(Accept) => return true,
                Some(Reject) => return false,
                Some(Call(next_fun)) => {
                    current_function = &map[next_fun];
                    continue 'outer;
                }
                None => {}
            }
        }
    }
}

fn parse_system(lines: &Vec<String>) -> Syst {
    let (raw_functions, raw_parts) = lines.split(|l| l.is_empty()).collect_tuple().unwrap();
    Syst {
        functions: raw_functions.iter().map(|f| parse_function(f)).collect(),
        parts: raw_parts.iter().map(|p| parse_part(p)).collect(),
    }
}

fn parse_part(raw_part: &String) -> Part {
    raw_part
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(|s| (s.chars().next().unwrap(), s[2..].parse().unwrap()))
        .collect()
}

fn parse_function(raw_function: &String) -> (String, Function) {
    let (name, raw_body) = raw_function
        .strip_suffix('}')
        .unwrap()
        .split('{')
        .collect_tuple()
        .unwrap();
    let cases = raw_body.split(',').map(|s| parse_case(s)).collect_vec();
    (name.to_string(), Function { cases })
}

fn parse_case(raw_case: &str) -> Case {
    //tq{s<1378:A,x>1056:A,x<975:A,R}
    let split = raw_case.split([':']).collect_vec();
    match &split[..] {
        [action] => Any(parse_action(*action)),
        [condition, action] if condition.contains('<') => LessThan(
            condition.chars().next().unwrap(),
            condition[2..].parse().unwrap(),
            parse_action(*action),
        ),
        [condition, action] if condition.contains('>') => GreaterThan(
            condition.chars().next().unwrap(),
            condition[2..].parse().unwrap(),
            parse_action(*action),
        ),
        _ => panic!(),
    }
}

fn parse_action(raw_action: &str) -> Action {
    match raw_action {
        "A" => Accept,
        "R" => Reject,
        label => Call(label.to_string()),
    }
}

struct Syst {
    functions: FxHashMap<String, Function>,
    parts: Vec<Part>,
}

struct Function {
    cases: Vec<Case>,
}

enum Case {
    GreaterThan(char, i32, Action),
    LessThan(char, i32, Action),
    Any(Action),
}

enum Action {
    Call(String),
    Accept,
    Reject,
}

type Part = FxHashMap<char, i32>;
