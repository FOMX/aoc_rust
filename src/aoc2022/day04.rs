/// problem: https://adventofcode.com/2022/day/4
/// input: https://adventofcode.com/2022/day/4/input
use std::str::FromStr;

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 4,
    name: "Elf Ranges",
};

pub struct ElfRange {
    low: usize,
    high: usize,
}

pub struct Pair {
    a: ElfRange,
    b: ElfRange,
}

impl Pair {
    fn is_enveloped(&self) -> bool {
        (self.a.high <= self.b.high && self.a.low >= self.b.low)
            || (self.b.high <= self.a.high && self.b.low >= self.a.low)
    }

    fn is_overlapped(&self) -> bool {
        !((self.a.low > self.b.high) || (self.b.low > self.a.high))
    }
}
impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (a, b) = s.split_once(',').unwrap();
        let (alow, ahigh) = a.split_once('-').unwrap();
        let (blow, bhigh) = b.split_once('-').unwrap();
        let a = ElfRange {
            low: alow.parse().unwrap(),
            high: ahigh.parse().unwrap(),
        };
        let b = ElfRange {
            low: blow.parse().unwrap(),
            high: bhigh.parse().unwrap(),
        };

        Ok(Pair { a, b })
    }
}

pub fn part_1_solution(pairs: Vec<Pair>) -> usize {
    pairs.iter().filter(|&pair| pair.is_enveloped()).count()
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
