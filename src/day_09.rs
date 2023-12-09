use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
fn solve_part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence: Vec<_> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            next_value(&sequence)
        })
        .sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence: Vec<_> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            previous_value(&sequence)
        })
        .sum()
}

fn next_value(sequence: &[isize]) -> isize {
    if sequence.len() <= 1 {
        return 0;
    }

    let mut diffs = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        diffs.push(sequence[i] - sequence[i - 1]);
    }

    let last = *sequence.last().unwrap();
    let diff = if diffs.iter().all(|n| *n == 0) {
        0
    } else {
        next_value(&diffs)
    };

    last + diff
}

fn previous_value(sequence: &[isize]) -> isize {
    if sequence.len() <= 1 {
        return 0;
    }

    let mut diffs = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        diffs.push(sequence[i] - sequence[i - 1]);
    }

    let first = sequence[0];
    let diff = if diffs.iter().all(|n| *n == 0) {
        0
    } else {
        previous_value(&diffs)
    };

    first - diff
}
