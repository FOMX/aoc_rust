/// problem: https://adventofcode.com/2021/day/12
/// input: "https://adventofcode.com/2021/day/12/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 12,
    name: "Passage Pathing",
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

pub struct Cave {
    pub name: String,
    pub is_big: bool,
    pub neighbours: HashSet<String>,
}

pub struct Edge {
    pub left: String,
    pub right: String,
}

pub fn add_edge_to_caves(caves: &mut HashMap<String, Cave>, edge: Edge) {
    caves
        .entry(edge.left.to_owned())
        .or_insert(Cave {
            name: edge.left.to_owned(),
            is_big: edge.left.chars().next().unwrap().is_uppercase(),
            neighbours: HashSet::new(),
        })
        .neighbours
        .insert(edge.right.to_owned());
    caves
        .entry(edge.right.to_owned())
        .or_insert(Cave {
            name: edge.right.to_owned(),
            is_big: edge.right.chars().next().unwrap().is_uppercase(),
            neighbours: HashSet::new(),
        })
        .neighbours
        .insert(edge.left.to_owned());
}

impl FromStr for Edge {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').expect("unable to split at - ");
        Ok(Self {
            left: left.to_owned(),
            right: right.to_owned(),
        })
    }
}

pub fn part_1_solution(caves: HashMap<String, Cave>) -> usize {
    // all paths
    // tedious
    let mut complete_paths: HashSet<Vec<String>> = HashSet::new();

    let mut path_stems = VecDeque::new();
    path_stems.push_back(vec!["start".to_owned()]);

    while let Some(curr_path) = path_stems.pop_front() {
        let last_cave = curr_path.last().unwrap();
        if last_cave == "end" {
            complete_paths.insert(curr_path);
            continue;
        }

        for neighbour in caves[last_cave].neighbours.iter() {
            if caves[neighbour].is_big | !curr_path.contains(neighbour) {
                let mut new_path = curr_path.clone();
                new_path.push(neighbour.to_owned());
                path_stems.push_back(new_path)
            }
        }
    }
    complete_paths.len()
}

struct Path {
    path: Vec<String>,
    has_visited_small_twice: bool,
}

pub fn part_2_solution(caves: HashMap<String, Cave>) -> usize {
    // all paths
    // tedious
    let mut complete_paths: HashSet<Vec<String>> = HashSet::new();

    let mut path_stems = VecDeque::new();
    path_stems.push_back(Path {
        path: vec!["start".to_owned()],
        has_visited_small_twice: false,
    });
    // println!("{:?}", path_stems);
    while let Some(curr_path) = path_stems.pop_front() {
        let last_cave = curr_path.path.last().unwrap();
        if last_cave == "end" {
            complete_paths.insert(curr_path.path);
            continue;
        }

        for neighbour in caves[last_cave].neighbours.iter() {
            if neighbour != "start" {
                let mut has_visited_small_twice = curr_path.has_visited_small_twice;
                if !caves[neighbour].is_big && curr_path.path.contains(neighbour) {
                    if has_visited_small_twice {
                        continue;
                    }
                    has_visited_small_twice = true;
                }

                let mut new_path = curr_path.path.clone();
                new_path.push(neighbour.to_owned());
                path_stems.push_back(Path {
                    path: new_path,
                    has_visited_small_twice: has_visited_small_twice,
                })
            }
        }
    }
    complete_paths.len()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn example_part_1() {
        let edges: Vec<Edge> = read_to_one_per_line::<Edge>(P.example_path("_1")).unwrap();
        let mut caves: HashMap<String, Cave> = HashMap::new();
        for edge in edges {
            add_edge_to_caves(&mut caves, edge);
        }

        let expected = 10;
        assert_eq!(expected, part_1_solution(caves))
    }

    #[test]
    fn input_part_1() {
        let edges: Vec<Edge> = read_to_one_per_line::<Edge>(P.input_path()).unwrap();
        let mut caves: HashMap<String, Cave> = HashMap::new();
        for edge in edges {
            add_edge_to_caves(&mut caves, edge);
        }

        let expected = 3679;
        assert_eq!(expected, part_1_solution(caves))
    }

    #[test]
    fn example_part_2() {
        let edges: Vec<Edge> = read_to_one_per_line::<Edge>(P.example_path("_1")).unwrap();
        let mut caves: HashMap<String, Cave> = HashMap::new();
        for edge in edges {
            add_edge_to_caves(&mut caves, edge);
        }

        let expected = 36;
        assert_eq!(expected, part_2_solution(caves))
    }

    #[test]
    fn input_part_2() {
        let edges: Vec<Edge> = read_to_one_per_line::<Edge>(P.input_path()).unwrap();
        let mut caves: HashMap<String, Cave> = HashMap::new();
        for edge in edges {
            add_edge_to_caves(&mut caves, edge);
        }

        let expected = 107395;
        assert_eq!(expected, part_2_solution(caves))
    }
}
