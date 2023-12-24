use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> usize {
    let hailstones: Vec<_> = input.lines().map(Hailstone::from).collect();
    let mut count = 0;

    for i in 0..hailstones.len() - 1 {
        for j in i + 1..hailstones.len() {
            let first = &hailstones[i];
            let second = &hailstones[j];

            if let Some(intersection) = first.intersects_2d(second) {
                if within_test_area_2d(intersection) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> f64 {
    // Assumptions: Velocities are integer and relatively small (+- 500)
    //              Times for intercept are integers
    //              Positions for intercept are integers
    // Given that, if two hailstones have the same velocity in a single direction, then the
    // distance in that direction between the hzilstones must be an even multiple of the relative
    // velocity of the rock - Otherwise it would be impossible for the rock to intercept both
    // at an integer time / position. Therefore:
    //  distance % relative_velocity == 0
    //  (b.x - a.x) % (rock_vx - vx) == 0
    //
    // We can do the same calculation for y and z for any pair of hailstones that have the same
    // velocity in those directions.
    //
    // Working with the assumption that the velocity is relatively small, we can build a set of
    // possible values and refine it as we find pairs.
    let hailstones: Vec<_> = input.lines().map(Hailstone::from).collect();
    let mut possible_x_velocities = HashSet::<isize>::new();
    let mut possible_y_velocities = HashSet::<isize>::new();
    let mut possible_z_velocities = HashSet::<isize>::new();

    for i in 0..hailstones.len() - 1 {
        let a = hailstones[i];
        for b in hailstones.iter().skip(i + 1) {
            if a.velocity.x == b.velocity.x {
                let mut new_x_set = HashSet::new();
                let distance = b.position.x - a.position.x;
                for possible_speed in -500..=500 {
                    let relative_speed = possible_speed - a.velocity.x;
                    if relative_speed != 0 && distance % relative_speed == 0 {
                        new_x_set.insert(possible_speed);
                    }
                }
                if possible_x_velocities.is_empty() {
                    possible_x_velocities = new_x_set;
                } else {
                    possible_x_velocities = possible_x_velocities
                        .intersection(&new_x_set)
                        .copied()
                        .collect();
                }
            }

            if a.velocity.y == b.velocity.y {
                let mut new_y_set = HashSet::new();
                let distance = b.position.y - a.position.y;
                for possible_speed in -500..=500 {
                    let relative_speed = possible_speed - a.velocity.y;
                    if relative_speed != 0 && distance % relative_speed == 0 {
                        new_y_set.insert(possible_speed);
                    }
                }
                if possible_y_velocities.is_empty() {
                    possible_y_velocities = new_y_set;
                } else {
                    possible_y_velocities = possible_y_velocities
                        .intersection(&new_y_set)
                        .copied()
                        .collect();
                }
            }

            if a.velocity.z == b.velocity.z {
                let mut new_z_set = HashSet::new();
                let distance = b.position.z - a.position.z;
                for possible_speed in -500..=500 {
                    let relative_speed = possible_speed - a.velocity.z;
                    if relative_speed != 0 && distance % relative_speed == 0 {
                        new_z_set.insert(possible_speed);
                    }
                }
                if possible_z_velocities.is_empty() {
                    possible_z_velocities = new_z_set;
                } else {
                    possible_z_velocities = possible_z_velocities
                        .intersection(&new_z_set)
                        .copied()
                        .collect();
                }
            }
        }
    }

    // It turns out the input is constructed such that there's a single value for each speed
    // So we can use that as the velocity. Then we can use existing points to determine the
    // starting position of the rock.
    let vx = possible_x_velocities.into_iter().next().unwrap() as f64;
    let vy = possible_y_velocities.into_iter().next().unwrap() as f64;
    let vz = possible_z_velocities.into_iter().next().unwrap() as f64;

    // From some algebraic solving of linear equations, the positions can be determined using:
    let first = hailstones[0];
    let second = hailstones[1];

    let vyx = (first.velocity.y as f64 - vy) / (first.velocity.x as f64 - vx);
    let vyx_2 = (second.velocity.y as f64 - vy) / (second.velocity.x as f64 - vx);
    let vzx = (first.velocity.z as f64 - vz) / (first.velocity.x as f64 - vx);

    let x = (first.position.y as f64 - second.position.y as f64 - first.position.x as f64 * vyx
        + second.position.x as f64 * vyx_2)
        / (vyx_2 - vyx);
    let y = first.position.y as f64 + (x - first.position.x as f64) * vyx;
    let z = first.position.z as f64 + (x - first.position.x as f64) * vzx;

    x + y + z
}

#[derive(Clone, Copy)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let mut values = value.split(", ").map(|n| n.parse::<isize>().unwrap());
        let x = values.next().unwrap();
        let y = values.next().unwrap();
        let z = values.next().unwrap();

        Self { x, y, z }
    }
}

#[derive(Clone, Copy)]
struct Hailstone {
    position: Vector,
    velocity: Vector,
}

impl Hailstone {
    fn slope_2d(&self) -> f64 {
        self.velocity.y as f64 / self.velocity.x as f64
    }

    fn y_intercept_2d(&self) -> f64 {
        self.position.y as f64 - (self.slope_2d() * self.position.x as f64)
    }

    fn intersects_2d(&self, other: &Self) -> Option<Vector> {
        let intersection_x =
            (other.y_intercept_2d() - self.y_intercept_2d()) / (self.slope_2d() - other.slope_2d());
        let intersection_y = (self.slope_2d() * intersection_x) + self.y_intercept_2d();

        let intersection_time_self =
            (intersection_x - self.position.x as f64) / self.velocity.x as f64;
        let intersection_time_other =
            (intersection_x - other.position.x as f64) / other.velocity.x as f64;

        if intersection_time_self >= 0. && intersection_time_other >= 0. {
            Some(Vector {
                x: intersection_x as isize,
                y: intersection_y as isize,
                z: 0,
            })
        } else {
            None
        }
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (pos, vel) = value.split_once(" @ ").unwrap();
        let position = Vector::from(pos);
        let velocity = Vector::from(vel);

        Self { position, velocity }
    }
}

fn within_test_area_2d(intersection: Vector) -> bool {
    intersection.x >= 200_000_000_000_000
        && intersection.x <= 400_000_000_000_000
        && intersection.y >= 200_000_000_000_000
        && intersection.y <= 400_000_000_000_000
}
