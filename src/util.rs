use itertools::{Itertools, Product};

use crate::util::Dir::{Down, Left, Right, Up};
use array2d::Array2D;
use bit_set::{BitSet, Iter};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Map;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{env, fs};

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
        let it = Self {
            day: day(file_name).parse().unwrap(),
            suffix: "".to_string(),
        };

        if env::var("TEST").is_ok() {
            it.test()
        } else {
            it
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

    pub fn parse_grid(&self) -> Vec<Vec<char>> {
        let lines: Vec<String> = parse_from_strings(&self.input_file());
        lines.iter().map(|s| s.chars().collect_vec()).collect_vec()
    }

    pub fn parse_grid_2d(&self) -> Array2D<char> {
        Array2D::from_rows(&self.parse_grid()).unwrap()
    }

    pub fn parse_from_grid(&self) -> Array2D<i32> {
        let lines: Vec<String> = parse_from_strings(&self.input_file());
        let vec: Vec<Vec<i32>> = lines
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        Array2D::from_rows(&vec).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

pub trait IntoUnsafe<T>: Sized {
    fn into_unsafe(self) -> T;
}

impl IntoUnsafe<i32> for usize {
    fn into_unsafe(self) -> i32 {
        self as i32
    }
}

impl<T> IntoUnsafe<T> for T {
    fn into_unsafe(self) -> T {
        self
    }
}

impl Point {
    pub(crate) fn new(x: impl IntoUnsafe<i32>, y: impl IntoUnsafe<i32>) -> Point {
        Point {
            x: x.into_unsafe(),
            y: y.into_unsafe(),
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

    pub fn mv(&self, d: Dir) -> Point {
        match d {
            Up => self.up(),
            Right => self.right(),
            Down => self.down(),
            Left => self.left(),
        }
    }

    pub fn mv_mulitple(&self, d: Dir, n: usize) -> Point {
        let n = n as i32;
        match d {
            Up => Point::new(self.x, self.y - n),
            Right => Point::new(self.x + n, self.y),
            Down => Point::new(self.x, self.y + n),
            Left => Point::new(self.x - n, self.y),
        }
    }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![self.up(), self.down(), self.left(), self.right()]
    }

    pub fn in_bounds(&self, b: &Bounds) -> bool {
        b.contains(self)
    }

    pub fn bounds<H>(col: &HashSet<Point, H>) -> Bounds {
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

pub struct BitSetGrid {
    height: usize,
    width: usize,
    inner: BitSet,
}

impl<'a> IntoIterator for &'a BitSetGrid {
    type Item = Point;
    type IntoIter = BitSetGridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetGridIter {
            width: self.width,
            inner: self.inner.into_iter(),
        }
    }
}

pub struct BitSetGridIter<'a> {
    width: usize,
    inner: Iter<'a, u32>,
}

impl<'a> Iterator for BitSetGridIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|v| Point {
            x: (v % self.width) as i32,
            y: (v / self.width) as i32,
        })
    }
}

impl BitSetGrid {
    pub fn new(height: usize, width: usize) -> BitSetGrid {
        BitSetGrid {
            height,
            width,
            inner: BitSet::with_capacity(width * height),
        }
    }

    pub fn from_hashset<H>(col: &HashSet<Point, H>) -> BitSetGrid {
        let bounds = Point::bounds(col);
        let mut grid = BitSetGrid::new(bounds.max_y as usize + 1, bounds.max_x as usize + 1);
        for p in col {
            grid.insert(p);
        }
        grid
    }

    pub fn insert(&mut self, point: &Point) -> bool {
        self.inner
            .insert((point.y as usize) * self.width + point.x as usize)
    }

    pub fn remove(&mut self, point: &Point) -> bool {
        self.inner
            .remove((point.y as usize) * self.width + point.x as usize)
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.inner
            .contains((point.y as usize) * self.width + point.x as usize)
    }

    pub fn bounds(&self) -> Bounds {
        Bounds {
            min_x: 0,
            max_x: (self.width - 1) as i32,
            min_y: 0,
            max_y: (self.height - 1) as i32,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn cw(&self) -> Dir {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn ccw(&self) -> Dir {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    pub fn pivot(&self) -> Dir {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
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

impl IntoIterator for Bounds {
    type Item = Point;
    type IntoIter = Map<Product<RangeInclusive<i32>, RangeInclusive<i32>>, fn((i32, i32)) -> Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.xs()
            .cartesian_product(self.ys())
            .map(|(x, y)| Point { x, y })
    }
}

impl Bounds {
    pub fn xs(&self) -> RangeInclusive<i32> {
        self.min_x..=self.max_x
    }

    pub fn ys(&self) -> RangeInclusive<i32> {
        self.min_y..=self.max_y
    }

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

    pub fn to_set(&self) -> HashSet<Point> {
        let mut set = HashSet::new();
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y {
                set.insert(Point::new(x, y));
            }
        }
        set
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
