use crate::util;
use itertools::{Itertools};
use crate::util::day;

pub fn main() {
    let day = day(file!());
    let file_name = &format!("resources/day{}.txt", day);
    let input = util::parse_strings(file_name);

    println!("Day 1, Part 1: Calibration value {:?}", calibration_value(input.clone(), false));
    println!("Day 1, Part 2: Calibration value {:?}", calibration_value(input.clone(), true));
}

fn calibration_value(calibration_strings: Vec<String>, allow_words: bool) -> i32 {
    calibration_strings.iter()
        .map(|s| find_digits(s, allow_words))
        .sum()
}

const NUMBER_WORDS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_digits(s: &str, allow_words: bool) -> i32 {
    let substrs = substrings(s);
    let matches = substrs.iter()
        .filter_map(|s| try_digit(s, allow_words))
        .collect_vec();
    let first = matches.first().unwrap();
    let last = matches.last().unwrap();
    first * 10 + last
}

fn try_digit(str: &str, allow_words: bool) -> Option<i32> {
    (0..10).find_map(|d| {
        if str.starts_with(&d.to_string()) || allow_words && str.starts_with(NUMBER_WORDS[d]) {
            Some(d as i32)
        } else {
            None
        }
    })
}

fn substrings(s: &str) -> Vec<&str> {
    (0..s.len())
        .map(|i| &s[i..])
        .collect_vec()
}

