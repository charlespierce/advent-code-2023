use aoc_runner_derive::aoc;
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

#[aoc(day19, part1)]
fn solve_part1(input: &str) -> usize {
    let (sorter_str, parts_str) = input.split_once("\n\n").unwrap();
    let sorter = Sorter::from(sorter_str);

    parts_str
        .lines()
        .map(Part::from)
        .filter(|part| sorter.part_accepted(*part))
        .map(Part::rating)
        .sum()
}

#[aoc(day19, part2)]
fn solve_part2(input: &str) -> usize {
    let sorter_str = input.split("\n\n").next().unwrap();
    let sorter = Sorter::from(sorter_str);

    sorter
        .accepted_ranges()
        .iter()
        .map(PartRange::combinations)
        .sum()
}

#[derive(Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn rating(self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        let pieces = value
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',');

        for piece in pieces {
            let (key, value_str) = piece.split_once('=').unwrap();
            let value = value_str.parse().unwrap();

            match key {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => unreachable!(),
            }
        }

        part
    }
}

#[derive(Clone, Copy)]
enum Target<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

impl<'a> From<&'a str> for Target<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Target::Accepted,
            "R" => Target::Rejected,
            s => Target::Workflow(s),
        }
    }
}

#[derive(Clone, Copy)]
enum Condition {
    XLess(usize),
    XGreater(usize),
    MLess(usize),
    MGreater(usize),
    ALess(usize),
    AGreater(usize),
    SLess(usize),
    SGreater(usize),
    Always,
}

impl Condition {
    fn applies(self, part: Part) -> bool {
        use Condition::*;

        match self {
            XLess(cmp) => part.x < cmp,
            XGreater(cmp) => part.x > cmp,
            MLess(cmp) => part.m < cmp,
            MGreater(cmp) => part.m > cmp,
            ALess(cmp) => part.a < cmp,
            AGreater(cmp) => part.a > cmp,
            SLess(cmp) => part.s < cmp,
            SGreater(cmp) => part.s > cmp,
            Always => true,
        }
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let (condition, number_str) = value.split_at(2);
        let number = number_str.parse().unwrap();

        match condition {
            "x<" => Self::XLess(number),
            "x>" => Self::XGreater(number),
            "m<" => Self::MLess(number),
            "m>" => Self::MGreater(number),
            "a<" => Self::ALess(number),
            "a>" => Self::AGreater(number),
            "s<" => Self::SLess(number),
            "s>" => Self::SGreater(number),
            _ => unreachable!(),
        }
    }
}

struct Workflow<'a> {
    predicates: Vec<(Condition, Target<'a>)>,
}

impl<'a> Workflow<'a> {
    fn apply(&self, part: Part) -> Target<'a> {
        for (condition, target) in &self.predicates {
            if condition.applies(part) {
                return *target;
            }
        }

        unreachable!()
    }
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let predicates = value
            .split(',')
            .map(|pred| match pred.split_once(':') {
                Some((condition, target)) => (Condition::from(condition), Target::from(target)),
                None => (Condition::Always, Target::from(pred)),
            })
            .collect();

        Self { predicates }
    }
}

