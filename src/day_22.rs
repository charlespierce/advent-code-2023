use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;

#[aoc(day22, part1)]
fn solve_part1(input: &str) -> usize {
    let mut stack = Stack::from(input);
    stack.drop();

    stack.count_disintegrations()
}

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> usize {
    let mut stack = Stack::from(input);
    stack.drop();

    stack.count_bricks_dropping()
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn drop(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

#[derive(Clone, Debug)]
enum Brick {
    X {
        x: RangeInclusive<usize>,
        y: usize,
        z: usize,
    },
    Y {
        x: usize,
        y: RangeInclusive<usize>,
        z: usize,
    },
    Z {
        x: usize,
        y: usize,
        z: RangeInclusive<usize>,
    },
}

impl Brick {
    fn overlaps(&self, point: Point) -> bool {
        match self {
            Brick::X { x, y, z } => point.y == *y && point.z == *z && x.contains(&point.x),
            Brick::Y { x, y, z } => point.x == *x && point.z == *z && y.contains(&point.y),
            Brick::Z { x, y, z } => point.x == *x && point.y == *y && z.contains(&point.z),
        }
    }

    fn footprint(&self) -> Vec<Point> {
        match self {
            Brick::X { x, y, z } => x.clone().map(|x| Point { x, y: *y, z: *z }).collect(),
            Brick::Y { x, y, z } => y.clone().map(|y| Point { x: *x, y, z: *z }).collect(),
            Brick::Z { x, y, z } => {
                vec![Point {
                    x: *x,
                    y: *y,
                    z: *z.start(),
                }]
            }
        }
    }

    fn drop(&mut self) {
        match self {
            Brick::X { z, .. } => {
                *z -= 1;
            }
            Brick::Y { z, .. } => {
                *z -= 1;
            }
            Brick::Z { z, .. } => {
                let new_range = (*z.start() - 1)..=(*z.end() - 1);
                *z = new_range;
            }
        }
    }

    fn lowest_z(&self) -> usize {
        match self {
            Brick::X { z, .. } => *z,
            Brick::Y { z, .. } => *z,
            Brick::Z { z, .. } => *z.start(),
        }
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('~').unwrap();
        let (start_x, start_y, start_z) = parse_coordinates(start);
        let (end_x, end_y, end_z) = parse_coordinates(end);

        if start_x < end_x {
            Brick::X {
                x: start_x..=end_x,
                y: start_y,
                z: start_z,
            }
        } else if start_x > end_x {
            Brick::X {
                x: end_x..=start_x,
                y: start_y,
                z: start_z,
            }
        } else if start_y < end_y {
            Brick::Y {
                x: start_x,
                y: start_y..=end_y,
                z: start_z,
            }
        } else if start_y > end_y {
            Brick::Y {
                x: start_x,
                y: end_y..=start_y,
                z: start_z,
            }
        } else if start_z < end_z {
            Brick::Z {
                x: start_x,
                y: start_y,
                z: start_z..=end_z,
            }
        } else if start_z > end_z {
            Brick::Z {
                x: start_x,
                y: start_y,
                z: end_z..=start_z,
            }
        } else {
            // If they all overlap, they're equivalent
            Brick::X {
                x: start_x..=end_x,
                y: start_y,
                z: start_z,
            }
        }
    }
}

fn parse_coordinates(value: &str) -> (usize, usize, usize) {
    let mut parts = value.split(',');
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    let z = parts.next().unwrap().parse().unwrap();

    (x, y, z)
}

struct Stack {
    bricks: Vec<Brick>,
}

impl Stack {
    fn count_bricks_dropping(&self) -> usize {
        let mut count = 0;

        for i in 0..self.bricks.len() {
            let mut new_bricks = self.bricks.clone();
            new_bricks.remove(i);

            let mut new_stack = Stack { bricks: new_bricks };
            count += new_stack.drop();
        }

        count
    }

    fn count_disintegrations(&self) -> usize {
        let mut count = 0;

        for i in 0..self.bricks.len() {
            if self.can_disintegrate(i) {
                count += 1;
            }
        }

        count
    }

    fn can_disintegrate(&self, index: usize) -> bool {
        let brick = &self.bricks[index];

        'outer: for i in (index + 1)..self.bricks.len() {
            let dropped_footprint: Vec<_> = self.bricks[i]
                .footprint()
                .into_iter()
                .map(Point::drop)
                .collect();

            let mut touches_brick = false;
            for point in &dropped_footprint {
                if brick.overlaps(*point) {
                    touches_brick = true;
                    break;
                }
            }

            if touches_brick {
                for j in 0..i {
                    if j != index {
                        for point in &dropped_footprint {
                            if self.bricks[j].overlaps(*point) {
                                continue 'outer;
                            }
                        }
                    }
                }

                return false;
            }
        }

        true
    }

    fn drop(&mut self) -> usize {
        let mut moved = 0;

        for i in 0..self.bricks.len() {
            let (previous, rest) = self.bricks.split_at_mut(i);
            let brick = &mut rest[0];

            let mut did_move = false;

            'outer: loop {
                if brick.lowest_z() == 1 {
                    break 'outer;
                }

                let footprint = brick.footprint();
                for point in footprint {
                    let dropped = point.drop();

                    for other in previous.iter() {
                        if other.overlaps(dropped) {
                            break 'outer;
                        }
                    }
                }

                did_move = true;
                brick.drop();
            }

            if did_move {
                moved += 1;
            }
        }

        moved
    }
}

impl From<&str> for Stack {
    fn from(value: &str) -> Self {
        let mut bricks: Vec<_> = value.lines().map(Brick::from).collect();
        bricks.sort_by_key(Brick::lowest_z);

        Self { bricks }
    }
}

#[test]
fn test_part1() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    assert_eq!(7, solve_part2(input));
}
