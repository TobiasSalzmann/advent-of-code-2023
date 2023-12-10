use itertools::Itertools;

use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::fs;
use std::str::FromStr;

pub fn parse_from_strings<T: FromStr>(file_path: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines().map(|s| s.parse().unwrap()).collect()
}

pub fn parse_strings(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn day(file_name: &str) -> &str {
    file_name
        .strip_prefix("src/day")
        .unwrap()
        .strip_suffix(".rs")
        .unwrap()
}

pub(crate) struct AdventHelper {
    day: u32,
    suffix: String,
}

impl AdventHelper {
    pub fn from_file_name(file_name: &str) -> Self {
        Self {
            day: day(file_name).parse().unwrap(),
            suffix: "".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn test(&self) -> Self {
        Self {
            day: self.day,
            suffix: ".test".to_string(),
        }
    }

    pub fn input_file(&self) -> String {
        format!("resources/day{}{}.txt", self.day, self.suffix)
    }

    pub fn part1<T: Display>(&self, template: &str, output: T) {
        self.part(1, template, output)
    }
    pub fn part2<T: Display>(&self, template: &str, output: T) {
        self.part(2, template, output)
    }

    fn part<T: Display>(&self, part_num: u32, template: &str, output: T) {
        let actual_output = template.replace("{}", &output.to_string());
        println!("Day {}, Part {}: {}", self.day, part_num, actual_output)
    }

    pub fn parse_from_strings<T: FromStr>(&self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        parse_from_strings(&self.input_file())
    }

    pub fn parse_sequences_from_strings<T: FromStr>(&self, separator: &str) -> Vec<Vec<T>>
    where
        <T as FromStr>::Err: Debug,
    {
        let lines: Vec<String> = parse_from_strings(&self.input_file());
        lines
            .iter()
            .map(|line| {
                line.split(separator)
                    .map(|x| x.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub(crate) fn new(x: impl Into<i32>, y: impl Into<i32>) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![self.up(), self.down(), self.left(), self.right()]
    }

    pub fn in_bounds(&self, b: &Bounds) -> bool {
        b.contains(self)
    }

    pub fn bounds(col: &HashSet<Point>) -> Bounds {
        let min_x = col.iter().map(|p| p.x).min().unwrap();
        let max_x = col.iter().map(|p| p.x).max().unwrap();
        let min_y = col.iter().map(|p| p.y).min().unwrap();
        let max_y = col.iter().map(|p| p.y).max().unwrap();
        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl Bounds {
    pub fn contains(&self, p: &Point) -> bool {
        self.min_x <= p.x && p.x <= self.max_x && self.min_y <= p.y && p.y <= self.max_y
    }

    pub fn expand(&self, _n: i32) -> Bounds {
        Bounds {
            min_x: self.min_x - 1,
            max_x: self.max_x + 1,
            min_y: self.min_y - 1,
            max_y: self.max_y + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::parse_strings;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_strings() {
        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("This\nis a\nFile!\n".as_bytes())
            .expect("Failed to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");

        let strings = parse_strings(filename);

        let expected_strings: Vec<String> =
            vec!["This".to_string(), "is a".to_string(), "File!".to_string()];
        assert_eq!(strings, expected_strings);
    }
}
