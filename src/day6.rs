use crate::util::AdventHelper;
use rayon::prelude::*;

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

fn multiply_ways_to_win(races: &[Race]) -> usize {
    races.iter().map(ways_to_win).product()
}

fn ways_to_win(race: &Race) -> usize {
    (0..=race.time)
        .into_par_iter()
        .filter(|seconds_pressed| {
            let seconds_remaining = race.time - seconds_pressed;
            let distance = seconds_remaining * seconds_pressed;
            distance > race.distance
        })
        .count()
}

struct Race {
    time: i64,
    distance: i64,
}
