use crate::util::AdventHelper;
use itertools::Itertools;

use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let plays: Vec<Play> = advent.parse_from_strings();

    advent.part1("winnings: {}", winnings(&plays, false));
    advent.part2("winnings: {}", winnings(&plays, true));
}

fn winnings(plays: &Vec<Play>, joker: bool) -> usize {
    plays
        .iter()
        .sorted_by_key(|p| p.hand.score(joker))
        .enumerate()
        .map(|(rank, play)| (rank + 1) * play.bid)
        .sum()
}

struct Hand(String);

impl Hand {
    fn score(&self, joker: bool) -> u64 {
        let values = self
            .0
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if joker {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => c.to_string().parse().unwrap(),
            })
            .collect_vec();
        let tie_breaker_score = 1_00_00_00_00 * values[0]
            + 1_00_00_00 * values[1]
            + 1_00_00 * values[2]
            + 1_00 * values[3]
            + values[4];

        let num_jokers = values.iter().filter(|v| **v == 1).count();

        let mut group_lengths: Vec<usize> = values
            .iter()
            .filter(|v| **v != 1)
            .sorted()
            .counts()
            .values()
            .sorted()
            .cloned()
            .collect_vec();

        if group_lengths.is_empty() {
            group_lengths = vec![5]
        } else {
            *group_lengths.last_mut().unwrap() += num_jokers;
        }

        let hand_score = match &group_lengths[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            [1, 1, 1, 1, 1] => 1,
            _ => panic!(),
        };
        1_00_00_00_00_00 * hand_score + tie_breaker_score
    }
}

struct Play {
    hand: Hand,
    bid: usize,
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split(" ").collect_tuple().unwrap();
        Ok(Play {
            hand: Hand(a.to_string()),
            bid: b.parse().unwrap(),
        })
    }
}
