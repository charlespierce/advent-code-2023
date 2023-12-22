use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

#[aoc(day21, part1)]
fn solve_part1(input: &str) -> usize {
    let map = Map::from(input);
    map.available_spaces_part1(64)
}

#[aoc(day21, part2)]
fn solve_part2(input: &str) -> usize {
    // From direct inspection, the input has size 131x131, with the start at the exact center
    // The requested number of steps is also of the form `131x + 65`. Some visual / geometric
    // analysis shows that the solution for the number of possible plots given those symmetries
    // is quadratic in `x`. A generic quadratic has 3 unknown coefficients, so we can use the
    // solution from part 1 to solve for 3 distinct values of `x` and calculate the coefficients.

    // n(steps) = y(x) = number of accessible plots given the number of steps
    // y(x) = ax^2 + bx + c
    // y(0) = c
    // y(1) = a + b + c => b = y(1) - y(0) - a
    // y(2) = 4a + 2b + c
    // => y(2) = 4a + 2y(1) - 2y(0) - 2a + c
    //         = 2a + 2y(1) - y(0)
    // =>   2a = y(2) - 2y(1) + y(0)
    // =>    a = (1/2)(y(2) - 2y(1) + y(0))

    let map = Map::from(input);

    let y0 = map.available_spaces_part2(65);
    let y1 = map.available_spaces_part2(196);
    let y2 = map.available_spaces_part2(327);

    let a = (y2 - 2 * y1 + y0) / 2;
    let b = y1 - y0 - a;
    let c = y0;

    // The requested number of steps is 26_501_365, which is given by:
    // 131(202_300) + 65 => x = 202_300

    let x = 202_300;

    a * x * x + b * x + c
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
enum Space {
    Rock,
    Plot,
    Start,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Plot,
            '#' => Space::Rock,
            'S' => Space::Start,
            _ => unreachable!(),
        }
    }
}

struct Map {
    spaces: HashMap<Point, Space>,
    rows: isize,
    cols: isize,
}

impl Map {
    fn available_spaces_part1(&self, steps: usize) -> usize {
        self.available_spaces_internal(steps, |point| {
            matches!(self.spaces.get(&point), Some(Space::Plot | Space::Start))
        })
    }

    fn available_spaces_part2(&self, steps: usize) -> usize {
        self.available_spaces_internal(steps, |point| {
            let adjusted_point = Point::new(
                point.row.rem_euclid(self.rows),
                point.col.rem_euclid(self.cols),
            );
            matches!(
                self.spaces.get(&adjusted_point),
                Some(Space::Plot | Space::Start)
            )
        })
    }

    fn available_spaces_internal<F>(&self, steps: usize, is_plot: F) -> usize
    where
        F: Fn(Point) -> bool,
    {
        let mut plots = HashSet::new();

        let (point, _) = self
            .spaces
            .iter()
            .find(|(_, space)| matches!(space, Space::Start))
            .unwrap();

        plots.insert(*point);

        for _ in 0..steps {
            let mut new_plots = HashSet::new();

            for position in plots {
                let up = position.shift(Direction::Up);
                if is_plot(up) {
                    new_plots.insert(up);
                }

                let down = position.shift(Direction::Down);
                if is_plot(down) {
                    new_plots.insert(down);
                }

                let left = position.shift(Direction::Left);
                if is_plot(left) {
                    new_plots.insert(left);
                }

                let right = position.shift(Direction::Right);
                if is_plot(right) {
                    new_plots.insert(right);
                }
            }

            plots = new_plots;
        }

        plots.len()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut spaces = HashMap::new();

        for (row, line) in value.lines().enumerate() {
            rows = rows.max(row + 1);
            for (col, chr) in line.chars().enumerate() {
                cols = cols.max(col + 1);
                spaces.insert(Point::new(row as isize, col as isize), chr.into());
            }
        }

        Self {
            spaces,
            rows: rows as isize,
            cols: cols as isize,
        }
    }
}
