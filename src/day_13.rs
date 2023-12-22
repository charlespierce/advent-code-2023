use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> usize {
    parse_patterns(input)
        .into_iter()
        .map(|pattern| match pattern.check_row_reflection(0) {
            Some(above) => 100 * above,
            None => pattern.check_column_reflection(0).unwrap_or(0),
        })
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> usize {
    parse_patterns(input)
        .into_iter()
        .map(|pattern| match pattern.check_row_reflection(1) {
            Some(above) => 100 * above,
            None => pattern.check_column_reflection(1).unwrap_or(0),
        })
        .sum()
}

struct Pattern {
    map: Vec<Vec<char>>,
}

impl Pattern {
    fn check_column_reflection(&self, num_wrong: usize) -> Option<usize> {
        let cols = self.map[0].len();
        'gaps: for gap in 0..cols - 1 {
            let mut wrong_count = 0;

            'cols: for left in 0..=gap {
                let right = 2 * gap - left + 1;
                if right >= cols {
                    continue 'cols;
                }

                for row in 0..self.map.len() {
                    if self.map[row][left] != self.map[row][right] {
                        wrong_count += 1;
                        if wrong_count > num_wrong {
                            continue 'gaps;
                        }
                    }
                }
            }

            if wrong_count == num_wrong {
                return Some(gap + 1);
            }
        }

        None
    }

    fn check_row_reflection(&self, num_wrong: usize) -> Option<usize> {
        let rows = self.map.len();

        'gaps: for gap in 0..rows - 1 {
            let mut wrong_count = 0;

            'rows: for above in 0..=gap {
                let below = 2 * gap - above + 1;
                if below >= rows {
                    continue 'rows;
                }

                for col in 0..self.map[0].len() {
                    if self.map[above][col] != self.map[below][col] {
                        wrong_count += 1;
                        if wrong_count > num_wrong {
                            continue 'gaps;
                        }
                    }
                }
            }

            if wrong_count == num_wrong {
                return Some(gap + 1);
            }
        }

        None
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let map = value.lines().map(|line| line.chars().collect()).collect();

        Self { map }
    }
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::from).collect()
}
