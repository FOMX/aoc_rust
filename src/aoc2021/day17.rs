/// problem: https://adventofcode.com/2021/day/17
/// input: "https://adventofcode.com/2021/day/17/input"
use itertools::Itertools;

use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 17,
    name: "Trick Shot",
};

#[derive(Debug, Clone, Copy)]
pub struct Coord(isize, isize);

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

pub struct Projectile {
    position: Coord,
    velocity: Coord,
    acceleration: Coord,
}

impl Projectile {
    pub fn new(initial_velocity: Coord) -> Self {
        Self {
            position: Coord(0, 0), // initial position is always 0
            velocity: initial_velocity,
            acceleration: Coord(-initial_velocity.0.signum(), -1),
        }
    }

    pub fn step(&mut self) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.acceleration.0 = -self.velocity.0.signum()
    }
}

pub fn step(start: &mut (isize, isize), velocity: &mut (isize, isize)) {
    start.0 += velocity.0;
    start.1 += velocity.1;
    velocity.0 += match velocity.0 {
        v if v < 0 => 1,
        v if v > 0 => -1,
        _ => 0,
    };
    velocity.1 -= 1;
}

// fn triangle_number(n: usize) -> usize {
//     n * (n + 1) / 2
// }

// fn reverse_triangle_number(n: usize) -> usize {
//     ((-1.0 + (1.0 + 8.0 * (n as f32)).sqrt()) / 2.0).ceil() as usize
//     // ((2 * n) as f32).sqrt().ceil() as usize
// }

#[derive(Debug)]
pub struct TargetArea {
    pub xmin: isize,
    pub xmax: isize,
    pub ymin: isize,
    pub ymax: isize,
}

impl TargetArea {
    pub fn is_in_area(&self, position: Coord) -> bool {
        let (x, y) = (position.0, position.1);
        x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }
    pub fn is_impossible(&self, p: &Projectile) -> bool {
        // x position out of range
        p.position.0.abs() > self.xmax.abs().max(self.xmin.abs()) ||
        // yvelocity is down and we're below the range
        (p.velocity.1 < 0 && p.position.1 < self.ymin) ||
        // xvel is 0 and not in range 
        (p.velocity.0 == 0 && (p.position.0 > self.xmax || p.position.0 < self.xmin))
    }

    pub fn candidate_velocities(&self) -> impl Iterator<Item = Coord> {
        let max_val = 500;
        let xmin = if self.xmax > 0 && self.xmin > 0 {
            0
        } else {
            -max_val
        };
        let xmax = if self.xmax < 0 && self.xmin < 0 {
            0
        } else {
            max_val
        };
        // TODO: sort appropriately e.g. start at 0 for x and y and move out
        (xmin..xmax)
            .sorted()
            .flat_map(move |x| (1..max_val).chain(-max_val..1).map(move |y| Coord(x, y)))
    }
}

pub fn part_1_solution(target_area: TargetArea) -> isize {
    let mut global_best_height = 0;
    for vel in target_area.candidate_velocities() {
        let mut p = Projectile::new(vel);
        let mut best_height = 0;
        for _ in 0..1000 {
            p.step();
            if p.position.1 > best_height {
                best_height = p.position.1;
            }
            if target_area.is_in_area(p.position) {
                if best_height > global_best_height {
                    global_best_height = best_height;
                }
                break;
            }
            if target_area.is_impossible(&p) {
                break;
            }
        }
    }
    global_best_height
}

pub fn part_2_solution(target_area: TargetArea) -> isize {
    let mut res = 0;
    for vel in target_area.candidate_velocities() {
        let mut p = Projectile::new(vel);
        for _ in 0..1000 {
            p.step();
            if target_area.is_in_area(p.position) {
                res += 1;
                break;
            }
            if target_area.is_impossible(&p) {
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    fn read_day_17(path: impl AsRef<std::path::Path>) -> anyhow::Result<TargetArea> {
        let file = read_to_string(path)?;
        let mut splits = file
            .trim() // get rid of trailing blank line
            .split(" ");

        // target area: x=20..30, y=-10..-5
        // skip 2
        splits.next();
        splits.next();

        let mut temp_xs = splits.next().unwrap().split("=");
        temp_xs.next();
        temp_xs = temp_xs.next().unwrap().split(",");
        let mut xs = temp_xs.next().unwrap().split("..");

        let mut temp_ys = splits.next().unwrap().split("=");
        temp_ys.next();
        let mut ys = temp_ys.next().unwrap().split("..");

        Ok(TargetArea {
            xmin: xs.next().unwrap().parse()?,
            xmax: xs.next().unwrap().parse()?,
            ymin: ys.next().unwrap().parse()?,
            ymax: ys.next().unwrap().parse()?,
        })
    }

    #[test]
    fn example_part_1() {
        let target_area = read_day_17(P.example_path("_1")).unwrap();
        let expected = 45;

        assert_eq!(expected, part_1_solution(target_area))
    }

    #[test]
    fn example2_part_1() {
        let target_area = read_day_17(P.example_path("_2")).unwrap();
        let expected = 45;

        assert_eq!(expected, part_1_solution(target_area))
    }

    #[test]
    fn input_part_1() {
        let target_area = read_day_17(P.input_path()).unwrap();
        let expected = 2775;
        assert_eq!(expected, part_1_solution(target_area))
    }

    #[test]
    fn example_part_2() {
        let target_area = read_day_17(P.example_path("_1")).unwrap();
        let expected = 112;
        assert_eq!(expected, part_2_solution(target_area))
    }

    #[test]
    fn input_part_2() {
        let target_area = read_day_17(P.input_path()).unwrap();
        let expected = 1566;
        assert_eq!(expected, part_2_solution(target_area))
    }
}
