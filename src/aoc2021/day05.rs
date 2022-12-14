use crate::Problem;
/// problem: https://adventofcode.com/2021/day/5
/// input: "https://adventofcode.com/2021/day/5/input"
use std::{cmp::max, str::FromStr};
const P: Problem = Problem {
    year: 2021,
    day: 5,
    name: "Hydrothermal Venture",
};

pub struct SeaFloor {
    vent_counts: Vec<Vec<usize>>,
}

impl SeaFloor {
    fn new(size: usize) -> Self {
        Self {
            vent_counts: vec![vec![0; size]; size],
        }
    }
}

fn line(start: usize, end: usize) -> Vec<usize> {
    // need to do this because rust's ranges suck
    if start < end {
        (start..=end).collect()
    } else {
        (end..=start).rev().collect()
    }
}

pub struct Vent {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Vent {
    fn is_orthogonal(&self) -> bool {
        // x1 = x2 or y1 = y2.
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn get_coords(&self) -> Vec<(usize, usize)> {
        let xs = line(self.start.0, self.end.0);
        let ys = line(self.start.1, self.end.1);
        if self.start.0 != self.end.0 && self.start.1 != self.end.1 {
            xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)).collect()
        } else {
            xs.iter()
                .flat_map(|x| ys.iter().map(|y| (*x, *y)))
                .collect()
        }
    }
}

impl FromStr for Vent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_s, finish_s) = s.split_once(" -> ").expect("should split once at ->");
        let (start_x, start_y) = start_s.split_once(",").expect("should split once at ,");
        let start = (
            start_x.parse::<usize>().expect("should be a number"),
            start_y.parse::<usize>().expect("should be a number"),
        );

        let (finish_x, finish_y) = finish_s.split_once(",").expect("should split once at ,");
        let finish = (
            finish_x.parse::<usize>().expect("should be a number"),
            finish_y.parse::<usize>().expect("should be a number"),
        );
        Ok(Self { start, end: finish })
    }
}

pub fn part_1_solution(vents: Vec<Vent>) -> usize {
    let mut vents = vents;
    vents.retain(|vent| vent.is_orthogonal());
    let max_coord = vents
        .iter()
        .map(|vent| max(max(vent.start.0, vent.start.1), max(vent.end.0, vent.end.1)))
        .max()
        .unwrap();

    let mut seafloor = SeaFloor::new(max_coord + 1);
    for vent in vents {
        for (x, y) in vent.get_coords() {
            seafloor.vent_counts[x][y] += 1;
        }
    }

    seafloor
        .vent_counts
        .iter()
        .flatten()
        .map(|v| if *v > 1 { 1 } else { 0 })
        .sum()
}
pub fn part_2_solution(vents: Vec<Vent>) -> usize {
    let max_coord = vents
        .iter()
        .map(|vent| max(max(vent.start.0, vent.start.1), max(vent.end.0, vent.end.1)))
        .max()
        .unwrap();

    let mut seafloor = SeaFloor::new(max_coord + 1);
    for vent in vents {
        for (x, y) in vent.get_coords() {
            seafloor.vent_counts[x][y] += 1;
        }
    }

    seafloor
        .vent_counts
        .iter()
        .flatten()
        .map(|v| if *v > 1 { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn example_part_1() {
        let vents = read_to_one_per_line::<Vent>(P.example_path("_1"))
            .ok()
            .unwrap();
        let expected = 5;
        assert_eq!(expected, part_1_solution(vents));
    }

    #[test]
    fn input_part_1() {
        let vents = read_to_one_per_line::<Vent>(P.input_path()).ok().unwrap();
        let expected = 4421;
        assert_eq!(expected, part_1_solution(vents));
    }

    #[test]
    fn example_part_2() {
        let vents = read_to_one_per_line::<Vent>(P.example_path("_1"))
            .ok()
            .unwrap();
        let expected = 12;
        assert_eq!(expected, part_2_solution(vents));
    }

    #[test]
    fn input_part_2() {
        let vents = read_to_one_per_line::<Vent>(P.input_path()).ok().unwrap();
        let expected = 18674;
        assert_eq!(expected, part_2_solution(vents));
    }
}
