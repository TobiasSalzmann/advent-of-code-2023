use crate::util::Dir::{Down, Left, Right, Up};
use crate::util::{AdventHelper, Dir, Point};
use array2d::Array2D;
use pathfinding::prelude::dijkstra;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid = advent.parse_from_grid();
    advent.part1("Minimal Heat Loss:  {}", find_cheapest(&grid, 0, 3));
    advent.part2("Minimal Heat Loss:  {}", find_cheapest(&grid, 4, 10));
}

fn find_cheapest(grid: &Array2D<i32>, min_streak: i32, max_streak: i32) -> i32 {
    let start = State {
        point: Point::new(0, 0),
        streak: 0,
        orientation: None,
    };
    let target = Point::new(grid.row_len() - 1, grid.column_len() - 1);
    let (_, minimal_heat_loss) = dijkstra(
        &start,
        |current| successors(current, grid, min_streak, max_streak),
        |state| state.point == target,
    )
    .unwrap();
    minimal_heat_loss
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    point: Point,
    streak: i32,
    orientation: Option<Dir>,
}
fn successors(
    current: &State,
    grid: &Array2D<i32>,
    min_streak: i32,
    max_streak: i32,
) -> Vec<(State, i32)> {
    let State {
        point: p,
        streak,
        orientation,
    } = current;
    let mut successors = vec![];
    let can_turn = *streak >= min_streak;
    let can_stay = *streak < max_streak;
    for new_dir in [Up, Right, Down, Left] {
        let candidate @ Point { x: new_x, y: new_y } = p.mv(new_dir);
        let Some(cost) = grid.get(new_y as usize, new_x as usize) else {
            continue;
        };
        let is_pivot = Some(new_dir.pivot()) == *orientation;
        if is_pivot {
            continue;
        }

        let is_same_direction = Some(new_dir) == *orientation;
        let is_valid_stay = can_stay && is_same_direction;

        let is_valid_turn = can_turn && Some(new_dir) != *orientation;
        let new_streak = if is_valid_turn { 1 } else { streak + 1 };

        if is_valid_stay || is_valid_turn || orientation.is_none() {
            successors.push((
                State {
                    point: candidate,
                    streak: new_streak,
                    orientation: Some(new_dir),
                },
                *cost,
            ));
        }
    }

    successors
}
