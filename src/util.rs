use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

use std::slice::Iter;
use std::str::FromStr;
use itertools::{Itertools};

pub fn parse_int_lists(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    let lines: Vec<&str> = contents.lines()
        .collect();
    lines.split(|s| s.is_empty())
        .map(|ss| ss.iter().filter_map(|s| s.parse::<i32>().ok()).collect())
        .collect()
}

pub fn parse_strings(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse_from_strings<T: FromStr>(file_path: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines()
        .map(|s| s.parse().unwrap())
        .collect()
}


#[derive(PartialEq, Debug)]
pub struct Grid<T> {
    inner: HashMap<(i32, i32), T>,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl<T> Grid<T> {
    pub fn from_lines(f: impl Fn(char) -> T + Sized, lines: Iter<&str>) -> Grid<T> {
        let mut inner = HashMap::new();

        for (row, line) in lines.enumerate() {
            for (col, c) in line.chars().enumerate() {
                inner.insert((col as i32, row as i32), f(c));
            }
        }

        let Bounds {min_x, max_x, min_y, max_y} = bounds(&inner);

        Grid {
            inner,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}


pub struct Bounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32
}

pub fn bounds<T>(inner: &HashMap<(i32, i32), T>) -> Bounds {
    let min_x = inner.keys().map(|k| k.0).min().unwrap();
    let max_x = inner.keys().map(|k| k.0).max().unwrap();
    let min_y = inner.keys().map(|k| k.1).min().unwrap();
    let max_y = inner.keys().map(|k| k.1).max().unwrap();
    Bounds {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

impl<T: Clone> Grid<T> {
    pub fn value_at(&self, x: i32, y: i32) -> Option<T> {
        self.inner.get(&(x, y)).cloned()
    }

    pub fn entry_at(&self, x: i32, y: i32) -> Option<((i32, i32), T)> {
        self.inner.get(&(x, y)).map(|v| ((x, y), v.clone()))
    }

    fn dir(&self, start_x: i32, start_y: i32, d_x: i32, d_y: i32) -> Vec<T> {
        (0..)
            .into_iter()
            .map(|i| self.inner.get(&(start_x + i * d_x, start_y + i * d_y)))
            .take_while(|o| o.is_some())
            .map(|o| o.unwrap())
            .cloned()
            .collect_vec()
    }

    pub fn right(&self, start_x: i32, start_y: i32) -> Vec<T> {
        self.dir(start_x, start_y, 1, 0)
    }

    pub fn left(&self, start_x: i32, start_y: i32) -> Vec<T> {
        self.dir(start_x, start_y, -1, 0)
    }

    pub fn down(&self, start_x: i32, start_y: i32) -> Vec<T> {
        self.dir(start_x, start_y, 0, 1)
    }

    pub fn up(&self, start_x: i32, start_y: i32) -> Vec<T> {
        self.dir(start_x, start_y, 0, -1)
    }

    pub fn edges(&self) -> Vec<(((i32, i32), T), ((i32, i32), T))> {
        let mut v = vec![];
        for x in self.min_x..=self.max_x - 1 {
            for y in self.min_y..=self.max_y {
                if let (Some(a), Some(b)) = (self.entry_at(x, y), self.entry_at(x + 1, y)) {
                    v.push((a.clone(), b.clone()));
                    v.push((b, a));
                }
            }
        }
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y - 1 {
                if let (Some(a), Some(b)) = (self.entry_at(x, y), self.entry_at(x, y + 1)) {
                    v.push((a.clone(), b.clone()));
                    v.push((b, a));
                }
            }
        }
        v
    }
}

impl<T: Clone + Eq + PartialEq> Grid<T> {
    pub fn location_of(&self, value: &T) -> Option<(i32, i32)> {
        self.inner.iter()
            .find(|((_x, _y), v)| *v == value)
            .map(|(k, _)| k.clone())
    }
}


impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                write!(f, "{}", self.inner.get(&(x, y)).map(|v| v.to_string()).unwrap_or(" ".to_string()))?;
            }
            if y < self.max_y {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

pub fn parse_grid<T>(file_path: &str, f: impl Fn(char) -> T) -> Grid<T> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    Grid::from_lines(f, contents.lines().collect_vec().iter())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Write;
    use crate::util::{Grid, parse_grid, parse_int_lists, parse_strings};
    use tempfile::NamedTempFile;

    #[test]
    fn parses_int_lists() {
        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("43\n\n2\n20\n22\n\n1\n1\n1\n1\n1\n".as_bytes()).expect("Failed to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");

        let lists = parse_int_lists(filename);

        let expected_lists: Vec<Vec<i32>> = vec![
            vec![43],
            vec![2, 20, 22],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(lists, expected_lists);
    }

    #[test]
    fn parses_strings() {
        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("This\nis a\nFile!\n".as_bytes()).expect("Failed to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");

        let strings = parse_strings(filename);

        let expected_strings: Vec<String> = vec![
            "This".to_string(),
            "is a".to_string(),
            "File!".to_string(),
        ];
        assert_eq!(strings, expected_strings);
    }

    #[test]
    fn parses_grid() {
        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("12\n34".as_bytes()).expect("Failed to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");
        let grid = Grid {
            inner: HashMap::from([
                ((0, 0), 1),
                ((1, 0), 2),
                ((0, 1), 3),
                ((1, 1), 4),
            ]),
            min_x: 0,
            max_x: 1,
            min_y: 0,
            max_y: 1,
        };
        assert_eq!(parse_grid(filename, |c| c.to_string().parse::<i32>().unwrap()), grid);
    }
}