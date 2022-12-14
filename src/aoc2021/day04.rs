/// problem: https://adventofcode.com/2021/day/4
/// input: "https://adventofcode.com/2021/day/4/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 4,
    name: "Giant Squid",
};

use std::{collections::HashSet, str::Lines};

pub struct Board {
    rows: Vec<HashSet<usize>>,
    cols: Vec<HashSet<usize>>,
    finished: bool,
}

impl Board {
    pub fn new(lines: &mut Lines) -> Self {
        let mut rows: Vec<HashSet<usize>> = vec![HashSet::new(); 5];
        let mut cols: Vec<HashSet<usize>> = vec![HashSet::new(); 5];
        for (row, line) in lines.take(5).enumerate() {
            assert!(line != "");
            for (col, val) in line
                .split_whitespace()
                .map(|v| v.parse::<usize>().expect("should be number"))
                .enumerate()
            {
                rows.get_mut(row).unwrap().insert(val);
                cols.get_mut(col).unwrap().insert(val);
            }
        }

        Self {
            rows,
            cols,
            finished: false,
        }
    }

    fn process(&mut self, value: usize) -> Option<usize> {
        for row in self.rows.iter_mut() {
            if row.remove(&value) {
                if row.is_empty() {
                    self.finished = true;
                    let prod = value * self.rows.iter().flatten().sum::<usize>();
                    return Some(prod);
                }
            }
        }
        for col in self.cols.iter_mut() {
            if col.remove(&value) && col.is_empty() {
                self.finished = true;
                let prod = value * self.cols.iter().flatten().sum::<usize>();
                return Some(prod);
            }
        }
        None
    }
}

pub fn part_1_solution(moves: Vec<usize>, boards: Vec<Board>) -> usize {
    let mut boards = boards;
    for m in moves {
        for board in boards.iter_mut() {
            match board.process(m) {
                Some(result) => return result,
                None => {}
            }
        }
    }
    0
}

pub fn part_2_solution(moves: Vec<usize>, boards: Vec<Board>) -> usize {
    let mut boards = boards;
    let mut last_score = 0;
    for m in moves {
        for board in boards.iter_mut() {
            match board.process(m) {
                // unable to use retain as the closure takes &T not &mut T
                Some(result) => {
                    last_score = result;
                }
                None => {}
            };
        }
        boards.retain(|board| !board.finished);
    }
    last_score
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::fs::read_to_string;

    fn read_day4(path: impl AsRef<std::path::Path>) -> Result<(Vec<usize>, Vec<Board>)> {
        let file = read_to_string(path)?; // need to create this binding as lines takes a reference to a string
        let mut lines = file.lines();
        let moves = match lines.next() {
            Some(s) => s
                .split(",")
                .map(|m| m.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            None => panic!("moves should be first line"),
        };

        let mut boards: Vec<Board> = Vec::new();
        while let Some(empty_line) = lines.next() {
            assert_eq!(empty_line, "");
            boards.push(Board::new(&mut lines));
        }
        Ok((moves, boards))
    }

    #[test]
    fn example_part1() {
        let (moves, boards) = read_day4(P.example_path("_1")).unwrap();
        let expected = 4512;
        assert_eq!(expected, part_1_solution(moves, boards));
    }

    #[test]
    fn input_part1() {
        let (moves, boards) = read_day4(P.input_path()).unwrap();
        let expected = 8580;
        assert_eq!(expected, part_1_solution(moves, boards));
    }

    #[test]
    fn example_part2() {
        let (moves, boards) = read_day4(P.example_path("_1")).unwrap();
        let expected = 1924;
        assert_eq!(expected, part_2_solution(moves, boards));
    }

    #[test]
    fn input_part2() {
        let (moves, boards) = read_day4(P.input_path()).unwrap();
        let expected = 9576;
        assert_eq!(expected, part_2_solution(moves, boards));
    }
}
