use crate::dijkstra::{Dijkstra, Value};
use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    let grid = Grid::from(input);

    let (_, cost) = Dijkstra::new(
        Position {
            point: Point::new(0, 0),
            direction: Direction::Left,
            count: 0,
        },
        |position| grid.destination(position.point),
        |position| grid.neighbors_part1(*position),
    )
    .next()
    .unwrap();

    cost
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);

    let (_, cost) = Dijkstra::new(
        Position {
            point: Point::new(0, 0),
            direction: Direction::Left,
            count: 0,
        },
        |position| grid.destination(position.point),
        |position| grid.neighbors_part2(*position),
    )
    .next()
    .unwrap();

    cost
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    point: Point,
    direction: Direction,
    count: u8,
}

impl Value for Position {
    type Id = Self;

    fn id(&self) -> Self::Id {
        *self
    }
}

struct Grid {
    rows: usize,
    cols: usize,
    costs: HashMap<Point, usize>,
}

impl Grid {
    fn destination(&self, point: Point) -> bool {
        point.row == self.rows - 1 && point.col == self.cols - 1
    }

    fn neighbors_part1(&self, position: Position) -> Vec<(Position, usize)> {
        let mut options = Vec::with_capacity(3);

        match (position.direction, position.count) {
            (Direction::Up, count @ 1..=2) => {
                options.extend(self.generate_neighbor(position.point, Direction::Up, count + 1));
                options.extend(self.generate_neighbor(position.point, Direction::Left, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Right, 1));
            }
            (Direction::Up, 3) => {
                options.extend(self.generate_neighbor(position.point, Direction::Left, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Right, 1));
            }
            (Direction::Down, count @ 0..=2) => {
                options.extend(self.generate_neighbor(position.point, Direction::Down, count + 1));
                options.extend(self.generate_neighbor(position.point, Direction::Left, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Right, 1));
            }
            (Direction::Down, 3) => {
                options.extend(self.generate_neighbor(position.point, Direction::Left, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Right, 1));
            }
            (Direction::Left, count @ 0..=2) => {
                options.extend(self.generate_neighbor(position.point, Direction::Left, count + 1));
                options.extend(self.generate_neighbor(position.point, Direction::Up, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Down, 1));
            }
            (Direction::Left, 3) => {
                options.extend(self.generate_neighbor(position.point, Direction::Up, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Down, 1));
            }
            (Direction::Right, count @ 0..=2) => {
                options.extend(self.generate_neighbor(position.point, Direction::Right, count + 1));
                options.extend(self.generate_neighbor(position.point, Direction::Up, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Down, 1));
            }
            (Direction::Right, 3) => {
                options.extend(self.generate_neighbor(position.point, Direction::Up, 1));
                options.extend(self.generate_neighbor(position.point, Direction::Down, 1));
            }
            _ => unreachable!(),
        }

        options
    }

    fn neighbors_part2(&self, position: Position) -> Vec<(Position, usize)> {
        let mut options = Vec::with_capacity(3);
        let point = position.point;
        match (position.direction, position.count) {
            (_, 0) => {
                options.extend(self.generate_neighbor(point, Direction::Up, 1));
                options.extend(self.generate_neighbor(point, Direction::Down, 1));
                options.extend(self.generate_neighbor(point, Direction::Left, 1));
                options.extend(self.generate_neighbor(point, Direction::Right, 1));
            }
            (Direction::Up, count @ 1..=3) => {
                options.extend(self.generate_neighbor(point, Direction::Up, count + 1));
            }
            (Direction::Up, count @ 4..=9) => {
                options.extend(self.generate_neighbor(point, Direction::Up, count + 1));
                options.extend(self.generate_neighbor(point, Direction::Left, 1));
                options.extend(self.generate_neighbor(point, Direction::Right, 1));
            }
            (Direction::Up, 10) => {
                options.extend(self.generate_neighbor(point, Direction::Left, 1));
                options.extend(self.generate_neighbor(point, Direction::Right, 1));
            }
            (Direction::Down, count @ 1..=3) => {
                options.extend(self.generate_neighbor(point, Direction::Down, count + 1));
            }
            (Direction::Down, count @ 4..=9) => {
                options.extend(self.generate_neighbor(point, Direction::Down, count + 1));
                options.extend(self.generate_neighbor(point, Direction::Left, 1));
                options.extend(self.generate_neighbor(point, Direction::Right, 1));
            }
            (Direction::Down, 10) => {
                options.extend(self.generate_neighbor(point, Direction::Left, 1));
                options.extend(self.generate_neighbor(point, Direction::Right, 1));
            }
            (Direction::Left, count @ 1..=3) => {
                options.extend(self.generate_neighbor(point, Direction::Left, count + 1));
            }
            (Direction::Left, count @ 4..=9) => {
                options.extend(self.generate_neighbor(point, Direction::Left, count + 1));
                options.extend(self.generate_neighbor(point, Direction::Up, 1));
                options.extend(self.generate_neighbor(point, Direction::Down, 1));
            }
            (Direction::Left, 10) => {
                options.extend(self.generate_neighbor(point, Direction::Up, 1));
                options.extend(self.generate_neighbor(point, Direction::Down, 1));
            }
            (Direction::Right, count @ 1..=3) => {
                options.extend(self.generate_neighbor(point, Direction::Right, count + 1));
            }
            (Direction::Right, count @ 4..=9) => {
                options.extend(self.generate_neighbor(point, Direction::Right, count + 1));
                options.extend(self.generate_neighbor(point, Direction::Up, 1));
                options.extend(self.generate_neighbor(point, Direction::Down, 1));
            }
            (Direction::Right, 10) => {
                options.extend(self.generate_neighbor(point, Direction::Up, 1));
                options.extend(self.generate_neighbor(point, Direction::Down, 1));
            }
            _ => unreachable!(),
        }

        options
    }

    fn generate_neighbor(
        &self,
        point: Point,
        direction: Direction,
        count: u8,
    ) -> Option<(Position, usize)> {
        if let Some(point) = self.shift(point, direction) {
            let cost = *self.costs.get(&point).unwrap();
            Some((
                Position {
                    point,
                    direction,
                    count,
                },
                cost,
            ))
        } else {
            None
        }
    }

    fn shift(&self, point: Point, direction: Direction) -> Option<Point> {
        match direction {
            Direction::Up => (point.row > 0).then(|| Point::new(point.row - 1, point.col)),
            Direction::Down => {
                (point.row < self.rows - 1).then(|| Point::new(point.row + 1, point.col))
            }
            Direction::Left => (point.col > 0).then(|| Point::new(point.row, point.col - 1)),
            Direction::Right => {
                (point.col < self.cols - 1).then(|| Point::new(point.row, point.col + 1))
            }
        }
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut costs = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row, line) in value.lines().enumerate() {
            for (col, cost_chr) in line.chars().enumerate() {
                costs.insert(
                    Point::new(row, col),
                    cost_chr.to_digit(10).unwrap() as usize,
                );
                cols = cols.max(col + 1);
            }
            rows = rows.max(row + 1);
        }

        Self { rows, cols, costs }
    }
}
