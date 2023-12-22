use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.propagate_light(Point::new(0, 0), Direction::Right)
}

#[aoc(day16, part2)]
fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut max_energized = 0;

    for row in 0..grid.rows {
        let energized = grid.propagate_light(Point::new(row, 0), Direction::Right);
        max_energized = max_energized.max(energized);

        let energized = grid.propagate_light(Point::new(row, grid.cols - 1), Direction::Left);
        max_energized = max_energized.max(energized);
    }

    for col in 0..grid.cols {
        let energized = grid.propagate_light(Point::new(0, col), Direction::Down);
        max_energized = max_energized.max(energized);

        let energized = grid.propagate_light(Point::new(grid.rows - 1, col), Direction::Up);
        max_energized = max_energized.max(energized);
    }

    max_energized
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

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    DownMirror,
    UpMirror,
    UpDownSplitter,
    LeftRightSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '\\' => Tile::DownMirror,
            '/' => Tile::UpMirror,
            '|' => Tile::UpDownSplitter,
            '-' => Tile::LeftRightSplitter,
            _ => unreachable!(),
        }
    }
}

struct Grid {
    tiles: HashMap<Point, Tile>,
    rows: usize,
    cols: usize,
}

impl Grid {
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

    fn propagate_light(&self, start: Point, dir: Direction) -> usize {
        let mut visited: HashMap<Point, HashSet<Direction>> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, dir));

        macro_rules! maybe_queue {
            ($point:expr, $dir:expr) => {
                if let Some(point) = self.shift($point, $dir) {
                    let seen = visited.get(&point).is_some_and(|dirs| dirs.contains(&$dir));

                    if !seen {
                        queue.push_back((point, $dir));
                    }
                }
            };
        }

        while let Some((point, direction)) = queue.pop_front() {
            visited.entry(point).or_default().insert(direction);
            let tile = self.tiles.get(&point).unwrap();

            match (tile, direction) {
                (Tile::Empty, _) => maybe_queue!(point, direction),
                (Tile::DownMirror, Direction::Right) => maybe_queue!(point, Direction::Down),
                (Tile::DownMirror, Direction::Left) => maybe_queue!(point, Direction::Up),
                (Tile::DownMirror, Direction::Up) => maybe_queue!(point, Direction::Left),
                (Tile::DownMirror, Direction::Down) => maybe_queue!(point, Direction::Right),
                (Tile::UpMirror, Direction::Right) => maybe_queue!(point, Direction::Up),
                (Tile::UpMirror, Direction::Left) => maybe_queue!(point, Direction::Down),
                (Tile::UpMirror, Direction::Up) => maybe_queue!(point, Direction::Right),
                (Tile::UpMirror, Direction::Down) => maybe_queue!(point, Direction::Left),
                (Tile::UpDownSplitter, Direction::Down) => maybe_queue!(point, Direction::Down),
                (Tile::UpDownSplitter, Direction::Up) => maybe_queue!(point, Direction::Up),
                (Tile::UpDownSplitter, Direction::Left | Direction::Right) => {
                    maybe_queue!(point, Direction::Up);
                    maybe_queue!(point, Direction::Down);
                }
                (Tile::LeftRightSplitter, Direction::Left) => maybe_queue!(point, Direction::Left),
                (Tile::LeftRightSplitter, Direction::Right) => {
                    maybe_queue!(point, Direction::Right)
                }
                (Tile::LeftRightSplitter, Direction::Up | Direction::Down) => {
                    maybe_queue!(point, Direction::Left);
                    maybe_queue!(point, Direction::Right);
                }
            }
        }

        visited.len()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut tiles = HashMap::new();

        for (row, line) in value.lines().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                tiles.insert(Point { row, col }, chr.into());
                cols = cols.max(col + 1);
            }
            rows = rows.max(row + 1);
        }

        Grid { tiles, rows, cols }
    }
}

#[test]
fn test_part1() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    assert_eq!(46, solve_part1(input));
}
