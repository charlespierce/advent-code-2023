use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day8, part1)]
fn solve_part1(input: &str) -> usize {
    let (moves_str, network_str) = input.split_once("\n\n").unwrap();
    let moves = Moves::from(moves_str);
    let network = Network::from(network_str);

    network.run("AAA", |key| key == "ZZZ", moves)
}

#[aoc(day8, part2)]
fn solve_part2(input: &str) -> usize {
    let (moves_str, network_str) = input.split_once("\n\n").unwrap();
    let moves = Moves::from(moves_str);
    let network = Network::from(network_str);

    // Note: In the general case, we wouldn't be able to assume that the navigation is periodic
    // and doesn't have a prefix. However, from inspection of the results, each of the starting
    // positions _does_ turn out to have an exactly periodic cycle so we can simplify the math
    // of finding the end point to be finding the lowest common multiple of all of the periods
    network
        .nodes()
        .filter(|n| n.ends_with('A'))
        .map(|start| network.run(start, |key| key.ends_with('Z'), moves.clone()))
        .reduce(lowest_common_multiple)
        .unwrap()
}

fn lowest_common_multiple(a: usize, b: usize) -> usize {
    (a / greatest_common_divisor(a, b)) * b
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    loop {
        let r = a % b;
        if r == 0 {
            break b;
        }
        a = b;
        b = r;
    }
}

#[derive(Clone)]
struct Moves {
    list: Vec<Direction>,
    index: usize,
}

impl Moves {
    fn next(&mut self) -> Direction {
        let result = self.list[self.index];
        self.index = (self.index + 1) % self.list.len();

        result
    }
}

impl From<&str> for Moves {
    fn from(value: &str) -> Self {
        let list = value.chars().map(Direction::from).collect();

        Self { list, index: 0 }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

struct Edges<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Edges<'a> {
    fn from(value: &'a str) -> Self {
        let left = &value[1..4];
        let right = &value[6..9];

        Self { left, right }
    }
}

struct Network<'a> {
    edges: HashMap<&'a str, Edges<'a>>,
}

impl<'a> Network<'a> {
    fn run<P>(&self, start: &'a str, at_end: P, mut moves: Moves) -> usize
    where
        P: Fn(&'a str) -> bool,
    {
        let mut current = start;
        let mut count = 0;

        while !at_end(current) {
            current = match moves.next() {
                Direction::Left => self.edges[current].left,
                Direction::Right => self.edges[current].right,
            };
            count += 1;
        }

        count
    }

    fn nodes(&self) -> impl Iterator<Item = &str> {
        self.edges.keys().copied()
    }
}

impl<'a> From<&'a str> for Network<'a> {
    fn from(value: &'a str) -> Self {
        let edges = value
            .lines()
            .map(|line| {
                let (key, edge) = line.split_once(" = ").unwrap();
                (key, edge.into())
            })
            .collect();

        Self { edges }
    }
}
