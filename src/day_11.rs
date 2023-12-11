use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut universe = Universe::from(input);
    universe.expand(2);

    universe.shortest_paths()
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> usize {
    let mut universe = Universe::from(input);
    universe.expand(1_000_000);

    universe.shortest_paths()
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

    fn distance(self, other: Point) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

struct Universe {
    galaxies: HashSet<Point>,
    rows: usize,
    cols: usize,
}

impl Universe {
    fn has_galaxy(&self, row: usize, col: usize) -> bool {
        self.galaxies.contains(&Point::new(row, col))
    }

    fn expand(&mut self, factor: usize) {
        let mut expansion = 0;
        let mut col_map = HashMap::new();

        for col in 0..self.cols {
            col_map.insert(col, col + expansion);

            let is_empty = (0..self.rows).all(|row| !self.has_galaxy(row, col));

            if is_empty {
                expansion += factor - 1;
            }
        }

        let new_cols = self.cols + expansion;
        expansion = 0;
        let mut row_map = HashMap::new();

        for row in 0..self.rows {
            row_map.insert(row, row + expansion);

            let is_empty = (0..self.cols).all(|col| !self.has_galaxy(row, col));

            if is_empty {
                expansion += factor - 1;
            }
        }

        let new_rows = self.rows + expansion;

        let new_galaxies = self
            .galaxies
            .iter()
            .map(|point| Point::new(row_map[&point.row], col_map[&point.col]))
            .collect();

        self.galaxies = new_galaxies;
        self.rows = new_rows;
        self.cols = new_cols;
    }

    fn shortest_paths(&self) -> usize {
        let mut total = 0;

        for a in &self.galaxies {
            for b in &self.galaxies {
                total += a.distance(*b);
            }
        }

        // Note, this double counts all the pairs
        total / 2
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut galaxies = HashSet::new();

        for (row, line) in value.lines().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                if chr == '#' {
                    galaxies.insert(Point::new(row, col));
                }
                cols = cols.max(col + 1);
            }
            rows = rows.max(row + 1);
        }

        Universe {
            galaxies,
            rows,
            cols,
        }
    }
}
