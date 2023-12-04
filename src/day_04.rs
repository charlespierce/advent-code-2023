use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|card| {
            let matches = match_count(card);
            if matches > 0 {
                2_usize.pow(matches as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    let mut cards = HashMap::new();

    for (index, matches) in input.lines().map(match_count).enumerate() {
        let id = index + 1;
        let card_count_ref = cards.entry(id).or_default();
        *card_count_ref += 1;
        let card_count: usize = *card_count_ref;

        for step in 1..=matches {
            *cards.entry(id + step).or_default() += card_count;
        }
    }

    cards.values().copied().sum()
}

fn match_count(card: &str) -> usize {
    let (_, card_str) = card.split_once(": ").unwrap();
    let (winning_str, have_str) = card_str.split_once(" | ").unwrap();
    let winning: HashSet<_> = winning_str
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    have_str
        .split_whitespace()
        .filter(|num| {
            let value = num.parse::<usize>().unwrap();
            winning.contains(&value)
        })
        .count()
}
