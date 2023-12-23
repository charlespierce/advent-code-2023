use aoc_runner_derive::aoc;
use indexmap::IndexSet;
use std::collections::{HashMap, VecDeque};

#[aoc(day23, part1)]
fn solve_part1(input: &str) -> usize {
    let map = Map::from_part1(input);
    map.longest_path()
}

#[aoc(day23, part2)]
fn solve_part2(input: &str) -> usize {
    let map = Map::from_part2(input);
    map.longest_path()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn shift(self, direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Point::new(self.row - 1, self.col),
            Down => Point::new(self.row + 1, self.col),
            Left => Point::new(self.row, self.col - 1),
            Right => Point::new(self.row, self.col + 1),
        }
    }
}

#[derive(Clone)]
struct Path {
    steps: IndexSet<Point>,
}

impl Path {
    fn new(point: Point) -> Self {
        let mut steps = IndexSet::new();
        steps.insert(point);

        Self { steps }
    }

    fn end(&self) -> Point {
        *self.steps.last().unwrap()
    }

    fn start(&self) -> Point {
        *self.steps.first().unwrap()
    }

    fn join(&self, point: Point) -> Option<Self> {
        if self.steps.contains(&point) {
            None
        } else {
            let mut copy = self.clone();
            copy.steps.insert(point);
            Some(copy)
        }
    }

    fn len(&self) -> usize {
        self.steps.len() - 1
    }
}

struct Map {
    spots: HashMap<Point, Tile>,
    rows: isize,
    cols: isize,
}

impl Map {
    fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut options = Vec::new();

        let up = point.shift(Direction::Up);
        if let Some(Tile::Open | Tile::SlopeUp) = self.spots.get(&up) {
            options.push(up);
        }

        let down = point.shift(Direction::Down);
        if let Some(Tile::Open | Tile::SlopeDown) = self.spots.get(&down) {
            options.push(down);
        }

        let left = point.shift(Direction::Left);
        if let Some(Tile::Open | Tile::SlopeLeft) = self.spots.get(&left) {
            options.push(left);
        }

        let right = point.shift(Direction::Right);
        if let Some(Tile::Open | Tile::SlopeRight) = self.spots.get(&right) {
            options.push(right);
        }

        options
    }

    fn possible_continuations(&self, path: &Path) -> Vec<Path> {
        let point = path.end();
        let mut results = Vec::new();

        for neighbor in self.neighbors(point) {
            if let Some(new_path) = path.join(neighbor) {
                results.push(new_path);
            }
        }

        results
    }

    fn start(&self) -> Point {
        for col in 0..self.cols {
            let point = Point::new(0, col);
            if let Some(Tile::Open) = self.spots.get(&point) {
                return point;
            }
        }

        unreachable!();
    }

    fn end(&self) -> Point {
        for col in 0..self.cols {
            let point = Point::new(self.rows - 1, col);
            if let Some(Tile::Open) = self.spots.get(&point) {
                return point;
            }
        }

        unreachable!();
    }

    fn create_mapping(&self, map_start: Point) -> Mapping {
        let mut queue = VecDeque::new();
        let mut edges: HashMap<_, HashMap<_, _>> = HashMap::new();
        queue.push_back(Path::new(map_start));

        while let Some(path) = queue.pop_front() {
            let mut possibles = self.possible_continuations(&path);

            if possibles.is_empty() {
                let len = edges
                    .entry(path.start())
                    .or_default()
                    .entry(path.end())
                    .or_default();
                if path.len() > *len {
                    *len = path.len();
                }
            } else if possibles.len() == 1 {
                queue.push_front(possibles.pop().unwrap());
            } else {
                let len = edges
                    .entry(path.start())
                    .or_default()
                    .entry(path.end())
                    .or_default();
                if path.len() > *len {
                    *len = path.len();
                }

                if !edges.contains_key(&path.end()) {
                    for possible in possibles {
                        let mut new_path = Path::new(path.end());
                        new_path.steps.insert(possible.end());
                        queue.push_back(new_path);
                    }
                }
            }
        }

        Mapping { edges }
    }

    fn longest_path(&self) -> usize {
        let start = self.start();
        let end = self.end();

        println!("Creating Mapping");
        let mapping = self.create_mapping(start);
        println!("Mapped");
        mapping.longest_path(start, end)
    }

    fn from_part1(value: &str) -> Self {
        let mut spots = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row, line) in value.lines().enumerate() {
            rows = rows.max(row + 1);
            for (col, chr) in line.chars().enumerate() {
                cols = cols.max(col + 1);
                let point = Point::new(row as isize, col as isize);
                spots.insert(point, Tile::from_part1(chr));
            }
        }

        Self {
            spots,
            rows: rows as isize,
            cols: cols as isize,
        }
    }

    fn from_part2(value: &str) -> Self {
        let mut spots = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row, line) in value.lines().enumerate() {
            rows = rows.max(row + 1);
            for (col, chr) in line.chars().enumerate() {
                cols = cols.max(col + 1);
                let point = Point::new(row as isize, col as isize);
                spots.insert(point, Tile::from_part2(chr));
            }
        }

        Self {
            spots,
            rows: rows as isize,
            cols: cols as isize,
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Open,
    Forest,
    SlopeDown,
    SlopeUp,
    SlopeLeft,
    SlopeRight,
}

impl Tile {
    fn from_part1(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Forest,
            'v' => Tile::SlopeDown,
            '<' => Tile::SlopeLeft,
            '>' => Tile::SlopeRight,
            '^' => Tile::SlopeUp,
            _ => unreachable!(),
        }
    }

    fn from_part2(value: char) -> Self {
        match value {
            '.' | 'v' | '<' | '>' | '^' => Tile::Open,
            '#' => Tile::Forest,
            _ => unreachable!(),
        }
    }
}

struct Mapping {
    edges: HashMap<Point, HashMap<Point, usize>>,
}

impl Mapping {
    fn longest_path(&self, map_start: Point, map_end: Point) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((0, IndexSet::from([map_start])));

        let mut longest = 0;

        while let Some((distance, path)) = queue.pop_front() {
            let next_start = path.last().unwrap();
            for (end, length) in self.edges.get(next_start).unwrap() {
                let next_length = distance + length;
                if *end == map_end {
                    longest = longest.max(next_length);
                } else if !path.contains(end) {
                    let mut new_path = path.clone();
                    new_path.insert(*end);
                    queue.push_back((next_length, new_path));
                }
            }
        }

        longest
    }
}
