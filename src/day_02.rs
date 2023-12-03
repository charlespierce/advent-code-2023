use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{u16, u8},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, pair},
    IResult,
};
use std::ops::Add;

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> u16 {
    input
        .lines()
        .filter_map(|line| {
            let game = parse_game(line);

            game.possible_part_1().then_some(game.id)
        })
        .sum()
}

struct Game {
    id: u16,
    pulls: Vec<Pull>,
}

impl Game {
    fn possible_part_1(&self) -> bool {
        const MAX_RED: u8 = 12;
        const MAX_GREEN: u8 = 13;
        const MAX_BLUE: u8 = 14;

        let max = self.pulls.iter().copied().fold(Pull::default(), Pull::max);

        max.red <= MAX_RED && max.green <= MAX_GREEN && max.blue <= MAX_BLUE
    }

    fn minimum_possible_pull(self) -> Pull {
        self.pulls.into_iter().fold(Pull::default(), Pull::max)
    }
}

fn parse_game(input: &str) -> Game {
    map(
        all_consuming(pair(parse_game_id, parse_pull_list)),
        |(id, pulls)| Game { id, pulls },
    )(input)
    .unwrap()
    .1
}

fn parse_game_id(input: &str) -> IResult<&str, u16> {
    delimited(tag("Game "), u16, tag(": "))(input)
}

fn parse_pull_list(input: &str) -> IResult<&str, Vec<Pull>> {
    separated_list1(tag("; "), parse_pull)(input)
}

fn parse_pull(input: &str) -> IResult<&str, Pull> {
    map(
        separated_list1(tag(", "), alt((parse_red, parse_green, parse_blue))),
        |list| list.into_iter().fold(Pull::default(), Pull::add),
    )(input)
}

fn parse_red(input: &str) -> IResult<&str, Pull> {
    map(pair(u8, tag(" red")), |(count, _)| Pull::new_red(count))(input)
}

fn parse_green(input: &str) -> IResult<&str, Pull> {
    map(pair(u8, tag(" green")), |(count, _)| Pull::new_green(count))(input)
}

fn parse_blue(input: &str) -> IResult<&str, Pull> {
    map(pair(u8, tag(" blue")), |(count, _)| Pull::new_blue(count))(input)
}

#[derive(Clone, Copy, Default)]
struct Pull {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pull {
    fn new_red(count: u8) -> Self {
        Self {
            red: count,
            green: 0,
            blue: 0,
        }
    }

    fn new_green(count: u8) -> Self {
        Self {
            red: 0,
            green: count,
            blue: 0,
        }
    }

    fn new_blue(count: u8) -> Self {
        Self {
            red: 0,
            green: 0,
            blue: count,
        }
    }

    fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(self) -> usize {
        self.red as usize * self.green as usize * self.blue as usize
    }
}

impl Add for Pull {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Pull {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game = parse_game(line);
            game.minimum_possible_pull().power()
        })
        .sum()
}