struct Sorter<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> Sorter<'a> {
    fn part_accepted(&self, part: Part) -> bool {
        let mut current = Target::Workflow("in");

        while let Target::Workflow(id) = current {
            let workflow = self.workflows.get(id).unwrap();

            current = workflow.apply(part);
        }

        matches!(current, Target::Accepted)
    }

    fn accepted_ranges(&self) -> Vec<PartRange> {
        let mut accepted = Vec::new();
        let mut ranges = VecDeque::new();
        ranges.push_back((PartRange::new(), Target::Workflow("in")));

        while let Some((range, target)) = ranges.pop_front() {
            match target {
                Target::Rejected => {}
                Target::Accepted => accepted.push(range),
                Target::Workflow(id) => {
                    let workflow = self.workflows.get(id).unwrap();
                    let mut active_range = range;

                    for (condition, target) in &workflow.predicates {
                        let (matched, no_matched) = active_range.split_condition(*condition);

                        if let Some(matched_range) = matched {
                            ranges.push_back((matched_range, *target));
                        }

                        if let Some(no_matched_range) = no_matched {
                            active_range = no_matched_range;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        accepted
    }
}

impl<'a> From<&'a str> for Sorter<'a> {
    fn from(value: &'a str) -> Self {
        let workflows = value
            .lines()
            .map(|line| {
                let (id, conditions) = line.split_once('{').unwrap();
                let workflow = Workflow::from(conditions.trim_end_matches('}'));

                (id, workflow)
            })
            .collect();

        Self { workflows }
    }
}

#[derive(Clone)]
struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    fn split_condition(&self, condition: Condition) -> (Option<Self>, Option<Self>) {
        match condition {
            Condition::Always => (Some(self.clone()), None),
            Condition::XLess(cmp) => {
                let x_start = *self.x.start();
                let x_end = *self.x.end();

                if cmp <= x_start {
                    (None, Some(self.clone()))
                } else if cmp <= x_end {
                    (
                        Some(Self {
                            x: x_start..=(cmp - 1),
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                        Some(Self {
                            x: cmp..=x_end,
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::XGreater(cmp) => {
                let x_start = *self.x.start();
                let x_end = *self.x.end();

                if cmp >= x_end {
                    (None, Some(self.clone()))
                } else if cmp >= x_start {
                    (
                        Some(Self {
                            x: (cmp + 1)..=x_end,
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                        Some(Self {
                            x: x_start..=cmp,
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::MLess(cmp) => {
                let m_start = *self.m.start();
                let m_end = *self.m.end();

                if cmp <= m_start {
                    (None, Some(self.clone()))
                } else if cmp <= m_end {
                    (
                        Some(Self {
                            x: self.x.clone(),
                            m: m_start..=(cmp - 1),
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                        Some(Self {
                            x: self.x.clone(),
                            m: cmp..=m_end,
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::MGreater(cmp) => {
                let m_start = *self.m.start();
                let m_end = *self.m.end();

                if cmp >= m_end {
                    (None, Some(self.clone()))
                } else if cmp >= m_start {
                    (
                        Some(Self {
                            x: self.x.clone(),
                            m: (cmp + 1)..=m_end,
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                        Some(Self {
                            x: self.x.clone(),
                            m: m_start..=cmp,
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::ALess(cmp) => {
                let a_start = *self.a.start();
                let a_end = *self.a.end();

                if cmp <= a_start {
                    (None, Some(self.clone()))
                } else if cmp <= a_end {
                    (
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: a_start..=(cmp - 1),
                            s: self.s.clone(),
                        }),
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: cmp..=a_end,
                            s: self.s.clone(),
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::AGreater(cmp) => {
                let a_start = *self.a.start();
                let a_end = *self.a.end();

                if cmp >= a_end {
                    (None, Some(self.clone()))
                } else if cmp >= a_start {
                    (
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: (cmp + 1)..=a_end,
                            s: self.s.clone(),
                        }),
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: a_start..=cmp,
                            s: self.s.clone(),
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::SLess(cmp) => {
                let s_start = *self.s.start();
                let s_end = *self.s.end();

                if cmp <= s_start {
                    (None, Some(self.clone()))
                } else if cmp <= s_end {
                    (
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: s_start..=(cmp - 1),
                        }),
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: cmp..=s_end,
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
            Condition::SGreater(cmp) => {
                let s_start = *self.s.start();
                let s_end = *self.s.end();

                if cmp >= s_end {
                    (None, Some(self.clone()))
                } else if cmp >= s_start {
                    (
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: (cmp + 1)..=s_end,
                        }),
                        Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: s_start..=cmp,
                        }),
                    )
                } else {
                    (Some(self.clone()), None)
                }
            }
        }
    }

    fn combinations(&self) -> usize {
        let x_range = *self.x.end() - *self.x.start() + 1;
        let m_range = *self.m.end() - *self.m.start() + 1;
        let a_range = *self.a.end() - *self.a.start() + 1;
        let s_range = *self.s.end() - *self.s.start() + 1;

        x_range * m_range * a_range * s_range
    }
}
