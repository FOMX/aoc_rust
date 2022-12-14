/// problem: https://adventofcode.com/2021/day/13
/// input: "https://adventofcode.com/2021/day/13/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 13,
    name: "Transparent Origami",
};
use anyhow::Result;
use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug)]
pub struct Move {
    pub axis: Axis,
    pub pivot: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (ax, co) = s
            .split(" ")
            .last()
            .expect("unable to split at ' '")
            .split_once('=')
            .expect("unable to split at '='");
        let axis = match ax {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("incorrect axis observed"),
        };
        let coordinate: usize = co.parse()?;
        Ok(Self {
            axis,
            pivot: coordinate,
        })
    }
}

pub struct Board {
    pieces: HashSet<(usize, usize)>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut max_x, mut max_y) = (0, 0);
        let mut pieces: Vec<(usize, usize)> = Vec::new();
        for (piece_x, piece_y) in self.pieces.iter() {
            max_x = max_x.max(*piece_x);
            max_y = max_y.max(*piece_y);
            pieces.push((*piece_x, *piece_y));
        }
        let mut display_pieces: Vec<Vec<char>> = Vec::new();
        for _row in 0..=max_y {
            display_pieces.push(Vec::new());
            for _col in 0..=max_x {
                display_pieces.last_mut().unwrap().push('.');
            }
        }
        for piece in pieces {
            display_pieces[piece.1][piece.0] = '#';
        }

        let mut s = "".to_owned();
        for row in display_pieces {
            for col in row {
                s.push_str(format!("{}", col).as_str());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Board {
    fn fold_up(&mut self, m: Move) {
        let pivot = m.pivot;
        match m.axis {
            Axis::X => {
                // TODO: drain_filter when it becomes available
                let pieces_to_foldup: Vec<(usize, usize)> = self
                    .pieces
                    .clone()
                    .into_iter()
                    .filter(|p| p.0 > pivot)
                    .collect();
                for piece in pieces_to_foldup {
                    self.pieces.remove(&piece);
                    let delta = piece.0 - pivot;
                    if delta <= pivot {
                        self.pieces.insert((pivot - delta, piece.1));
                    }
                }
            }
            Axis::Y => {
                // TODO: drain_filter when it becomes available
                let pieces_to_foldup: Vec<(usize, usize)> = self
                    .pieces
                    .clone()
                    .into_iter()
                    .filter(|p| p.1 > pivot)
                    .collect();
                for piece in pieces_to_foldup {
                    self.pieces.remove(&piece);
                    let delta = piece.1 - pivot;
                    if delta <= pivot {
                        self.pieces.insert((piece.0, pivot - delta));
                    }
                }
            }
        }
    }
}

pub fn part_1_solution(board: Board, moves: Vec<Move>) -> usize {
    let mut board = board;
    for m in moves.into_iter().take(1) {
        board.fold_up(m);
    }
    board.pieces.len()
}

pub fn part_2_solution(board: Board, moves: Vec<Move>) -> Board {
    let mut board = board;
    for m in moves {
        board.fold_up(m);
    }
    board
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::fs::read_to_string;

    fn read_day13(path: impl AsRef<std::path::Path>) -> Result<(Board, Vec<Move>)> {
        let file = read_to_string(path)?; // need to create this binding as lines takes a reference to a string
        let mut lines = file.lines();

        let mut board = Board {
            pieces: HashSet::new(),
        };
        while let Some(next_line) = lines.next() {
            if next_line.is_empty() {
                break;
            }
            let (row, col) = next_line.split_once(",").expect("unable to split at , ");
            board.pieces.insert((row.parse()?, col.parse()?));
        }

        let mut moves: Vec<Move> = Vec::new();
        while let Some(m) = lines.next() {
            moves.push(m.parse()?);
        }

        Ok((board, moves))
    }
    #[test]
    fn example_part_1() {
        let (board, moves) = read_day13(P.example_path("_1")).unwrap();
        let expected = 17;
        assert_eq!(expected, part_1_solution(board, moves))
    }

    #[test]
    fn input_part_1() {
        let (board, moves) = read_day13(P.input_path()).unwrap();
        let expected = 770;
        assert_eq!(expected, part_1_solution(board, moves))
    }

    #[test]
    fn example_part_2() {
        let (board, moves) = read_day13(P.example_path("_1")).unwrap();
        let board = part_2_solution(board, moves);
        println!("{}", board);
    }

    #[test]
    fn input_part_2() {
        let (board, moves) = read_day13(P.input_path()).unwrap();
        let board = part_2_solution(board, moves);
        println!("{}", board);
    }
}
