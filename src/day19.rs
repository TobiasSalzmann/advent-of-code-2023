use crate::day19::Action::{Accept, Call, Reject};
use crate::day19::Case::{Any, GreaterThan, LessThan};
use crate::util::AdventHelper;
use itertools::Itertools;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

use rustc_hash::FxHashMap;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let system: Syst = parse_system(&advent.parse_from_strings());

    advent.part1("Count accepted: {}", count_accepted(&system));
    advent.part2("All accepted: {}", count_total_accepted(&system.functions))
}

fn count_total_accepted(map: &FxHashMap<String, Function>) -> u64 {
    let mut queue = vec![(
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
        &map["in"],
    )];

    let mut accepted_count = 0;

    while let Some((mut part, current_function)) = queue.pop() {
        for case in &current_function.cases {
            let (action, finished_part) = match case {
                GreaterThan(key, check, action) => {
                    let (p1, p2) = part.split(*key, check + 1);
                    part = p1;
                    (action, p2)
                }
                LessThan(key, check, action) => {
                    let (p1, p2) = part.split(*key, *check);
                    part = p2;
                    (action, p1)
                }
                Any(action) => (action, part.clone()),
            };
            if finished_part.is_empty() {
                break;
            }
            match action {
                Accept => accepted_count += finished_part.size(),
                Call(next_fun) => {
                    queue.push((finished_part, &map[next_fun]));
                }
                _ => {}
            }
        }
    }
    accepted_count
}

#[derive(Clone)]
struct PartRange {
    x: RangeInclusive<i32>,
    m: RangeInclusive<i32>,
    a: RangeInclusive<i32>,
    s: RangeInclusive<i32>,
}

impl PartRange {
    fn size(&self) -> u64 {
        let x = max(self.x.end() - self.x.start() + 1, 0) as u64;
        let m = max(self.m.end() - self.m.start() + 1, 0) as u64;
        let a = max(self.a.end() - self.a.start() + 1, 0) as u64;
        let s = max(self.s.end() - self.s.start() + 1, 0) as u64;
        x * m * a * s
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    fn split(&self, c: char, value: i32) -> (Self, Self) {
        match c {
            'x' => (
                PartRange {
                    x: *self.x.start()..=min(value - 1, *self.x.end()),
                    ..self.clone()
                },
                PartRange {
                    x: max(value, *self.x.start())..=*self.x.end(),
                    ..self.clone()
                },
            ),
            'm' => (
                PartRange {
                    m: *self.m.start()..=min(value - 1, *self.m.end()),
                    ..self.clone()
                },
                PartRange {
                    m: max(value, *self.m.start())..=*self.m.end(),
                    ..self.clone()
                },
            ),
            'a' => (
                PartRange {
                    a: *self.a.start()..=min(value - 1, *self.a.end()),
                    ..self.clone()
                },
                PartRange {
                    a: max(value, *self.a.start())..=*self.a.end(),
                    ..self.clone()
                },
            ),
            's' => (
                PartRange {
                    s: *self.s.start()..=min(value - 1, *self.s.end()),
                    ..self.clone()
                },
                PartRange {
                    s: max(value, *self.s.start())..=*self.s.end(),
                    ..self.clone()
                },
            ),

            _ => panic!(),
        }
    }
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
