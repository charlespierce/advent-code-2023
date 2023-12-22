use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_row)
        .map(|(row, expected)| row.count_matches(&expected))
        .sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_row)
        .map(|(row, expected)| {
            let mut more_expected = Vec::with_capacity(5 * expected.len());
            let mut more_springs = Vec::with_capacity(4 + 5 * row.springs.len());

            for i in 0..5 {
                more_expected.extend(expected.iter().copied());
                if i > 0 {
                    more_springs.push(Spring::Unknown);
                }
                more_springs.extend(row.springs.iter().copied());
            }

            (
                Row {
                    springs: more_springs,
                },
                more_expected,
            )
        })
        .map(|(row, expected)| row.count_matches(&expected))
        .sum()
}

fn parse_row(line: &str) -> (Row, Vec<usize>) {
    let (row_str, count_str) = line.split_once(' ').unwrap();
    let springs = row_str.chars().map(Spring::from).collect();
    let counts = count_str.split(',').map(|n| n.parse().unwrap()).collect();

    (Row { springs }, counts)
}

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Working,
            '#' => Spring::Broken,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }
}

struct Row {
    springs: Vec<Spring>,
}

impl Row {
    fn count_matches(&self, expected: &[usize]) -> usize {
        let mut cache = HashMap::new();
        count_matches_internal(&self.springs, expected, &mut cache)
    }
}

/// count_matches_internal will always be called with the springs right _after_
/// a possible contiguous broken block that matches a value in broken_sets, including
/// the working (or unknown) spring as a separator
///
/// We start at the beginning of the remaining section and take possible offsets to match
/// the _next_ set of spring offsets before recursing
fn count_matches_internal(
    springs: &[Spring],
    broken_sets: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(cached) = cache.get(&(springs.len(), broken_sets.len())) {
        return *cached;
    }

    let Some((&next_broken_set, remaining_sets)) = broken_sets.split_first() else {
        // There aren't any more broken sets expected, so we can directly determine if the
        // springs match all working
        let count = if springs.contains(&Spring::Broken) {
            0
        } else {
            // If there aren't any broken springs, then there's only 1 possible combination
            1
        };

        cache.insert((springs.len(), broken_sets.len()), count);

        return count;
    };

    // If the current broken set is longer than the remaining springs, it can't be a match
    let max_offset = if springs.len() >= next_broken_set {
        springs.len() - next_broken_set
    } else {
        cache.insert((springs.len(), broken_sets.len()), 0);
        return 0;
    };

    let mut count = 0;

    for offset in 0..=max_offset {
        // If we've skipped over any broken springs, we have gone too far and so are done trying
        // to match
        if springs[..offset].contains(&Spring::Broken) {
            break;
        }

        // If the current group starting at the offset contains any working springs, then it
        // isn't a match so we can skip to the next offset
        if springs[offset..offset + next_broken_set].contains(&Spring::Working) {
            continue;
        }

        // Now we have a possible match for the next broken set, so we need to recursively count
        // the possible combinations. There are two possible cases: We are matching the last
        // expected set of broken springs or not.
        if remaining_sets.is_empty() {
            // If we are matching the last set of broken springs, we don't need to worry about any
            // separator elements, so we can skip the current set and match on the remaining.
            count +=
                count_matches_internal(&springs[offset + next_broken_set..], remaining_sets, cache);
        } else {
            // We aren't matching the _last_ set of broken springs, so we need to consider the
            // next element _after_ the broken set to make sure it is a separator. If it doesn't
            // exist, then we can't possibly match so we can stop checking offsets. If it is
            // _also_ a broken spring, then we don't have a separator so this is not a match
            match springs.get(offset + next_broken_set) {
                None => break,
                Some(Spring::Broken) => continue,
                Some(_) => {
                    count += count_matches_internal(
                        &springs[offset + next_broken_set + 1..],
                        remaining_sets,
                        cache,
                    );
                }
            }
        }
    }

    cache.insert((springs.len(), broken_sets.len()), count);
    count
}

#[test]
fn test_part2() {
    assert_eq!(
        525152,
        solve_part2(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
        )
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        21,
        solve_part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
        )
    );
}
