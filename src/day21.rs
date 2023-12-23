use crate::util::{AdventHelper, Point};
use array2d::Array2D;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let garden = Array2D::from_rows(&advent.parse_grid()).unwrap();

    advent.part1("Reachable plots: {}", reachable_plots(&garden, 64));
    advent.part2("Reachable plots: {}", reachable_plots(&garden, 26501365));
}

fn reachable_plots(garden: &Array2D<char>, steps: usize) -> usize {
    let ((y, x), _) = garden
        .enumerate_row_major()
        .find(|((y, x), c)| **c == 'S')
        .unwrap();
    let mut cache = FxHashMap::default();
    let mut total = 0;
    let mut top_remaining = (steps as i64) - y as i64 - 1;
    while top_remaining > 0 {
        total += reachable_plots_row(
            &Point::new(x, garden.column_len() - 1),
            garden,
            top_remaining as usize,
            &mut cache,
        );
        top_remaining -= garden.column_len() as i64;
    }
    let mut bottom_remaining = steps as i64 - (garden.column_len() as i64 - y as i64);

    while bottom_remaining > 0 {
        total += reachable_plots_row(
            &Point::new(x, 0),
            garden,
            bottom_remaining as usize,
            &mut cache,
        );
        bottom_remaining -= garden.column_len() as i64;
    }
    total += reachable_plots_row(&Point::new(x, y), garden, steps, &mut cache);
    total
}

fn reachable_plots_row(
    start: &Point,
    garden: &Array2D<char>,
    steps: usize,
    cache: &mut FxHashMap<(Point, usize), usize>,
) -> usize {
    let modulus = garden.row_len();
    let base = steps % modulus;
    let first = base + modulus;
    let second = base + 2 * modulus;

    let base_res = *cache
        .entry((*start, base))
        .or_insert_with(|| reachable_plots_with_wrapped_rows(start, garden, base));
    if steps == base {
        return base_res;
    }
    let first_res = *cache
        .entry((*start, first))
        .or_insert_with(|| reachable_plots_with_wrapped_rows(start, garden, first));
    let second_res = *cache
        .entry((*start, second))
        .or_insert_with(|| reachable_plots_with_wrapped_rows(start, garden, second));
    let period = second_res - first_res;
    (steps / modulus - 1) * period + first_res
}

fn reachable_plots_with_wrapped_rows(start: &Point, garden: &Array2D<char>, steps: usize) -> usize {
    let mut current = FxHashSet::from_iter([start.clone()]);
    for _ in 1..=steps {
        current = current
            .iter()
            .flat_map(|p| p.neighbours())
            .filter(|p| {
                let tile = garden.get(
                    p.y as usize,
                    p.x.rem_euclid(garden.row_len() as i32) as usize,
                );
                tile == Some(&'.') || tile == Some(&'S')
            })
            .collect();
    }

    current.len()
}
