use aoc_runner_derive::aoc;
use std::collections::{hash_map::Entry, HashMap};

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> usize {
    let mut platform = Platform::from(input);
    platform.tilt_up();
    platform.load_up()
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> usize {
    let mut platform = Platform::from(input);
    let mut cache = HashMap::new();
    cache.insert(platform.identity(), 0);

    let mut cycle = 0;
    while cycle < 1_000_000_000 {
        platform.cycle();
        cycle += 1;

        match cache.entry(platform.identity()) {
            Entry::Vacant(vac) => {
                vac.insert(cycle);
            }
            Entry::Occupied(occ) => {
                let previous = *occ.get();
                let period = cycle - previous;
                let diff = 1_000_000_000 - cycle;
                let remaining = diff % period;
                cycle = 1_000_000_000 - remaining;
            }
        }
    }

    platform.load_up()
}

#[derive(Clone, Copy)]
enum Space {
    Empty,
    Rounded,
    Cube,
}

impl Space {
    fn to_char(self) -> char {
        match self {
            Space::Empty => '.',
            Space::Rounded => 'O',
            Space::Cube => '#',
        }
    }
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty,
            'O' => Space::Rounded,
            '#' => Space::Cube,
            _ => unreachable!(),
        }
    }
}

struct Platform {
    map: Vec<Vec<Space>>,
}

impl Platform {
    fn get(&self, row: usize, col: usize) -> Space {
        self.map[row][col]
    }

    fn identity(&self) -> String {
        let mut result = String::with_capacity(self.map.len() * self.map[0].len());

        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                result.push(self.get(row, col).to_char());
            }
        }

        result
    }

    fn cycle(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }

    fn tilt_up(&mut self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if matches!(self.get(row, col), Space::Rounded) {
                    self.shift_up(row, col);
                }
            }
        }
    }

    fn tilt_left(&mut self) {
        for col in 0..self.map[0].len() {
            for row in 0..self.map.len() {
                if matches!(self.get(row, col), Space::Rounded) {
                    self.shift_left(row, col);
                }
            }
        }
    }

    fn tilt_down(&mut self) {
        for offset in 0..self.map.len() {
            let row = self.map.len() - 1 - offset;
            for col in 0..self.map[0].len() {
                if matches!(self.get(row, col), Space::Rounded) {
                    self.shift_down(row, col);
                }
            }
        }
    }

    fn tilt_right(&mut self) {
        let cols = self.map[0].len();
        for offset in 0..cols {
            let col = cols - 1 - offset;
            for row in 0..self.map.len() {
                if matches!(self.get(row, col), Space::Rounded) {
                    self.shift_right(row, col);
                }
            }
        }
    }

    fn shift_up(&mut self, start_row: usize, start_col: usize) {
        let mut current = start_row;

        while current > 0 {
            match self.get(current - 1, start_col) {
                Space::Cube | Space::Rounded => break,
                Space::Empty => {
                    current -= 1;
                }
            }
        }

        if current != start_row {
            self.map[current][start_col] = Space::Rounded;
            self.map[start_row][start_col] = Space::Empty;
        }
    }

    fn shift_left(&mut self, start_row: usize, start_col: usize) {
        let mut current = start_col;

        while current > 0 {
            match self.get(start_row, current - 1) {
                Space::Cube | Space::Rounded => break,
                Space::Empty => {
                    current -= 1;
                }
            }
        }

        if current != start_col {
            self.map[start_row][current] = Space::Rounded;
            self.map[start_row][start_col] = Space::Empty;
        }
    }

    fn shift_down(&mut self, start_row: usize, start_col: usize) {
        let mut current = start_row;

        while current < self.map.len() - 1 {
            match self.get(current + 1, start_col) {
                Space::Cube | Space::Rounded => break,
                Space::Empty => {
                    current += 1;
                }
            }
        }

        if current != start_row {
            self.map[current][start_col] = Space::Rounded;
            self.map[start_row][start_col] = Space::Empty;
        }
    }

    fn shift_right(&mut self, start_row: usize, start_col: usize) {
        let mut current = start_col;

        while current < self.map[0].len() - 1 {
            match self.get(start_row, current + 1) {
                Space::Cube | Space::Rounded => break,
                Space::Empty => {
                    current += 1;
                }
            }
        }

        if current != start_col {
            self.map[start_row][current] = Space::Rounded;
            self.map[start_row][start_col] = Space::Empty;
        }
    }

    fn load_up(&self) -> usize {
        let row_count = self.map.len();
        let mut total_load = 0;

        for row in 0..row_count {
            for col in 0..self.map[0].len() {
                if matches!(self.get(row, col), Space::Rounded) {
                    total_load += row_count - row;
                }
            }
        }

        total_load
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|line| line.chars().map(Space::from).collect())
            .collect();

        Self { map }
    }
}
