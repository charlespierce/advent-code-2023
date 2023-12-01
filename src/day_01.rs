use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next().unwrap();
            let last_digit = digits.next_back().unwrap_or(first_digit);

            first_digit * 10 + last_digit
        })
        .sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first_digit = match_first_digit(line);
            let last_digit = match_last_digit(line);

            first_digit * 10 + last_digit
        })
        .sum()
}

fn match_first_digit(line: &str) -> usize {
    let mut current = line;

    while !current.is_empty() {
        if let Some(value) = match_digit(current) {
            return value;
        }
        current = &current[1..];
    }

    unreachable!("No digits found!");
}

fn match_last_digit(line: &str) -> usize {
    let mut current = line;
    while !current.is_empty() {
        if let Some(value) = match_digit_back(current) {
            return value;
        }
        current = &current[..current.len() - 1];
    }

    unreachable!("No digits found!");
}

fn match_digit(line: &str) -> Option<usize> {
    for (digit, value) in DIGIT_VALUES {
        if line.starts_with(digit) {
            return Some(value);
        }
    }

    None
}

fn match_digit_back(line: &str) -> Option<usize> {
    for (digit, value) in DIGIT_VALUES {
        if line.ends_with(digit) {
            return Some(value);
        }
    }

    None
}

const DIGIT_VALUES: [(&str, usize); 20] = [
    ("zero", 0),
    ("0", 0),
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];
