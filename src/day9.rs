use crate::util::AdventHelper;
use itertools::Itertools;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let sequences: Vec<Vec<i32>> = advent.parse_sequences_from_strings(" ");

    advent.part1(
        "sum of extrapolated values: {}",
        sum_extrapolated_values(&sequences),
    );
    advent.part2(
        "sum of extrapolated values: {}",
        sum_extrapolated_first_values(&sequences),
    );
}

fn sum_extrapolated_values(sequences: &Vec<Vec<i32>>) -> i32 {
    sequences.iter().map(extrapolated_value).sum()
}

fn sum_extrapolated_first_values(sequences: &Vec<Vec<i32>>) -> i32 {
    sequences.iter().map(extrapolated_first_value).sum()
}

fn extrapolated_value(sequence: &Vec<i32>) -> i32 {
    let derivatives = derivatives(sequence);
    derivatives.iter().map(|x| x.last().unwrap()).sum()
}

fn extrapolated_first_value(sequence: &Vec<i32>) -> i32 {
    let derivatives = derivatives(sequence);
    let mut value = 0;
    for x in derivatives.iter().rev() {
        value = x.first().unwrap() - value
    }
    value
}

fn derivatives(sequence: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut derivatives = vec![sequence.clone()];
    while derivatives.last().unwrap().iter().any(|x| *x != 0) {
        let next_derivative = derivatives
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        derivatives.push(next_derivative);
    }
    derivatives
}
