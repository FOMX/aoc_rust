use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

use crate::Problem;
/// problem: https://adventofcode.com/2022/day/15
/// input: https://adventofcode.com/2022/day/15/input

const P: Problem = Problem {
    year: 2022,
    day: 15,
    name: "TODO",
};

#[derive(Debug)]
pub struct Sensor {
    origin: (isize, isize),
    beacon: (isize, isize),
    range: usize,
}
impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        //Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let (origin, beacon) = s
            .trim()
            .replace("Sensor at x=", "")
            .replace(", y=", ",")
            .replace(": closest beacon is at x=", "|")
            .split('|')
            .map(|coords| {
                coords
                    .split(',')
                    .map(|n| n.parse().expect("Should be usize"))
                    .collect_tuple::<(isize, isize)>()
                    .expect("must be 2")
            })
            .collect_tuple::<(_, _)>()
            .expect("must be 2");

        Ok(Self {
            origin: origin,
            beacon: beacon,
            range: Self::manhattan(origin, beacon),
        })
    }
}

impl Sensor {
    fn manhattan(p1: (isize, isize), p2: (isize, isize)) -> usize {
        ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as usize
    }

    fn is_in_range(&self, p2: (isize, isize)) -> bool {
        Self::manhattan(self.origin, p2) <= self.range && p2 != self.beacon
    }

    fn out_of_range(&self, p2: (isize, isize)) -> bool {
        Self::manhattan(self.origin, p2) > self.range
    }
}

fn pt1(input: &str, target: isize) -> usize {
    // TODO: speed up lol
    let sensors = parse(input);
    let mut count = 0;
    for x in -10000000..10000000 {
        if sensors.iter().any(|sensor| sensor.is_in_range((x, target))) {
            count += 1;
        }
    }
    count
}

trait InclusiveRangeExt {
    // TODO impl merge
    fn contains_range(&self, other: &Self) -> bool;

    fn overlaps(&self, other: &Self) -> bool;

    fn is_adjacent(&self, other: &Self) -> bool;
}

impl InclusiveRangeExt for RangeInclusive<isize> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        self.start() == &(other.end() + 1) || self.end() == &(other.start() - 1)
    }
}

fn pt2(input: &str, x_range: RangeInclusive<isize>, y_range: RangeInclusive<isize>) -> isize {
    // TODO: speed up lol
    let sensors = parse(input);
    let mut beacon = (0, 0);
    let x_iter = x_range.collect::<Vec<isize>>();
    let y_iter = y_range.collect::<Vec<isize>>();

    for &x in x_iter.iter() {
        let mut ys: Vec<RangeInclusive<isize>> = Vec::new();
        for sensor in sensors.iter() {
            if sensor.is_in_range((x, sensor.origin.1)) {
                let delta = sensor.range as isize - (sensor.origin.0 - x).abs();
                let low = (sensor.origin.1 - delta);
                let high = (sensor.origin.1 + delta);
                ys.push(low..=high)
            }
        }
        ys.sort_by(|a, b| a.start().cmp(&b.start()));
        ys = ys.iter().fold(
            Vec::new(),
            |mut acc: Vec<RangeInclusive<isize>>, next_range| {
                if let Some(last) = acc.last() {
                    if last.contains_range(&next_range) {
                        // drop next_range
                    } else if last.overlaps(&next_range) || last.is_adjacent(&next_range) {
                        let last = acc.pop().unwrap();
                        acc.push(*last.start()..=*next_range.end());
                    } else {
                        acc.push(next_range.clone())
                    }
                } else {
                    acc.push(next_range.clone())
                }
                acc
            },
        );

        if ys.len() > 1 {
            println!("{x} {:?}", ys);
            beacon = (x, ys[0].end() + 1);
            println!("{:?}", beacon);
            break;
        }
    }
    
    println!("{:?}", beacon);
    return beacon.0 * 4000000 + beacon.1;
}

fn parse(input: &str) -> Vec<Sensor> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        let parsed = parse(&input);
        for p in parsed {
            println!("{:?}", p);
        }
    }

    #[test]
    fn pt1_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        let target = 10;
        assert_eq!(pt1(&input, target), 26);
    }

    #[test]
    fn pt1_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        let target = 2000000;
        assert_eq!(pt1(&input, target), 5878678); // 5878678
    }

    #[test]
    fn pt2_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt2(&input, 0..=20, 0..=20), 56000011);
    }

    #[test]
    fn pt2_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt2(&input, 0..=4000000, 0..=4000000), 11796491041245);
    }
}
