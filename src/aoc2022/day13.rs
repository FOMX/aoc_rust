use crate::Problem;
use std::str::{Chars, FromStr};
/// problem: https://adventofcode.com/2022/day/13
/// input: https://adventofcode.com/2022/day/13/input

const P: Problem = Problem {
    year: 2022,
    day: 13,
    name: "Distress Signal",
};

#[derive(Debug, Clone)]
pub enum Node {
    ListNode(ListNode),
    Val(usize),
}

#[derive(Debug, Clone)]
pub struct ListNode {
    pub children: Vec<Node>,
}

impl FromStr for ListNode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        ListNode::from_chars(&mut s.chars())
    }
}
impl ListNode {
    fn from_chars(chars: &mut Chars) -> anyhow::Result<Self> {
        // first char
        let mut children = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                ']' => break, // end of chars
                '[' => children.push(Node::ListNode(ListNode::from_chars(chars)?)),
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
        Ok(Self { children })
    }
}

#[derive(Debug)]
struct Pair {
    left: ListNode,
    right: ListNode,
}

impl Pair {
    fn is_in_order(&self) -> bool {
        fn helper(left: &Node, right: &Node) -> bool {
            match left {
                Node::ListNode(l_c) => match right {
                    Node::ListNode(r_c) => {
                        for (l, r) in l_c.children.iter().zip(&r_c.children) {
                            if helper(l, r) {
                                return true;
                            }
                        }
                        if l_c.children.len() > r_c.children.len() {
                            false
                        } else {
                            true
                        }
                    }
                    Node::Val(r_v) => {
                        if helper(&l_c.children[0], &Node::Val(*r_v)) {
                            true
                        } else if l_c.children.len() > 1 {
                            false
                        } else {
                            true
                        }
                    }
                },
                Node::Val(l_v) => match right {
                    Node::ListNode(r_c) => {
                        if r_c.children.len() < 1 {
                            false
                        } else {
                            helper(&r_c.children[0], &Node::Val(*l_v))
                        }
                    }
                    Node::Val(r_v) => l_v <= r_v,
                },
            }
        }
        helper(
            &Node::ListNode(self.left.clone()),
            &Node::ListNode(self.right.clone()),
        )
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut l = s.lines();
        Ok(Self {
            left: l.next().unwrap().to_owned().parse::<ListNode>().unwrap(),
            right: l.next().unwrap().to_owned().parse::<ListNode>().unwrap(),
        })
    }
}

fn pt1(input: &str) -> usize {
    let pairs = parse(input);
    println!("{:?}", pairs);
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| if p.is_in_order() { Some(i) } else { None })
        .sum()
}

fn pt2(input: &str) -> usize {
    let parsed = parse(input);
    todo!()
}

fn parse(input: &str) -> Vec<Pair> {
    input
        .replace('\r', "")
        .split("\n\n")
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    // solution in local not available
    // #[test]
    // fn pt1_example() {
    //     let input = read_to_string(P.example_path("_1")).expect("no such file");
    //     assert_eq!(pt1(&input), 13);
    // }

    // #[test]
    // fn pt1_input() {
    //     let input = read_to_string(P.input_path()).expect("no such file");
    //     assert_eq!(pt1(&input), 0);
    // }

    // #[test]
    // fn pt2_example() {
    //     let input = read_to_string(P.example_path("_1")).expect("no such file");
    //     assert_eq!(pt2(&input), 0);
    // }

    // #[test]
    // fn pt2_input() {
    //     let input = read_to_string(P.input_path()).expect("no such file");
    //     assert_eq!(pt2(&input), 0);
    // }
}
