
use crate::Problem;
/// problem: https://adventofcode.com/2022/day/14
/// input: https://adventofcode.com/2022/day/14/input

const P: Problem = Problem {
    year: 2022,
    day: 14,
    name: "TODO",
};
use std::{
    cmp::{max, min},
    str::FromStr,
};

#[derive(Debug)]
pub struct RockPath(Vec<[usize; 2]>);

impl FromStr for RockPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(Self(
            s.split(" -> ")
                .map(|p| {
                    let (s, f) = p.split_once(',').unwrap();
                    [s.parse::<usize>().unwrap(), f.parse::<usize>().unwrap()]
                })
                .collect(),
        ))
    }
}

fn drop_sand(rocks: &mut Vec<Vec<bool>>, max_depth: usize) -> bool {
    let mut s = (500, 0);
    while s.1 < max_depth {
        if !rocks[s.0][s.1 + 1] {
            s.1 += 1;
            continue;
        } else if !rocks[s.0 - 1][s.1 + 1] {
            s.1 += 1;
            s.0 -= 1;
            continue;
        } else if !rocks[s.0 + 1][s.1 + 1] {
            s.1 += 1;
            s.0 += 1;
            continue;
        } else if !rocks[s.0][s.1] && rocks[s.0][s.1 + 1] {
            rocks[s.0][s.1] = true;
            return true;
        } else if rocks[s.0][s.1] {
            return false;
        }
    }
    return false;
}
fn pt1(input: &str) -> usize {
    let rockpaths = parse(input);
    let mut rocks = vec![vec![false; 1000]; 1000];
    let mut max_depth = 0;
    for rockpath in rockpaths {
        for window in rockpath.0.windows(2) {
            let (x1, x2, y1, y2) = (
                min(window[0][0], window[1][0]),
                max(window[0][0], window[1][0]),
                min(window[0][1], window[1][1]),
                max(window[0][1], window[1][1]),
            );
            max_depth = max(max_depth, y2);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    rocks[x][y] = true;
                }
            }
        }
    }

    let mut i = 0;
    while drop_sand(&mut rocks, max_depth) {
        i += 1;
    }
    i
}

fn pt2(input: &str) -> usize {
    let rockpaths = parse(input);
    let mut rocks = vec![vec![false; 1000]; 1000];
    let mut max_depth = 0;
    for rockpath in rockpaths {
        for window in rockpath.0.windows(2) {
            let (x1, x2, y1, y2) = (
                min(window[0][0], window[1][0]),
                max(window[0][0], window[1][0]),
                min(window[0][1], window[1][1]),
                max(window[0][1], window[1][1]),
            );
            max_depth = max(max_depth, y2);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    rocks[x][y] = true;
                }
            }
        }
    }
    max_depth += 2;
    for x in 0..1000 {
        rocks[x][max_depth] = true;
    }

    let mut i = 0;
    while drop_sand(&mut rocks, max_depth +1) {
        i += 1;
    }
    i
}

fn parse(input: &str) -> Vec<RockPath> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn pt1_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt1(&input), 24);
    }

    #[test]
    fn pt1_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt1(&input), 674);
    }

    #[test]
    fn pt2_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt2(&input), 93);
    }

    #[test]
    fn pt2_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt2(&input), 24958);
    }
}
