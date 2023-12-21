use crate::util::{AdventHelper, Point};
use array2d::Array2D;
use rustc_hash::FxHashSet;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let garden = Array2D::from_rows(&advent.parse_grid()).unwrap();

    advent.part1("Reachable plots: {}", reachable_plots(&garden, 64));
}

fn reachable_plots(garden: &Array2D<char>, steps: usize) -> usize {
    let ((y, x), _) = garden
        .enumerate_row_major()
        .find(|((y, x), c)| **c == 'S')
        .unwrap();
    let mut current = FxHashSet::from_iter([Point::new(x, y)]);
    for _ in 1..=steps {
        current = current
            .iter()
            .flat_map(|p| p.neighbours())
            .filter(|p| {
                let tile = garden.get(p.y as usize, p.x as usize);
                tile == Some(&'.') || tile == Some(&'S')
            })
            .collect();
    }

    current.len()
}
