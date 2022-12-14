use itertools::Itertools;

use crate::Problem;
use std::str::{Chars, FromStr};
/// problem: https://adventofcode.com/2022/day/13
/// input: https://adventofcode.com/2022/day/13/input

const P: Problem = Problem {
    year: 2022,
    day: 13,
    name: "Distress Signal",
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    List(Vec<Node>),
    Val(usize),
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let cs = &mut s.chars();
        cs.next(); // burn first [
        Node::from_chars(cs)
    }
}
impl Node {
    fn from_chars(chars: &mut Chars) -> anyhow::Result<Self> {
        // first char
        let mut children = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                ']' => break, // end of chars
                '[' => children.push(Node::from_chars(chars)?),
                token if token.is_ascii_digit() => {
                    let mut v = String::from(token);
                    for ch in chars.take_while(|&tok| tok.is_digit(10)) {
                        v.push(ch);
                    }
                    children.push(Node::Val(usize::from_str(&v).unwrap()));
                }
                ',' => {}
                _ => unreachable!(),
            }
        }
        Ok(Node::List(children))
    }
}

#[derive(Debug)]
struct Pair {
    left: Node,
    right: Node,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
                            }
                        }
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Node::Val(l_v), Node::Val(r_v)) => l_v.cmp(r_v),
            (Node::List(l_c), Node::List(r_c)) => {
                for (l, r) in l_c.iter().zip(r_c.iter()) {
                    match l.cmp(r) {
                        std::cmp::Ordering::Equal => {} // continue
                        order => return order,
                        }
                    }
                l_c.len().cmp(&r_c.len())
                        }
            (Node::List(l_c), Node::Val(r_v)) => self.cmp(&Node::List(vec![other.clone()])),
            (Node::Val(l_v), Node::List(r_c)) => Node::List(vec![self.clone()]).cmp(other),
                    }
                        }
                    }

impl Pair {
    fn is_in_order(&self) -> bool {
        self.left <= self.right
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut l = s.lines();
        Ok(Self {
            left: l.next().unwrap().to_owned().parse::<Node>().unwrap(),
            right: l.next().unwrap().to_owned().parse::<Node>().unwrap(),
        })
    }
}

fn pt1(input: &str) -> usize {
    let pairs = parse(input);
    for (i, pair) in pairs.iter().enumerate() {
        println!("{} {}, ", i, pair.is_in_order());
    }
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| if p.is_in_order() { Some(i + 1) } else { None })
        .sum()
}

fn pt2(input: &str) -> usize {
    let mut pairs = parse_pt2(input);
    let dividers: Vec<Node> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    pairs.extend(dividers.clone());
    pairs
        .iter()
        .sorted()
        .positions(|x| dividers.contains(&x))
        .fold(1, |acc, x| acc * (x + 1))
}

fn parse(input: &str) -> Vec<Pair> {
    input
        .replace('\r', "")
        .split("\n\n")
        .map(|l| l.parse().unwrap())
        .collect()
}

fn parse_pt2(input: &str) -> Vec<Node> {
    input
        .replace('\r', "")
        .replace("\n\n", "\n")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn pt1_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt1(&input), 13);
    }
    
    #[test]
    fn pt1_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt1(&input), 6544); // too low 7592 too high
    }

    #[test]
    fn pt2_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt2(&input), 140);
    }

    #[test]
    fn pt2_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt2(&input), 19493);
    }
}
