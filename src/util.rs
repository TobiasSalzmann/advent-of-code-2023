use std::fmt::{Debug, Display};
use std::fs;
use std::str::FromStr;

pub fn parse_int_lists(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    let lines: Vec<&str> = contents.lines().collect();
    lines
        .split(|s| s.is_empty())
        .map(|ss| ss.iter().filter_map(|s| s.parse::<i32>().ok()).collect())
        .collect()
}

pub fn parse_from_strings<T: FromStr>(file_path: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn parse_strings(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn day(file_name: &str) -> &str {
    file_name.strip_prefix("src/day").unwrap()
        .strip_suffix(".rs").unwrap()
}

pub(crate) struct AdventHelper {
    day: u32,
    suffix: String
}

impl AdventHelper {
    pub fn from_file_name(file_name: &str) -> Self {
        Self { day: day(file_name).parse().unwrap(), suffix: "".to_string() }
    }

    pub fn test(&self) -> Self {
        Self { day: self.day, suffix: ".test".to_string()}
    }

    pub fn input_file(&self) -> String {
        format!("resources/day{}{}.txt", self.day, self.suffix)
    }

    /// Returns the value in seconds.
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

    pub fn parse_from_strings<T: FromStr>(&self) -> Vec<T> where <T as FromStr>::Err: Debug {
        parse_from_strings(&self.input_file())
    }
}



#[cfg(test)]
mod tests {
    use crate::util::{parse_int_lists, parse_strings};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_int_lists() {
        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("43\n\n2\n20\n22\n\n1\n1\n1\n1\n1\n".as_bytes())
            .expect("Failed to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");

        let lists = parse_int_lists(filename);

        let expected_lists: Vec<Vec<i32>> = vec![vec![43], vec![2, 20, 22], vec![1, 1, 1, 1, 1]];
        assert_eq!(lists, expected_lists);
    }

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
