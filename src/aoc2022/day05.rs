/// problem: https://adventofcode.com/2022/day/5
/// input: https://adventofcode.com/2022/day/5/input
use std::{
    collections::VecDeque,
    str::{FromStr, Lines},
};

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 5,
    name: "Elf Stacks",
};

/*
*/
#[derive(Debug)]
pub struct Action {
    count: usize,
    from: usize,
    to: usize,
}
impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        //move 1 from 2 to 1
        let mut splits = s.split(' ');
        splits.next();
        let count = splits.next().unwrap().parse::<usize>().unwrap();
        splits.next();
        let from = splits.next().unwrap().parse::<usize>().unwrap();
        splits.next();
        let to = splits.next().unwrap().parse::<usize>().unwrap();
        Ok(Action { count, from, to })
    }
}
#[derive(Debug)]
pub struct Stacks(Vec<Vec<char>>);
impl Stacks {
    pub fn from_lines(lines: &mut Lines) -> Self {
        // [W] [V]     [P]
        // [B] [T]     [C] [B]     [G]
        // [G] [S]     [V] [H] [N] [T]
        // [Z] [B] [W] [J] [D] [M] [S]
        // [R] [C] [N] [N] [F] [W] [C]     [W]
        // [D] [F] [S] [M] [L] [T] [L] [Z] [Z]
        // [C] [W] [B] [G] [S] [V] [F] [D] [N]
        // [V] [G] [C] [Q] [T] [J] [P] [B] [M]
        //  1   2   3   4   5   6   7   8   9

        let mut vs: Vec<Vec<char>> = Vec::new();

        for ln in lines.by_ref() {
            // this will move the &mut lines
            let mut cs = ln.chars();
            cs.next();
            if cs.next().expect("the blank row should be skipped") == '1' {
                break;
            }

            let cleaned = ln
                .chars() // [W] [V]     [P]
                .collect::<Vec<char>>() //  ^   ^   ^   ^
                .chunks(4) // 0123012301230123
                .map(|chunk| chunk[1]) // the target value is at ind 1
                .collect::<Vec<char>>(); // W,V, ,P

            for (stack_ind, char) in cleaned.iter().enumerate() {
                match char {
                    ' ' => match vs.get(stack_ind) {
                        Some(_) => {} // do nothing
                        None => vs.insert(stack_ind, Vec::new()),
                    },
                    c => match vs.get_mut(stack_ind) {
                        Some(v) => v.push(*c), // do nothing
                        None => vs.insert(stack_ind, vec![*c]),
                    },
                }
            }
        }
        for v in vs.iter_mut() {
            v.reverse();
        }
        Self(vs)
    }
}

pub fn part_1_solution(mut stacks: Stacks, actions: Vec<Action>) -> String {
    for action in actions {
        for _ in 0..action.count {
            let piece = stacks.0[action.from - 1].pop();
            if let Some(c) = piece {
                stacks.0[action.to - 1].push(c)
            };
        }
    }
    stacks
        .0
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

pub fn part_2_solution(mut stacks: Stacks, actions: Vec<Action>) -> String {
    for action in actions {
        let mut pieces = VecDeque::new();
        for _ in 0..action.count {
            let piece = stacks.0[action.from - 1].pop();
            if let Some(c) = piece {
                pieces.push_front(c)
            };
        }
        for piece in pieces {
            stacks.0[action.to - 1].push(piece)
        }
    }
    stacks
        .0
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
#[allow(unused)]
mod test {

    use super::*;
    use crate::{read_to_one_per_line, read_to_vec_per_line};
    use std::{fs::read_to_string, path::Path};

    pub fn read_day_5(path: impl AsRef<Path>) -> (Stacks, Vec<Action>) {
        let file = read_to_string(path).unwrap();
        let mut lines = file.lines();

        let stacks = Stacks::from_lines(&mut lines);
        match lines.next() {
            Some(ln) => assert!(ln.is_empty()),
            None => unreachable!(),
        }

        let mut actions = Vec::new();
        for line in lines {
            actions.push(Action::from_str(line).unwrap());
        }
        (stacks, actions)
    }

    // #[test]
    // fn test() {
    //     let splits = read_to_chunks(P.get_example_path("_1")).unwrap();
    //     println!("{:?}", splits);
    // }

    #[test]
    fn example_part_1() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let (stacks, actions) = read_day_5(P.example_path("_1"));

        let expected = "CMZ".to_owned();
        assert_eq!(expected, part_1_solution(stacks, actions))
    }

    #[test]
    fn input_part_1() {
        let (stacks, actions) = read_day_5(P.input_path());
        let expected = "TBVFVDZPN".to_owned();
        assert_eq!(expected, part_1_solution(stacks, actions))
    }

    #[test]
    fn example_part_2() {
        let (stacks, actions) = read_day_5(P.example_path("_1"));
        let expected = "MCD".to_owned();
        assert_eq!(expected, part_2_solution(stacks, actions))
    }

    #[test]
    fn input_part_2() {
        let (stacks, actions) = read_day_5(P.input_path());
        let expected = "VLCWHTDSZ".to_owned();
        assert_eq!(expected, part_2_solution(stacks, actions))
    }
}
