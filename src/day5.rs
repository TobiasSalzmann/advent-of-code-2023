use crate::util::AdventHelper;
use itertools::Itertools;
use std::cmp::{max, min};

use std::ops::Range;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines: Vec<String> = advent.parse_from_strings();
    let almanac = parse_seed_maps(&lines);

    advent.part1("Lowest location number: {}", lowest_location_number(&almanac));
    advent.part2("Lowest location number: {}", lowest_location_range_number(&almanac));
}

fn lowest_location_number(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|n| {
            almanac
                .maps
                .iter()
                .fold(n.clone(), |number, map| map.apply(number))
        })
        .min()
        .unwrap()
}

fn lowest_location_range_number(almanac: &Almanac) -> u64 {
    let mut seed_ranges = almanac.seed_ranges();
    for map in almanac.maps.iter() {
        let old_seed_ranges = seed_ranges.clone();
        seed_ranges = vec![];
        for seed_range in old_seed_ranges {
            for new_range in map.apply_range(seed_range) {
                seed_ranges.push(new_range)
            }
        }
    }
    seed_ranges.iter().map(|r| r.start).min().unwrap()
}

#[derive(Debug)]
struct AlmanacMapping {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

impl AlmanacMapping {
    fn source_range(&self) -> Range<u64> {
        self.source_range_start..(self.source_range_start + self.range_length)
    }

    fn apply(&self, number: u64) -> u64 {
        self.destination_range_start + (number - self.source_range_start)
    }
}

#[derive(Debug)]
struct AlmanacMap {
    mappings: Vec<AlmanacMapping>,
}

impl AlmanacMap {
    fn apply(&self, number: u64) -> u64 {
        for mapping in &self.mappings {
            if mapping.source_range().contains(&number) {
                return mapping.destination_range_start + (number - mapping.source_range_start);
            }
        }
        number
    }

    fn apply_range(&self, numbers: Range<u64>) -> Vec<Range<u64>> {
        let sorted_mappings = self
            .mappings
            .iter()
            .sorted_by_key(|m| m.source_range_start)
            .collect_vec();

        let mut start = numbers.start;
        let end = numbers.end;
        let mut mappings = vec![];

        for mapping in sorted_mappings.clone() {
            let m_range = mapping.source_range();
            let before_bit = start..min(m_range.start, numbers.end);
            if !before_bit.is_empty() {
                mappings.push(before_bit)
            }
            let overlapping_bit = max(numbers.start, m_range.start)..min(numbers.end, m_range.end);
            if !overlapping_bit.is_empty() {
                mappings
                    .push(mapping.apply(overlapping_bit.start)..mapping.apply(overlapping_bit.end))
            }
            start = m_range.end
        }

        let remaining = start..end;
        if !remaining.is_empty() {
            mappings.push(start..end)
        }

        mappings
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_ranges(&self) -> Vec<Range<u64>> {
        self.seeds.iter()
            .cloned()
            .tuples()
            .map(|(a, b)| a..a+b)
            .collect_vec()
    }
}

fn parse_seed_maps(lines: &Vec<String>) -> Almanac {
    let mut seeds = vec![];
    let mut maps = vec![];
    for line in lines {
        if let Some(line) = line.strip_prefix("seeds: ") {
            seeds = line.split(" ").map(|n| n.parse().unwrap()).collect_vec()
        } else if line.ends_with(" map:") {
            maps.push(AlmanacMap { mappings: vec![] })
        } else if !line.is_empty() {
            let (destination_range_start, source_range_start, range_length) = line
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            maps.last_mut().unwrap().mappings.push(AlmanacMapping {
                source_range_start,
                destination_range_start,
                range_length,
            })
        }
    }

    Almanac { seeds, maps }
}
