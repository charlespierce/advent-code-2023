use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> usize {
    let (seeds_line, maps) = input.split_once("\n\n").unwrap();
    let almanac = Almanac::from(maps);

    seeds_line[7..]
        .split_ascii_whitespace()
        .map(|seed| almanac.location_for_seed(seed.parse().unwrap()))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> usize {
    let (seeds_line, maps) = input.split_once("\n\n").unwrap();
    let almanac = Almanac::from(maps);

    let mut min_location = usize::MAX;
    let mut seed_values = seeds_line[7..].split_ascii_whitespace();

    loop {
        let Some(start_str) = seed_values.next() else {
            break;
        };
        let start = start_str.parse().unwrap();
        let length = seed_values.next().unwrap().parse().unwrap();

        let possible_min = almanac.min_location_for_range(Range { start, length });

        min_location = min_location.min(possible_min);
    }

    min_location
}

#[derive(Debug)]
struct Range {
    start: usize,
    length: usize,
}

struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn location_for_seed(&self, seed: usize) -> usize {
        let mut current = seed;

        for map in &self.maps {
            current = map.lookup_single(current);
        }

        current
    }

    fn min_location_for_range(&self, seed_range: Range) -> usize {
        let location_ranges = self.maps.iter().fold(vec![seed_range], |ranges, map| {
            ranges
                .into_iter()
                .flat_map(|range| map.lookup_range(range))
                .collect()
        });

        location_ranges
            .into_iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        Self {
            maps: value.split("\n\n").map(Map::from).collect(),
        }
    }
}

struct Map {
    ranges: Vec<RangeMap>,
}

impl Map {
    fn lookup_single(&self, source_value: usize) -> usize {
        match self
            .ranges
            .iter()
            .filter_map(|range| range.lookup(source_value))
            .next()
        {
            Some(destination_value) => destination_value,
            None => source_value,
        }
    }

    fn lookup_range(&self, range: Range) -> Vec<Range> {
        let mut results = Vec::new();

        let mut current = range.start;
        let mut remaining = range.length;
        let mut map_index = 0;

        while remaining > 0 {
            let Some(range_map) = self.ranges.get(map_index) else {
                break;
            };
            map_index += 1;

            if range_map.source_start > current + remaining - 1 {
                break;
            }

            if range_map.source_start + range_map.length - 1 < current {
                continue;
            }

            if current < range_map.source_start {
                let length = range_map.source_start - current;
                results.push(Range {
                    start: current,
                    length,
                });
                current = range_map.source_start;
                remaining -= length;
            }

            let diff = current - range_map.source_start;
            let mapped_length = range_map.length - diff;

            if mapped_length > remaining {
                results.push(Range {
                    start: range_map.destination_start + diff,
                    length: remaining,
                });
                remaining = 0;
                break;
            } else {
                results.push(Range {
                    start: range_map.destination_start + diff,
                    length: mapped_length,
                });
                current += mapped_length;
                remaining -= mapped_length;
            }
        }

        if remaining > 0 {
            results.push(Range {
                start: current,
                length: remaining,
            });
        }

        results
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut ranges: Vec<_> = value.lines().skip(1).map(RangeMap::from).collect();
        ranges.sort_by_key(|r| r.source_start);

        Self { ranges }
    }
}

struct RangeMap {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl RangeMap {
    fn lookup(&self, source_value: usize) -> Option<usize> {
        let Some(diff) = source_value.checked_sub(self.source_start) else {
            return None;
        };

        if diff > self.length - 1 {
            return None;
        }

        Some(self.destination_start + diff)
    }
}

impl From<&str> for RangeMap {
    fn from(value: &str) -> Self {
        let mut values = value.split_ascii_whitespace();
        let destination_start = values.next().unwrap().parse().unwrap();
        let source_start = values.next().unwrap().parse().unwrap();
        let length = values.next().unwrap().parse().unwrap();

        Self {
            destination_start,
            source_start,
            length,
        }
    }
}

#[test]
fn test_part_2() {
    let data = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(solve_part2(data), 46);
}
