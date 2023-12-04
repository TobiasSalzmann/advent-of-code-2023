use crate::util::{AdventHelper};
use itertools::Itertools;

use std::collections::{HashMap};
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let cards: Vec<Card> = advent.parse_from_strings();

    advent.part1("Sum of scores: {}", sum_scores(&cards));
    advent.part2("Total cards: {}", count_scratchcards(&cards));
}

fn count_scratchcards(scratchcards: &Vec<Card>) -> usize {
    let mut number_of_cards: HashMap<u32, usize> =
        scratchcards.iter().map(|card| (card.id, 1)).collect();
    for card in scratchcards {
        let multiplicity = number_of_cards[&card.id];
        let count = card.count_winning();
        for id in card.id + 1..=card.id + count {
            number_of_cards.insert(id, number_of_cards[&id] + multiplicity);
        }
    }
    number_of_cards.values().sum()
}

fn sum_scores(cards: &Vec<Card>) -> u32 {
    cards.iter().map(|card| card.score()).sum()
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>,
}

impl Card {
    pub(crate) fn score(&self) -> u32 {
        let mut score = 0;
        for n in &self.actual_numbers {
            if self.winning_numbers.contains(&n) {
                if score == 0 {
                    score = 1
                } else {
                    score *= 2
                }
            }
        }
        score
    }

    pub(crate) fn count_winning(&self) -> u32 {
        let mut score = 0;
        for n in &self.actual_numbers {
            if self.winning_numbers.contains(&n) {
                score += 1
            }
        }
        score
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.replace("   ", " ").replace("  ", " ");
        let (raw_id, rest) = normalized
            .strip_prefix("Card ")
            .unwrap()
            .split(": ")
            .collect_tuple()
            .unwrap();
        let (raw_win, raw_act) = rest.split(" | ").collect_tuple().unwrap();
        let id = raw_id.parse().unwrap();
        let winning_numbers = raw_win.split(" ").map(|s| s.parse().unwrap()).collect_vec();
        let actual_numbers = raw_act.split(" ").map(|s| s.parse().unwrap()).collect_vec();
        Ok(Card {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
}
