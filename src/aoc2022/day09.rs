/// problem: https://adventofcode.com/2022/day/9
/// input: https://adventofcode.com/2022/day/9/input
///
use std::{collections::HashSet, str::FromStr};

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 9,
    name: "Rope Bridge",
};

pub struct Instruction {
    dir: Coord,
    steps: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut cs = s.split(' ');
        let dir = match cs.next().unwrap() {
            "R" => Coord(0, 1),
            "U" => Coord(1, 0),
            "L" => Coord(0, -1),
            "D" => Coord(-1, 0),
            _ => panic!(),
        };
        let steps = cs.next().unwrap().parse().unwrap();
        Ok(Instruction { dir, steps })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(isize, isize);
impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Coord {
    pub fn length(&self) -> f32 {
        ((self.0.pow(2) + self.1.pow(2)) as f32).sqrt()
    }

    pub fn direction(&self) -> Coord {
        Coord(self.0.signum(), self.1.signum())
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn update_tail(head: &Coord, tail: &Coord) -> Coord {
    let head_vec = *head - *tail;
    let length = head_vec.length();
    let dir = head_vec.direction(); // dir is either  (1,0), (-1,0), (0,-1), (0,1),  (1,1), (-1,-1)
    if length <= 2f32.sqrt() {
        // could just do <2
        // the vec is either (0,1) (1,0) (1,1) do nothing
        *tail
    } else {
        *tail + dir
    }
}

pub fn part_1_solution(instructions: Vec<Instruction>) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut tail = Coord(0, 0);
    let mut head = Coord(0, 0);

    for inst in instructions {
        for _ in 0..inst.steps {
            head += inst.dir;
            tail = update_tail(&head, &tail);
            visited.insert(tail);
        }
    }
    visited.len()
}
pub fn part_2_solution(instructions: Vec<Instruction>) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut tails = vec![Coord(0, 0); 9];
    let mut head = Coord(0, 0);

    for inst in instructions {
        for _ in 0..inst.steps {
            head += inst.dir;
            tails[0] = update_tail(&head, &tails[0]);
            for i in 0..8 {
                tails[i + 1] = update_tail(&tails[i], &tails[i + 1]);
            }
            visited.insert(tails[8]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let dirs = read_to_one_per_line::<Instruction>(P.example_path("_1")).unwrap();
        let expected = 13;
        assert_eq!(expected, part_1_solution(dirs))
    }

    #[test]
    fn input_part_1() {
        let dirs = read_to_one_per_line::<Instruction>(P.input_path()).unwrap();
        let expected = 5779;
        assert_eq!(expected, part_1_solution(dirs))
    }

    #[test]
    fn example_part_2() {
        let dirs = read_to_one_per_line::<Instruction>(P.example_path("_1")).unwrap();
        let expected = 1;
        assert_eq!(expected, part_2_solution(dirs))
    }

    #[test]
    fn input_part_2() {
        let dirs = read_to_one_per_line::<Instruction>(P.input_path()).unwrap();
        let expected = 2331;
        assert_eq!(expected, part_2_solution(dirs))
    }
}
