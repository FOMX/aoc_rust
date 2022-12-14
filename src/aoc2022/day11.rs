/// problem: https://adventofcode.com/2022/day/11
/// input: https://adventofcode.com/2022/day/11/input
use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 11,
    name: "TODO",
};

#[derive(Debug, Clone)]
pub enum Op {
    Mult(isize),
    Square,
    Add(isize),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<isize>,
    pub operation: Op,
    pub divisor: isize,
    pub true_monkey: usize,
    pub false_monkey: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .unwrap()
            .chars()
            .nth(7)
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let items_str = lines.next().unwrap().replace("  Starting items: ", "");
        let items = items_str
            .split(", ")
            .map(|i| isize::from_str(i).unwrap())
            .collect();
        let op_str = lines
            .next()
            .unwrap()
            .replace(' ', "")
            .replace("Operation:new=old", "");
        let operation = if op_str == String::from("*old") {
            Op::Square
        } else {
            let cs = op_str.clone().chars().collect_vec();
            let o = cs[0];
            let num = cs[1..].to_owned().iter().collect::<String>();
            match o {
                '*' => Op::Mult(isize::from_str(&num)?),
                '+' => Op::Add(isize::from_str(&num)?),
                '-' => Op::Add(-isize::from_str(&num)?),
                _ => {
                    unreachable!()
                }
            }
        };

        let divisor = isize::from_str(lines.next().unwrap().split(' ').last().unwrap())?;
        let true_monkey = usize::from_str(lines.next().unwrap().split(' ').last().unwrap())?;
        let false_monkey = usize::from_str(lines.next().unwrap().split(' ').last().unwrap())?;

        Ok(Monkey {
            id,
            items,
            operation,
            divisor,
            true_monkey,
            false_monkey,
        })
    }
}

fn pt1(input: &str) -> usize {
    let mut monkeys = parse(input);
    println!("{:?}", monkeys);
    let mut inspects = vec![0; monkeys.len()];
    for round in 0..20 {
        for i in 0..monkeys.len() {
            let mut monk = monkeys[i].clone();
            for mut item in monk.items {
                inspects[i] += 1;
                item = match monk.operation {
                    Op::Mult(v) => item * v,
                    Op::Square => item.pow(2),
                    Op::Add(v) => item + v,
                };
                item = item / 3;
                if item % monk.divisor == 0 {
                    monkeys[monk.true_monkey].items.push(item);
                } else {
                    monkeys[monk.false_monkey].items.push(item);
                }
            }
            monkeys[i].items = Vec::new();
        }
    }
    inspects.sort();
    inspects.reverse();
    println!("{:?}", inspects);
    inspects[0] * inspects[1]
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: isize, b: isize) -> isize {
    a * b / gcd(a, b)
}

fn pt2(input: &str) -> usize {
    let mut monkeys = parse(input);
    let bigdivisor = monkeys
        .iter()
        .map(|monk| monk.divisor)
        .reduce(|a, b| lcm(a, b))
        .unwrap();
    let mut inspects = vec![0; monkeys.len()];
    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monk = monkeys[i].clone();
            for mut item in monk.items {
                inspects[i] += 1;
                item = match monk.operation {
                    Op::Mult(v) => item * v,
                    Op::Square => item.pow(2),
                    Op::Add(v) => item + v,
                };
                item %= bigdivisor;
                if item % monk.divisor == 0 {
                    monkeys[monk.true_monkey].items.push(item);
                } else {
                    monkeys[monk.false_monkey].items.push(item);
                }
            }
            monkeys[i].items = Vec::new();
        }
    }
    inspects.sort();
    inspects.reverse();
    println!("{:?}", inspects);
    inspects[0] * inspects[1]
}

fn parse(input: &str) -> Vec<Monkey> {
    let ls = input.split("\r\n\r\n").into_iter();
    ls.map(|ls| ls.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn pt1_example() {
        let input = read_to_string(P.example_path("_1")).unwrap();
        let expected = 10605;
        assert_eq!(pt1(&input), expected);
    }

    #[test]
    fn pt1_input() {
        let input = read_to_string(P.input_path()).unwrap();
        let expected = 57838;
        assert_eq!(pt1(&input), expected);
    }

    #[test]
    fn pt2_example() {
        let input = read_to_string(P.example_path("_1")).unwrap();
        let expected = 2713310158;
        assert_eq!(pt2(&input), expected);
    }

    #[test]
    fn pt2_input() {
        let input = read_to_string(P.input_path()).unwrap();
        let expected = 15050382231;
        assert_eq!(pt2(&input), expected);
    }
}
