use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    let mut schematic = Schematic::from(input);
    let mut sum = 0;

    // Row & Column count determined manually by inspecting input
    for row in 0..140 {
        for col in 0..140 {
            let point = Point::new(row as isize, col as isize);
            if schematic.is_symbol(point) {
                for neighbor in point.neighbors() {
                    if schematic.is_digit(neighbor) {
                        let (number, already_seen) = schematic.read_part_number(neighbor);
                        if !already_seen {
                            sum += number;
                        }
                    }
                }
            }
        }
    }

    sum
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    let mut schematic = Schematic::from(input);

    let mut sum = 0;

    // Row & Column count determined manually by inspecting input
    for row in 0..140 {
        for col in 0..140 {
            let point = Point::new(row as isize, col as isize);
            sum += schematic.read_gear(point);
        }
    }

    sum
}

struct Schematic {
    values: HashMap<Point, char>,
    seen_part_numbers: HashMap<Point, usize>,
}

impl From<&str> for Schematic {
    fn from(input: &str) -> Self {
        let values = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, chr)| (Point::new(row as isize, col as isize), chr))
            })
            .collect();
        Self {
            values,
            seen_part_numbers: HashMap::new(),
        }
    }
}

impl Schematic {
    fn is_symbol(&self, point: Point) -> bool {
        match self.values.get(&point) {
            Some(chr) => !chr.is_ascii_digit() && chr != &'.',
            None => false,
        }
    }

    fn is_digit(&self, point: Point) -> bool {
        match self.values.get(&point) {
            Some(chr) => chr.is_ascii_digit(),
            None => false,
        }
    }

    fn read_gear(&mut self, point: Point) -> usize {
        if self.values.get(&point) == Some(&'*') {
            let mut first = 0;
            let mut second = 0;

            for neighbor in point.neighbors() {
                if self.is_digit(neighbor) {
                    let (candidate, _) = self.read_part_number(neighbor);

                    if first == 0 {
                        first = candidate;
                    } else if second == 0 {
                        if first != candidate {
                            second = candidate;
                        }
                    } else if first != candidate && second != candidate {
                        return 0;
                    }
                }
            }

            first * second
        } else {
            0
        }
    }

    fn read_part_number(&mut self, start: Point) -> (usize, bool) {
        let mut current = start;
        while current.col > 0 {
            let test = current.left();
            if self.values.get(&test).unwrap().is_ascii_digit() {
                current = test;
            } else {
                break;
            }
        }

        let initial = current;
        let mut number = 0;
        while let Some(chr) = self.values.get(&current) {
            match chr.to_digit(10) {
                Some(digit) => {
                    number *= 10;
                    number += digit as usize;
                    current = current.right();
                }
                None => break,
            }
        }

        (
            number,
            self.seen_part_numbers.insert(initial, number).is_some(),
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn neighbors(self) -> impl Iterator<Item = Self> {
        [
            Point::new(self.row - 1, self.col - 1),
            Point::new(self.row - 1, self.col),
            Point::new(self.row - 1, self.col + 1),
            Point::new(self.row, self.col - 1),
            Point::new(self.row, self.col + 1),
            Point::new(self.row + 1, self.col - 1),
            Point::new(self.row + 1, self.col),
            Point::new(self.row + 1, self.col + 1),
        ]
        .into_iter()
    }

    fn left(self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
}
