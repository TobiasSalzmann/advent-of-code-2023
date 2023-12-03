
use crate::util::{AdventHelper};
use itertools::Itertools;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let games: Vec<Game> = advent.parse_from_strings();

    advent.part1(
        "Number of possible games: {}",
        sum_valid_games(games.clone(), 12, 13, 14),
    );
    advent.part2("Minimum Power: {}", sum_minimum_powers(games.clone()));
}

fn sum_minimum_powers(games: Vec<Game>) -> usize {
    games.iter().map(|game| game.minimum_power()).sum()
}

fn sum_valid_games(games: Vec<Game>, max_red: usize, max_green: usize, max_blue: usize) -> u32 {
    games
        .into_iter()
        .filter(|game| game.is_valid(max_red, max_green, max_blue))
        .map(|game| game.id)
        .sum()
}

#[derive(Debug, PartialEq, Clone)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        self.rounds
            .iter()
            .all(|round| max_red >= round.red && max_green >= round.green && max_blue >= round.blue)
    }

    fn minimum_power(&self) -> usize {
        let min_red = self.rounds.iter().map(|round| round.red).max().unwrap();
        let min_green = self.rounds.iter().map(|round| round.green).max().unwrap();
        let min_blue = self.rounds.iter().map(|round| round.blue).max().unwrap();

        min_red * min_green * min_blue
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_game, raw_rounds_str) = s.split(": ").collect_tuple().unwrap();
        let id = raw_game.strip_prefix("Game ").unwrap().parse().unwrap();
        let rounds = raw_rounds_str
            .split("; ")
            .map(|raw_round| raw_round.parse().unwrap())
            .collect_vec();
        Ok(Game { id, rounds })
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_colours = s.split(", ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for raw_colour in raw_colours {
            let (raw_num, colour) = raw_colour.split(" ").collect_tuple().unwrap();
            let num: usize = raw_num.parse().unwrap();
            match colour {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => panic!(),
            }
        }

        Ok(Round { red, green, blue })
    }
}
