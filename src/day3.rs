use crate::util::{AdventHelper, Point};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines: Vec<String> = advent.parse_from_strings();
    let symbols = find_symbols(&lines);
    let (part_numbers, parts) = find_labels(&lines, &symbols);

    advent.part1("Sum of part numbers: {}", sum_part_numbers(&part_numbers));
    advent.part2("Sum of gear ratios: {}", sum_gear_ratios(&parts));
}

fn sum_part_numbers(parts: &Vec<u32>) -> u32 {
    parts.iter().sum()
}

fn sum_gear_ratios(parts: &Vec<Part>) -> u32 {
    let _count = 0;
    parts
        .iter()
        .filter(|p| p.symbol == '*')
        .into_group_map_by(|g| g.position)
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[0].number * parts[1].number)
        .sum()
}

fn find_symbols(lines: &Vec<String>) -> HashMap<Point, char> {
    let mut symbols = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                symbols.insert(Point::new(x as i32, y as i32), c);
            }
        }
    }
    symbols
}

#[derive(Debug, Clone)]
struct Part {
    number: u32,
    position: Point,
    symbol: char,
}

fn find_labels(lines: &Vec<String>, symbols: &HashMap<Point, char>) -> (Vec<u32>, Vec<Part>) {
    let mut part_numbers: Vec<u32> = vec![];
    let mut parts: Vec<Part> = vec![];
    let re = Regex::new(r"\d+").unwrap();
    for (y, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let number = m.as_str().parse().unwrap();
            let new_parts = make_part(number, m.start() as i32, y as i32, m.len(), symbols);
            if !new_parts.is_empty() {
                part_numbers.push(number)
            }
            for new_part in new_parts {
                parts.push(new_part);
            }
        }
    }
    (part_numbers, parts)
}

fn make_part(
    number: u32,
    label_x: i32,
    label_y: i32,
    length: usize,
    symbols: &HashMap<Point, char>,
) -> Vec<Part> {
    let mut parts = vec![];
    for y in (label_y - 1)..=(label_y + 1) {
        for x in (label_x - 1)..=(label_x + length as i32) {
            let position = Point { x, y };
            if symbols.contains_key(&position) {
                parts.push(Part {
                    number,
                    position: position.clone(),
                    symbol: symbols[&position],
                });
            }
        }
    }
    parts
}
