use aoc_runner_derive::aoc;

// Copied from the input directly:
// Time:        45     98     83     73
// Distance:   295   1734   1278   1210
const INPUT: [Race; 4] = [
    Race {
        time: 45,
        length: 295,
    },
    Race {
        time: 98,
        length: 1734,
    },
    Race {
        time: 83,
        length: 1278,
    },
    Race {
        time: 73,
        length: 1210,
    },
];

#[aoc(day6, part1)]
fn solve_part1(_: &str) -> usize {
    INPUT.into_iter().map(|race| race.win_count()).product()
}

#[aoc(day6, part2)]
fn solve_part2(_: &str) -> usize {
    let combined_race = Race {
        time: 45_988_373,
        length: 295_173_412_781_210,
    };

    combined_race.win_count()
}

struct Race {
    time: usize,
    length: usize,
}

impl Race {
    fn win_count(&self) -> usize {
        let mut count = 0;
        for charge_time in 1..self.time {
            if charge_time * (self.time - charge_time) > self.length {
                count += 1;
            }
        }

        count
    }
}
