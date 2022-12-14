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
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    pub fn defeats(&self) -> Self {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper,
        }
    }
    pub fn draws(&self) -> Self {
        *self
    }

    pub fn loses_to(&self) -> Self {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        }
    }
}

pub struct Game {
    p1: RockPaperScissors,
    p2: RockPaperScissors,
}

impl Game {
    pub fn eval_p2(&self) -> usize {
        // TODO: grid lookup would be better?
        match self.p2 {
            RockPaperScissors::Rock => {
                1 + match self.p1 {
                    RockPaperScissors::Rock => 3,
                    RockPaperScissors::Paper => 0,
                    RockPaperScissors::Scissors => 6,
                }
            }
            RockPaperScissors::Paper => {
                2 + match self.p1 {
                    RockPaperScissors::Rock => 6,
                    RockPaperScissors::Paper => 3,
                    RockPaperScissors::Scissors => 0,
                }
            }
            RockPaperScissors::Scissors => {
                3 + match self.p1 {
                    RockPaperScissors::Rock => 0,
                    RockPaperScissors::Paper => 6,
                    RockPaperScissors::Scissors => 3,
                }
            }
        }
    }

    pub fn from_str_part1(s: &str) -> anyhow::Result<Self> {
        let (l, r) = s.trim().split_once(' ').expect("should be have space");
        let p1 = match l {
            "A" => RockPaperScissors::Rock,
            "B" => RockPaperScissors::Paper,
            "C" => RockPaperScissors::Scissors,
            _ => panic!("shouldn't be other char here"),
        };
        let p2 = match r {
            "X" => RockPaperScissors::Rock,
            "Y" => RockPaperScissors::Paper,
            "Z" => RockPaperScissors::Scissors,
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
            "A" => RockPaperScissors::Rock,
            "B" => RockPaperScissors::Paper,
            "C" => RockPaperScissors::Scissors,
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
    games.iter().map(|g| g.eval_p2()).sum()
}
pub fn part_2_solution(games: Vec<Game>) -> usize {
    games.iter().map(|g| g.eval_p2()).sum()
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
