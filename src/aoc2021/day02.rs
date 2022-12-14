/// problem: https://adventofcode.com/2021/day/2
/// input: "https://adventofcode.com/2021/day/2/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 2,
    name: "Dive!",
};

use crate::error::{Error, ParseError};
use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Instruction {
    direction: Direction,
    distance: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s.split_once(' ') {
            Some((direction, distance)) => Ok(Instruction {
                direction: match direction {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => {
                        panic!("no matching direction")
                    }
                },
                distance: match distance.parse::<usize>() {
                    Ok(d) => d,
                    Err(_) => return Err(Error::ParseError(ParseError::ExpectedDigit)),
                },
            }),
            None => Err(Error::ParseError(ParseError::InvalidSequence(
                "unable to split on ' '",
            ))),
        }
    }
}

pub fn part_1_solution(data: Vec<Instruction>) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    for instruction in data {
        match instruction.direction {
            Direction::Forward => horizontal += instruction.distance,
            Direction::Down => depth += instruction.distance,
            Direction::Up => depth -= instruction.distance,
        }
    }
    horizontal * depth
}

pub fn part_2_solution(data: Vec<Instruction>) -> usize {
    let mut aim: i32 = 0;
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    for instruction in data {
        match instruction.direction {
            Direction::Forward => {
                horizontal += instruction.distance;
                depth += (instruction.distance as i32 * aim) as usize; // TODO fiugure out how to correctly address this issue?
            }
            Direction::Down => aim += instruction.distance as i32,
            Direction::Up => aim -= instruction.distance as i32,
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod test {
    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn example() {
        let data = vec![
            Instruction {
                direction: Direction::Forward,
                distance: 5,
            },
            Instruction {
                direction: Direction::Down,
                distance: 5,
            },
            Instruction {
                direction: Direction::Forward,
                distance: 8,
            },
            Instruction {
                direction: Direction::Up,
                distance: 3,
            },
            Instruction {
                direction: Direction::Down,
                distance: 8,
            },
            Instruction {
                direction: Direction::Forward,
                distance: 2,
            },
        ];
        let expected = 150;
        assert_eq!(part_1_solution(data), expected);
    }

    #[test]
    fn example_import() -> Result<(), Error> {
        let data = read_to_one_per_line::<Instruction>(P.example_path("_1"))?;
        let expected = 150;
        assert_eq!(part_1_solution(data), expected);
        Ok(())
    }

    #[test]
    fn input_part1() {
        let data = read_to_one_per_line::<Instruction>(P.input_path())
            .ok()
            .unwrap();
        let expected = 1924923;
        assert_eq!(part_1_solution(data), expected);
    }

    #[test]
    fn example_part2() {
        let data = read_to_one_per_line::<Instruction>(P.example_path("_1"))
            .ok()
            .unwrap();
        let expected = 900;
        assert_eq!(part_2_solution(data), expected);
    }

    #[test]
    fn input_part2() {
        let data = read_to_one_per_line::<Instruction>(P.input_path())
            .ok()
            .unwrap();
        let expected = 1982495697;
        assert_eq!(part_2_solution(data), expected);
    }
}
