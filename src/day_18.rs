use aoc_runner_derive::aoc;

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> isize {
    let mut plot = Plot::default();
    plot.dig(input.lines().map(DigInstruction::from_part1));
    plot.count_space()
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> isize {
    let mut plot = Plot::default();
    plot.dig(input.lines().map(DigInstruction::from_part2));
    plot.count_space()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" | "3" => Direction::Up,
            "D" | "1" => Direction::Down,
            "L" | "2" => Direction::Left,
            "R" | "0" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

struct DigInstruction {
    direction: Direction,
    distance: isize,
}

impl DigInstruction {
    fn from_part1(value: &str) -> Self {
        let mut parts = value.split(' ');
        let direction = parts.next().unwrap().into();
        let distance = parts.next().unwrap().parse().unwrap();

        Self {
            direction,
            distance,
        }
    }

    fn from_part2(value: &str) -> Self {
        let hex = value.split(' ').nth(2).unwrap();
        let distance = isize::from_str_radix(&hex[2..7], 16).unwrap();
        let direction = Direction::from(&hex[7..8]);

        Self {
            direction,
            distance,
        }
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

    fn shift(self, direction: Direction, distance: isize) -> Self {
        match direction {
            Direction::Up => Point::new(self.row - distance, self.col),
            Direction::Down => Point::new(self.row + distance, self.col),
            Direction::Left => Point::new(self.row, self.col - distance),
            Direction::Right => Point::new(self.row, self.col + distance),
        }
    }
}

#[derive(Default)]
struct Plot {
    vertices: Vec<Point>,
    perimeter: isize,
}

impl Plot {
    fn dig<I>(&mut self, instructions: I)
    where
        I: IntoIterator<Item = DigInstruction>,
    {
        let mut current = Point::new(0, 0);
        self.vertices.push(current);

        for instruction in instructions {
            current = current.shift(instruction.direction, instruction.distance);
            self.vertices.push(current);
            self.perimeter += instruction.distance;
        }
    }

    fn count_space(&self) -> isize {
        let mut pair_sums = 0;

        for i in 1..self.vertices.len() {
            let first = self.vertices[i - 1];
            let second = self.vertices[i];

            pair_sums += first.row * second.col - first.col * second.row;
        }

        ((pair_sums.abs() + self.perimeter) / 2) + 1
    }
}
