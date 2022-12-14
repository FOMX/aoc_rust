/// problem: https://adventofcode.com/2022/day/4
/// input: https://adventofcode.com/2022/day/4/input
use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 4,
    name: "Elf Ranges",
};

#[repr(transparent)]
pub struct ElfRange(RangeInclusive<usize>);

impl ElfRange {
    fn contains_elfrange(&self, other: &Self) -> bool {
        // TODO how to auto destructure tuple struct
        self.0.contains(other.0.start()) && self.0.contains(other.0.end()) // O(1) just checks bounds
    }

    fn overlaps_elfrange(&self, other: &Self) -> bool {
        // TODO how to auto destructure tuple struct
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

pub struct Pair {
    left: ElfRange,
    right: ElfRange,
}

impl Pair {
    fn is_contained(&self) -> bool {
        self.left.contains_elfrange(&self.right) || self.right.contains_elfrange(&self.left)
    }

    fn is_overlapped(&self) -> bool {
        self.left.overlaps_elfrange(&self.right) || self.right.overlaps_elfrange(&self.left)
    }
}
impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        // 2-4,6-8
        let (left, right) = s
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|n| n.parse().expect("Should be usize"))
                    .collect_tuple::<(usize, usize)>()
                    .map(|(low, high)| ElfRange(low..=high))
                    .expect("must be 2")
            })
            .collect_tuple::<(_, _)>()
            .expect("must be 2");

        Ok(Pair { left, right })
    }
}

pub fn part_1_solution(pairs: Vec<Pair>) -> usize {
    pairs.iter().filter(|&pair| pair.is_contained()).count()
}
pub fn part_2_solution(pairs: Vec<Pair>) -> usize {
    pairs.iter().filter(|&pair| pair.is_overlapped()).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let pairs = read_to_one_per_line::<Pair>(P.example_path("_1")).unwrap();
        let expected = 2;
        assert_eq!(expected, part_1_solution(pairs))
    }

    #[test]
    fn input_part_1() {
        let pairs = read_to_one_per_line::<Pair>(P.input_path()).unwrap();
        let expected = 444;
        assert_eq!(expected, part_1_solution(pairs))
    }

    #[test]
    fn example_part_2() {
        let pairs = read_to_one_per_line::<Pair>(P.example_path("_1")).unwrap();
        let expected = 4;
        assert_eq!(expected, part_2_solution(pairs))
    }

    #[test]
    fn input_part_2() {
        let pairs = read_to_one_per_line::<Pair>(P.input_path()).unwrap();
        let expected = 801;
        assert_eq!(expected, part_2_solution(pairs))
    }
}
