use crate::util::AdventHelper;
use itertools::{repeat_n, Itertools};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let instructions_raw = advent.parse_from_strings::<String>()[0].clone();
    let instructions = instructions_raw.split(",").collect_vec();

    advent.part1("checksum:  {}", checksum(instructions.clone()));
    advent.part2("power:  {}", focusing_power(instructions.clone()));
}

fn checksum(instructions: Vec<&str>) -> usize {
    instructions.iter().map(|x| ascii_value(x.as_bytes())).sum()
}

fn focusing_power(instructions: Vec<&str>) -> i32 {
    let mut boxes: Vec<Vec<(String, i32)>> = repeat_n(vec![], 256).collect_vec();
    for instruction in instructions {
        let (lens_id, raw_focal_value) = instruction.split(['=', '-']).collect_tuple().unwrap();
        let hash = ascii_value(lens_id.as_bytes());
        let is_assignment = !raw_focal_value.is_empty();

        if let Some(existing_idx) = boxes[hash].iter().position(|(id, _)| *id == lens_id) {
            if is_assignment {
                boxes[hash][existing_idx] = (lens_id.to_string(), raw_focal_value.parse().unwrap())
            } else {
                boxes[hash].remove(existing_idx);
            }
        } else if is_assignment {
            boxes[hash].push((lens_id.to_string(), raw_focal_value.parse().unwrap()))
        }
    }

    let mut sum = 0;

    for (box_idx, b) in boxes.iter().enumerate() {
        for (lens_idx, (_lens_id, focal)) in b.iter().enumerate() {
            sum += (box_idx as i32 + 1) * (lens_idx as i32 + 1) * focal
        }
    }
    sum
}

fn ascii_value(instruction: &[u8]) -> usize {
    let mut sum = 0;
    for b in instruction {
        sum += *b as usize;
        sum *= 17;
        sum %= 256;
    }
    sum
}
