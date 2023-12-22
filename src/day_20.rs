use aoc_runner_derive::aoc;
use std::collections::{hash_map::Entry, HashMap, VecDeque};

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> usize {
    let mut array = Array::from(input);

    for _ in 0..1000 {
        array.push_button();
    }

    array.low_pulses * array.high_pulses
}

#[aoc(day20, part2)]
fn solve_part2(input: &str) -> usize {
    let mut array = Array::from(input);
    let mut highs = HashMap::new();

    for i in 1.. {
        if let Some(id) = array.push_button() {
            if let Entry::Vacant(vacant) = highs.entry(id) {
                vacant.insert(i);
            }

            if highs.len() == 4 {
                break;
            }
        }
    }

    highs
        .values()
        .copied()
        .reduce(lowest_common_multiple)
        .unwrap()
}

fn lowest_common_multiple(a: usize, b: usize) -> usize {
    (a / greatest_common_divisor(a, b)) * b
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    loop {
        let r = a % b;
        if r == 0 {
            break b;
        }
        a = b;
        b = r;
    }
}

#[derive(Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Copy)]
enum FlipFlopState {
    On,
    Off,
}

impl FlipFlopState {
    fn toggle(&mut self) -> Pulse {
        *self = match self {
            FlipFlopState::On => FlipFlopState::Off,
            FlipFlopState::Off => FlipFlopState::On,
        };

        match self {
            FlipFlopState::On => Pulse::High,
            FlipFlopState::Off => Pulse::Low,
        }
    }
}

enum Module<'a> {
    Broadcast,
    FlipFlop(FlipFlopState),
    Conjunction(HashMap<&'a str, Pulse>),
}

impl<'a> Module<'a> {
    fn process_pulse(&mut self, pulse: Pulse, source: &'a str) -> Option<Pulse> {
        match (self, pulse) {
            (Module::Broadcast, _) => Some(pulse),
            (Module::FlipFlop(_), Pulse::High) => None,
            (Module::FlipFlop(state), Pulse::Low) => Some(state.toggle()),
            (Module::Conjunction(stored), pulse) => {
                stored.insert(source, pulse);

                Some(
                    if stored.values().all(|pulse| matches!(pulse, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    },
                )
            }
        }
    }
}

struct Array<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    destinations: HashMap<&'a str, Vec<&'a str>>,
    low_pulses: usize,
    high_pulses: usize,
}

impl<'a> Array<'a> {
    fn push_button(&mut self) -> Option<&'a str> {
        let mut pulse_queue = VecDeque::new();
        let mut found_high = None;
        pulse_queue.push_back(("button", Pulse::Low));

        while let Some((source, pulse)) = pulse_queue.pop_front() {
            for destination in self.destinations.get(source).unwrap() {
                if let Some(module) = self.modules.get_mut(destination) {
                    if let Some(new_pulse) = module.process_pulse(pulse, source) {
                        pulse_queue.push_back((destination, new_pulse));
                    }
                }

                // These four are the inputs to the last conjunction. We want to determine
                // what the period is for them to emit high pulses. This value was found by
                // direct inspection of the input
                if matches!(pulse, Pulse::High) && *destination == "kc" {
                    found_high = Some(source);
                }

                match pulse {
                    Pulse::High => self.high_pulses += 1,
                    Pulse::Low => self.low_pulses += 1,
                }
            }
        }

        found_high
    }
}

impl<'a> From<&'a str> for Array<'a> {
    fn from(value: &'a str) -> Self {
        let mut modules = HashMap::new();
        let mut destinations = HashMap::new();

        for line in value.lines() {
            let (module, dests) = line.split_once(" -> ").unwrap();
            let parsed_dests: Vec<_> = dests.split(", ").collect();

            let (id, module) = match module.split_at(1) {
                ("b", _) => ("broadcaster", Module::Broadcast),
                ("%", id) => (id, Module::FlipFlop(FlipFlopState::Off)),
                ("&", id) => (id, Module::Conjunction(HashMap::new())),
                _ => unreachable!(),
            };

            modules.insert(id, module);
            destinations.insert(id, parsed_dests);
        }

        // Need to update all of the conjunction modules with their _sources_
        for (id, module) in modules.iter_mut() {
            if let Module::Conjunction(sources) = module {
                for (source, dests) in destinations.iter() {
                    if dests.contains(id) {
                        sources.insert(source, Pulse::Low);
                    }
                }
            }
        }

        destinations.insert("button", vec!["broadcaster"]);

        Self {
            modules,
            destinations,
            low_pulses: 0,
            high_pulses: 0,
        }
    }
}
