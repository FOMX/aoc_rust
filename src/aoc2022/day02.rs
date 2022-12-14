/// problem: https://adventofcode.com/2022/day/2
/// input: https://adventofcode.com/2022/day/2/input
use std::str::FromStr;

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 2,
    name: "Rock Paper Scissors",
};

#[derive(Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn defeats(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
    pub fn draws(&self) -> Self {
        *self
    }

    pub fn loses_to(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

pub struct Game {
    p1: Hand,
    p2: Hand,
}

impl Game {
    pub fn evaluate(&self) -> usize {
        // TODO: grid lookup would be better?
        match (self.p2, self.p1) {
            (Hand::Rock, Hand::Rock) => 1 + 3,
            (Hand::Rock, Hand::Paper) => 1 + 0,
            (Hand::Rock, Hand::Scissors) => 1 + 6,
            (Hand::Paper, Hand::Rock) => 2 + 6,
            (Hand::Paper, Hand::Paper) => 2 + 3,
            (Hand::Paper, Hand::Scissors) => 2 + 0,
            (Hand::Scissors, Hand::Rock) => 3 + 0,
            (Hand::Scissors, Hand::Paper) => 3 + 6,
            (Hand::Scissors, Hand::Scissors) => 3 + 3,
        }
    }

    /// TODO: i don't like this 
    pub fn from_str_part1(s: &str) -> anyhow::Result<Self> {
        let (l, r) = s.trim().split_once(' ').expect("should be have space");
        let p1 = match l {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("shouldn't be other char here"),
        };
        let p2 = match r {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("shouldn't be other char here"),
        };
        Ok(Self { p1, p2 })
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (l, r) = s.trim().split_once(' ').expect("should be have space");
        let p1 = match l {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("shouldn't be other char here"),
        };
        let p2 = match r {
            "X" => p1.defeats(),
            "Y" => p1.draws(),
            "Z" => p1.loses_to(),
            _ => panic!("shouldn't be other char here"),
        };
        Ok(Self { p1, p2 })
    }
}

pub fn part_1_solution(games: Vec<Game>) -> usize {
    games.iter().map(|g| g.evaluate()).sum()
}

pub fn part_2_solution(games: Vec<Game>) -> usize {
    games.iter().map(|g| g.evaluate()).sum()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;
    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let games: Vec<Game> = read_to_string(P.example_path("_1"))
            .expect("no such file")
            .lines()
            .map(|line| Game::from_str_part1(line).expect("should be a game"))
            .collect();
        let expected = 15;
        assert_eq!(expected, part_1_solution(games))
    }

    #[test]
    fn input_part_1() {
        let games: Vec<Game> = read_to_string(P.input_path())
            .expect("no such file")
            .lines()
            .map(|line| Game::from_str_part1(line).expect("should be a game"))
            .collect();
        let expected = 12855;
        assert_eq!(expected, part_1_solution(games))
    }

    #[test]
    fn example_part_2() {
        let games = read_to_one_per_line::<Game>(P.example_path("_1")).unwrap();
        let expected = 12;
        assert_eq!(expected, part_2_solution(games))
    }

    #[test]
    fn input_part_2() {
        let games = read_to_one_per_line::<Game>(P.input_path()).unwrap();
        let expected = 13726;
        assert_eq!(expected, part_2_solution(games))
    }
}
