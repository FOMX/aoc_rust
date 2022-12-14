/// problem: https://adventofcode.com/2022/day/3
/// input: https://adventofcode.com/2022/day/3/input
use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 3,
    name: "RuckSacks",
};

use std::{collections::HashSet, str::FromStr};

pub struct RuckSack {
    pub left: String,
    pub right: String,
}

impl FromStr for RuckSack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (left, right) = s.split_at(s.len() / 2);
        Ok(Self {
            left: left.to_owned(),
            right: right.to_owned(),
        })
    }
}

impl RuckSack {
    fn score(&self) -> usize {
        let mut hs = HashSet::new();
        for c in self.left.chars() {
            hs.insert(c);
        }
        let mut doubled_chars = HashSet::new();
        for c in self.right.chars() {
            if hs.contains(&c) {
                doubled_chars.insert(c);
            }
        }
        doubled_chars
            .iter()
            .map(|c| {
                if c.is_uppercase() {
                    (*c as usize) - ('A' as usize) + 27
                } else {
                    (*c as usize) - ('a' as usize) + 1
                }
            })
            .sum()
    }
}

pub fn part_1_solution(rucksacks: Vec<RuckSack>) -> usize {
    rucksacks.iter().map(|rs| rs.score()).sum()
}
pub fn part_2_solution(rucksacks: Vec<String>) -> usize {
    let mut score = 0;
    for chunk in rucksacks.chunks(3) {
        let mut hs = HashSet::new();
        for c in chunk[0].chars() {
            hs.insert(c);
        }
        hs.retain(|&c| chunk[1].contains(c));
        hs.retain(|&c| chunk[2].contains(c));
        score += hs
            .iter()
            .map(|c| {
                if c.is_uppercase() {
                    (*c as usize) - ('A' as usize) + 27
                } else {
                    (*c as usize) - ('a' as usize) + 1
                }
            })
            .sum::<usize>();
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let input = read_to_one_per_line::<RuckSack>(P.example_path("_1")).unwrap();
        let expected = 157;
        assert_eq!(expected, part_1_solution(input))
    }

    #[test]
    fn input_part_1() {
        let input = read_to_one_per_line::<RuckSack>(P.input_path()).unwrap();
        let expected = 7990;
        assert_eq!(expected, part_1_solution(input))
    }

    #[test]
    fn example_part_2() {
        let input = read_to_one_per_line::<String>(P.example_path("_1")).unwrap();
        let expected = 70;
        assert_eq!(expected, part_2_solution(input))
    }

    #[test]
    fn input_part_2() {
        let input = read_to_one_per_line::<String>(P.input_path()).unwrap();
        let expected = 2602;
        assert_eq!(expected, part_2_solution(input))
    }
}
