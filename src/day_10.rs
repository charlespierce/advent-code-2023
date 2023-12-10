use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> usize {
    let (zone, start) = parse_zone(input);
    // Direction determined from manual inspection of input
    let points = zone.traverse_loop(start, Direction::Down);

    points.len() / 2
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> usize {
    let (zone, start) = parse_zone(input);
    // Direction determined from manual inspection of input
    let points = zone.traverse_loop(start, Direction::Down);
    let mut contained_ground = 0;

    for row in 0..zone.rows {
        let mut parity = LoopParity::Out;
        for col in 0..zone.cols {
            let point = Point::new(row, col);
            let tile = zone.tile(point);

            if points.contains(&point) {
                // This tile is part of the loop and so can affect parity
                parity = parity.next(tile);
            } else if parity.is_in() {
                // This tile is not part of the loop and we are currently inside
                contained_ground += 1;
            }
        }
    }

    contained_ground
}

struct Zone {
    tiles: Vec<Tile>,
    cols: usize,
    rows: usize,
}

impl Zone {
    fn tile(&self, point: Point) -> Tile {
        let index = point.row * self.cols + point.col;
        self.tiles[index]
    }

    fn traverse_loop(&self, start: Point, direction: Direction) -> HashSet<Point> {
        let mut current = start;
        let mut dir = direction;
        let mut loop_points = HashSet::new();

        loop {
            loop_points.insert(current);

            current = current.shift(dir);
            let next_tile = self.tile(current);
            dir = match next_tile.next_direction(dir) {
                Some(d) => d,
                None => {
                    break;
                }
            };
        }

        loop_points
    }
}

fn parse_zone(input: &str) -> (Zone, Point) {
    let mut start = Point::new(0, 0);
    let mut cols = 0;
    let mut rows = 0;
    let mut tiles = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            let tile = chr.into();

            if matches!(tile, Tile::Start) {
                start = Point::new(row, col);
            }

            tiles.push(tile);

            cols = cols.max(col + 1);
        }
        rows = rows.max(row + 1);
    }

    (Zone { tiles, cols, rows }, start)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }

    fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point::new(self.row - 1, self.col),
            Direction::Down => Point::new(self.row + 1, self.col),
            Direction::Left => Point::new(self.row, self.col - 1),
            Direction::Right => Point::new(self.row, self.col + 1),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy)]
enum Tile {
    Start,
    Ground,
    UpDown,
    UpRight,
    UpLeft,
    LeftRight,
    DownRight,
    DownLeft,
}

impl Tile {
    fn next_direction(self, enter_move: Direction) -> Option<Direction> {
        match (enter_move, self) {
            (Direction::Left, Tile::UpRight) => Some(Direction::Up),
            (Direction::Left, Tile::LeftRight) => Some(Direction::Left),
            (Direction::Left, Tile::DownRight) => Some(Direction::Down),
            (Direction::Right, Tile::UpLeft) => Some(Direction::Up),
            (Direction::Right, Tile::LeftRight) => Some(Direction::Right),
            (Direction::Right, Tile::DownLeft) => Some(Direction::Down),
            (Direction::Up, Tile::UpDown) => Some(Direction::Up),
            (Direction::Up, Tile::DownRight) => Some(Direction::Right),
            (Direction::Up, Tile::DownLeft) => Some(Direction::Left),
            (Direction::Down, Tile::UpDown) => Some(Direction::Down),
            (Direction::Down, Tile::UpRight) => Some(Direction::Right),
            (Direction::Down, Tile::UpLeft) => Some(Direction::Left),
            _ => None,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            '.' => Tile::Ground,
            '|' => Tile::UpDown,
            'L' => Tile::UpRight,
            'J' => Tile::UpLeft,
            '-' => Tile::LeftRight,
            'F' => Tile::DownRight,
            '7' => Tile::DownLeft,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum LoopParity {
    Out,
    OutUp,
    OutDown,
    In,
    InUp,
    InDown,
}

impl LoopParity {
    fn is_in(self) -> bool {
        matches!(self, LoopParity::In | LoopParity::InUp | LoopParity::InDown)
    }

    fn next(self, tile: Tile) -> Self {
        use LoopParity::*;
        // By direct inspection, the Start tile is DownLeft
        match (self, tile) {
            // The Ground and LeftRight tiles do not change parity
            (val, Tile::Ground | Tile::LeftRight) => val,
            (Out, Tile::UpDown) => In,
            (Out, Tile::UpRight | Tile::UpLeft) => OutUp,
            (Out, Tile::DownRight | Tile::DownLeft | Tile::Start) => OutDown,
            (OutUp, Tile::UpDown) => unreachable!(),
            (OutUp, Tile::UpRight | Tile::UpLeft) => Out,
            (OutUp, Tile::DownRight | Tile::DownLeft | Tile::Start) => In,
            (OutDown, Tile::UpDown) => unreachable!(),
            (OutDown, Tile::UpRight | Tile::UpLeft) => In,
            (OutDown, Tile::DownRight | Tile::DownLeft | Tile::Start) => Out,
            (In, Tile::UpDown) => Out,
            (In, Tile::UpRight | Tile::UpLeft) => InUp,
            (In, Tile::DownRight | Tile::DownLeft | Tile::Start) => InDown,
            (InUp, Tile::UpDown) => unreachable!(),
            (InUp, Tile::UpRight | Tile::UpLeft) => In,
            (InUp, Tile::DownRight | Tile::DownLeft | Tile::Start) => Out,
            (InDown, Tile::UpDown) => unreachable!(),
            (InDown, Tile::UpRight | Tile::UpLeft) => Out,
            (InDown, Tile::DownRight | Tile::DownLeft | Tile::Start) => In,
        }
    }
}
