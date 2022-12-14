/// problem: https://adventofcode.com/2022/day/7
/// input: https://adventofcode.com/2022/day/7/input
///
use std::{collections::HashMap, str::FromStr};

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 7,
    name: "No Space Left On Device",
};

#[derive(Debug, Clone)]
pub struct File {
    size: usize,
    _name: String,
}

pub enum Command {
    ChangeDir(String),
    ListDir,
    MakeFile(File),
    MakeDir(String),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut splits = s.split(' ');
        let res = match splits.next().unwrap() {
            "$" => {
                // cmd
                match splits.next().unwrap() {
                    "ls" => Command::ListDir,
                    "cd" => Command::ChangeDir(splits.next().unwrap().to_owned()),
                    _ => panic!(),
                }
            }
            "dir" => Command::MakeDir(splits.next().unwrap().to_owned()),
            size => Command::MakeFile(File {
                size: size.parse()?,
                _name: splits.next().unwrap().to_owned(),
            }),
        };

        Ok(res)
    }
}

pub fn part_1_solution(commands: Vec<Command>) -> usize {
    let mut paths: HashMap<Vec<String>, usize> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();
    for command in commands {
        match command {
            Command::ChangeDir(dir) => {
                if dir == ".." {
                    current_path.pop();
                } else {
                    current_path.push(dir);
                }
            }
            Command::ListDir => {}
            Command::MakeFile(file) => {
                let mut cp = current_path.clone();
                while !cp.is_empty() {
                    *paths.entry(cp.clone()).or_default() += file.size;
                    cp.pop();
                }
            }
            Command::MakeDir(_) => {}
        }
    }
    paths
        .values()
        .into_iter()
        .map(|size| if size <= &100000 { size } else { &0 })
        .sum()
}
pub fn part_2_solution(commands: Vec<Command>) -> usize {
    let mut paths: HashMap<Vec<String>, usize> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();
    for command in commands {
        match command {
            Command::ChangeDir(dir) => {
                if dir == ".." {
                    current_path.pop();
                } else {
                    current_path.push(dir);
                }
            }
            Command::ListDir => {}
            Command::MakeFile(file) => {
                let mut cp = current_path.clone();
                while !cp.is_empty() {
                    *paths.entry(cp.clone()).or_default() += file.size;
                    cp.pop();
                }
            }
            Command::MakeDir(_) => {}
        }
    }
    let target_space = 30000000;
    let total_allocation = 70000000;
    let home = vec!["/".to_owned()];
    let total_used = paths.get(&home).unwrap();
    let required_additional_space = target_space - (total_allocation - total_used);
    *paths
        .values()
        .into_iter()
        .filter(|&&size| size >= required_additional_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let commands = read_to_one_per_line::<Command>(P.example_path("_1")).unwrap();
        let expected = 95437;
        assert_eq!(expected, part_1_solution(commands))
    }

    #[test]
    fn input_part_1() {
        let commands = read_to_one_per_line::<Command>(P.input_path()).unwrap();
        let expected = 2104783;
        assert_eq!(expected, part_1_solution(commands))
    }

    #[test]
    fn example_part_2() {
        let commands = read_to_one_per_line::<Command>(P.example_path("_1")).unwrap();
        let expected = 24933642;
        assert_eq!(expected, part_2_solution(commands))
    }

    #[test]
    fn input_part_2() {
        let commands = read_to_one_per_line::<Command>(P.input_path()).unwrap();
        let expected = 5883165;
        assert_eq!(expected, part_2_solution(commands))
    }
}
