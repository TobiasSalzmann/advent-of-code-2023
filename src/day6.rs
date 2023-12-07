use crate::util::AdventHelper;
use itertools::Itertools;
use std::cmp::{max, min};

use std::ops::Range;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let races = vec![
        Race {
            time: 46,
            distance: 214,
        },
        Race {
            time: 80,
            distance: 1177,
        },
        Race {
            time: 78,
            distance: 1402,
        },
        Race {
            time: 66,
            distance: 1024,
        },
    ];

    advent.part1("product: {}", multiply_ways_to_win(&races));

    let race = Race {
        time: 46807866,
        distance: 214117714021024,
    };
    advent.part2("ways to win: {}", ways_to_win(&race));
}

fn multiply_ways_to_win(races: &Vec<Race>) -> i64 {
    races.iter().map(|race| ways_to_win(race)).product()
}

fn ways_to_win(race: &Race) -> i64 {
    let mut ways = 0;
    for seconds_pressed in 0..=race.time {
        let seconds_remaining = race.time - seconds_pressed;
        let distance = seconds_remaining * seconds_pressed;
        if (distance > race.distance) {
            ways += 1
        }
    }
    ways
}

struct Race {
    time: i64,
    distance: i64,
}
