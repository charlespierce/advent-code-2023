use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> usize {
    let mut array = LightArray::default();

    for operation in input.split(',').map(Operation::from) {
        array.apply_operation(operation);
    }

    array.focusing_power()
}

#[derive(Clone, Copy)]
struct Operation<'a> {
    label: &'a str,
    kind: OperationKind,
}

#[derive(Clone, Copy)]
enum OperationKind {
    Dash,
    Equals(usize),
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(value: &'a str) -> Self {
        match value.strip_suffix('-') {
            Some(label) => Operation {
                label,
                kind: OperationKind::Dash,
            },
            None => {
                let (label, focal_str) = value.split_once('=').unwrap();
                Operation {
                    label,
                    kind: OperationKind::Equals(focal_str.parse().unwrap()),
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[derive(Default)]
struct LightArray<'a> {
    boxes: HashMap<usize, Vec<Lens<'a>>>,
}

impl<'a> LightArray<'a> {
    fn apply_operation(&mut self, op: Operation<'a>) {
        let hash_value = hash(op.label);
        let entry = self.boxes.entry(hash_value);

        match op.kind {
            OperationKind::Dash => {
                entry.and_modify(|lenses| {
                    let new_lenses = lenses
                        .iter()
                        .copied()
                        .filter(|lens| lens.label != op.label)
                        .collect();
                    *lenses = new_lenses;
                });
            }
            OperationKind::Equals(focal_length) => {
                let lenses = entry.or_default();
                let mut found = false;

                for lens in lenses.iter_mut() {
                    if lens.label == op.label {
                        lens.focal_length = focal_length;
                        found = true;
                        break;
                    }
                }

                if !found {
                    lenses.push(Lens {
                        label: op.label,
                        focal_length,
                    });
                }
            }
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .map(|(box_number, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(lens_number, lens)| {
                        (*box_number + 1) * (lens_number + 1) * lens.focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn hash(input: &str) -> usize {
    let mut current = 0;

    for chr in input.chars() {
        current += chr as usize;
        current *= 17;
        current %= 256;
    }

    current
}
