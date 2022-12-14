/// problem: https://adventofcode.com/2021/day/18
/// input: "https://adventofcode.com/2021/day/18/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 18,
    name: "Snailfish",
};

use itertools::Itertools;
use std::str::{Chars, FromStr};

#[derive(Debug, Clone)]
pub enum SnailFish {
    Val(usize),
    Nested(Box<SnailFishNumber>),
}

#[derive(Debug, Clone)]
pub struct SnailFishNumber {
    pub left: SnailFish,
    pub right: SnailFish,
}

pub enum Exploded {
    Left(usize),
    Right(usize),
    Me((usize, usize)),
    None, // exploded and used values
}
impl SnailFishNumber {
    fn from_chars(chars: &mut Chars) -> anyhow::Result<Self> {
        // left
        let left = match chars.next().expect("unexpected end of string") {
            '[' => {
                // new snailfishnumber
                SnailFish::Nested(Box::new(SnailFishNumber::from_chars(chars)?))
            }
            token if token.is_ascii_digit() => SnailFish::Val(token.to_digit(10).unwrap() as usize),
            c => panic!("un expected char {c}"),
        };

        // this token MUST be a comma
        assert_eq!(',', chars.next().expect("expecting a comma"));

        // right
        let right = match chars.next().expect("unexpected end of string") {
            '[' => {
                // new snailfishnumber
                SnailFish::Nested(Box::new(SnailFishNumber::from_chars(chars)?))
            }
            token if token.is_ascii_digit() => SnailFish::Val(token.to_digit(10).unwrap() as usize),
            c => panic!("un expected char {c}"),
        };
        // this must be a ]
        assert_eq!(']', chars.next().expect("expecting a comma"));
        Ok(Self { left, right })
    }

    fn explode(&mut self) {
        fn push_down_helper(snf: &mut SnailFishNumber, exploded: Exploded) {
            match exploded {
                Exploded::Left(e) => match &mut snf.right {
                    SnailFish::Val(v) => *v += e,
                    SnailFish::Nested(nested) => {
                        push_down_helper(nested.as_mut(), exploded);
                    }
                },
                Exploded::Right(e) => match &mut snf.left {
                    SnailFish::Val(v) => *v += e,
                    SnailFish::Nested(nested) => {
                        push_down_helper(nested.as_mut(), exploded);
                    }
                },
                Exploded::Me(_) => unreachable!(),
                Exploded::None => unreachable!(),
            }
        }

        fn helper(sfn: &mut SnailFishNumber, depth: usize) -> Option<Exploded> {
            if depth > 4 {
                // explode
                let exploded_left = match &sfn.left {
                    SnailFish::Val(v) => *v,
                    SnailFish::Nested(_) => {
                        unreachable!("there shouldn't be nested fish this deep")
                    }
                };
                let exploded_right = match &sfn.right {
                    SnailFish::Val(v) => *v,
                    SnailFish::Nested(_) => {
                        unreachable!("there shouldn't be nested fish this deep")
                    }
                };
                return Some(Exploded::Me((exploded_left, exploded_right)));
            }
            // else
            match &mut sfn.left {
                SnailFish::Val(_) => {} // do nothing
                SnailFish::Nested(nested) => {
                    let left = helper(nested.as_mut(), depth + 1);
                    match left {
                        Some(exploded) => match exploded {
                            Exploded::Left(_) => return Some(exploded), // propagate left up ,
                            Exploded::Right(e) => {
                                match &mut sfn.right {
                                    SnailFish::Val(v) => *v += e,
                                    SnailFish::Nested(nested) => {
                                        // push down
                                        push_down_helper(nested.as_mut(), exploded);
                                    }
                                }
                                return Some(Exploded::None);
                            }
                            Exploded::Me((l, r)) => {
                                sfn.left = SnailFish::Val(0);
                                match &mut sfn.right {
                                    SnailFish::Val(v) => *v += r,
                                    SnailFish::Nested(nested) => {
                                        push_down_helper(nested.as_mut(), Exploded::Right(r))
                                    }
                                }
                                return Some(Exploded::Left(l));
                            }
                            Exploded::None => return Some(Exploded::None), // an explosion happened, pop to the top
                        },
                        None => {} // continue no explosion happened
                    }
                }
            }
            match &mut sfn.right {
                SnailFish::Val(_) => {} // do nothing
                SnailFish::Nested(nested) => {
                    let right = helper(nested.as_mut(), depth + 1);
                    match right {
                        Some(exploded) => match exploded {
                            Exploded::Right(_) => return Some(exploded), // propagate left up ,
                            Exploded::Left(e) => {
                                match &mut sfn.left {
                                    SnailFish::Val(v) => *v += e,
                                    SnailFish::Nested(nested) => {
                                        // push down
                                        push_down_helper(nested.as_mut(), exploded);
                                    }
                                }
                                return Some(Exploded::None);
                            }
                            Exploded::Me((l, r)) => {
                                sfn.right = SnailFish::Val(0);
                                match &mut sfn.left {
                                    SnailFish::Val(v) => *v += l,
                                    SnailFish::Nested(nested) => {
                                        push_down_helper(nested.as_mut(), Exploded::Left(l))
                                    }
                                }
                                return Some(Exploded::Right(r));
                            }
                            Exploded::None => return Some(Exploded::None), // an explosion happened, pop to the top
                        },
                        None => {} // continue no explosion happened
                    }
                }
            }

            None
        }

        // walk the tree down to depth 5 and explode out
        let did_explode = helper(self, 1).is_some();
        if did_explode {
            self.reduce();
        }
        self.split();
    }

    fn split(&mut self) {
        fn helper(sfn: &mut SnailFishNumber) -> bool {
            match &mut sfn.left {
                SnailFish::Val(v) => {
                    if *v >= 10 {
                        sfn.left = SnailFish::Nested(Box::new(SnailFishNumber {
                            left: SnailFish::Val(*v / 2),
                            right: SnailFish::Val((*v + 1) / 2),
                        }));
                        return true;
                    }
                }
                SnailFish::Nested(nested) => {
                    if helper(nested) {
                        return true;
                    }
                }
            };
            match &mut sfn.right {
                SnailFish::Val(v) => {
                    if *v >= 10 {
                        sfn.right = SnailFish::Nested(Box::new(SnailFishNumber {
                            left: SnailFish::Val(*v / 2),
                            right: SnailFish::Val((*v + 1) / 2),
                        }));
                        return true;
                    }
                }
                SnailFish::Nested(nested) => {
                    if helper(nested) {
                        return true;
                    }
                }
            };
            false
        }
        let did_split = helper(self);
        if did_split {
            self.reduce();
        }
    }

    fn reduce(&mut self) {
        self.explode();
        self.split();
    }

    fn magnitude(&self) -> usize {
        let left = match &self.left {
            SnailFish::Val(v) => *v,
            SnailFish::Nested(nested) => nested.magnitude(),
        };
        let right = match &self.right {
            SnailFish::Val(v) => *v,
            SnailFish::Nested(nested) => nested.magnitude(),
        };

        3 * left + 2 * right
    }
}

impl ToString for SnailFishNumber {
    fn to_string(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        chars.push('['); // first
        match &self.left {
            SnailFish::Val(val) => chars.extend(val.to_string().chars()),
            SnailFish::Nested(nested_fish) => chars.extend(nested_fish.to_string().chars()),
        };
        chars.push(',');
        match &self.right {
            SnailFish::Val(val) => chars.extend(val.to_string().chars()),
            SnailFish::Nested(nested_fish) => chars.extend(nested_fish.to_string().chars()),
        };
        chars.push(']');
        chars.iter().collect()
    }
}

impl FromStr for SnailFishNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        // [[[[4,3],4],4],[7,[[8,4],9]]] // commas can count the number of nested snaiulfish
        let mut chars = s.chars();
        chars.next();
        SnailFishNumber::from_chars(&mut chars)
    }
}

impl std::ops::Add for SnailFishNumber {
    type Output = SnailFishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = SnailFishNumber {
            left: SnailFish::Nested(Box::new(self)),
            right: SnailFish::Nested(Box::new(rhs)),
        };
        output.reduce();
        output
    }
}

pub fn part_1_solution(snails: Vec<SnailFishNumber>) -> usize {
    let final_snail = snails
        .into_iter()
        .reduce(|accum, next_fish| accum + next_fish)
        .unwrap();
    final_snail.magnitude()
}
pub fn part_2_solution(snails: Vec<SnailFishNumber>) -> usize {
    snails
        .into_iter()
        .permutations(2)
        .map(|v| {
            v.into_iter()
                .reduce(|accum, next_fish| accum + next_fish)
                .unwrap()
                .magnitude()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let snails = read_to_one_per_line::<SnailFishNumber>(P.example_path("_1")).unwrap();
        let final_snail = snails
            .into_iter()
            .reduce(|accum, next_fish| accum + next_fish)
            .unwrap();

        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string();
        // println!("{:?}", big_snail);
        println!("final snail:    {}", final_snail.to_string());
        assert_eq!(expected, final_snail.to_string());
    }

    #[test]
    fn example2_part_1() {
        let snails = read_to_one_per_line::<SnailFishNumber>(P.example_path("_2")).unwrap();
        let final_snail = snails
            .into_iter()
            .reduce(|accum, next_fish| accum + next_fish)
            .unwrap();

        let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string();
        // println!("{:?}", big_snail);
        println!("final snail:    {}", final_snail.to_string());
        assert_eq!(expected, final_snail.to_string());
    }

    #[test]
    fn test_magnitude() {
        let fish = SnailFishNumber::from_str("[[1,2],[[3,4],5]]").unwrap();
        let expected = 143;
        assert_eq!(expected, fish.magnitude());

        let fish = SnailFishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        let expected = 1384;
        assert_eq!(expected, fish.magnitude());

        let fish = SnailFishNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        let expected = 445;
        assert_eq!(expected, fish.magnitude());

        let fish = SnailFishNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        let expected = 791;
        assert_eq!(expected, fish.magnitude());

        let fish = SnailFishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        let expected = 1137;
        assert_eq!(expected, fish.magnitude());

        let fish =
            SnailFishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .unwrap();
        let expected = 3488;
        assert_eq!(expected, fish.magnitude());
    }

    #[test]
    fn example3_part_1() {
        let snails = read_to_one_per_line::<SnailFishNumber>(P.example_path("_3")).unwrap();
        let expected = 4140;
        assert_eq!(expected, part_1_solution(snails));
    }

    #[test]
    fn input_part_1() {
        let snails = read_to_one_per_line::<SnailFishNumber>(P.input_path()).unwrap();
        let expected = 4365;
        assert_eq!(expected, part_1_solution(snails));
    }

    #[test]
    fn example_part_2() {
        let snails = read_to_one_per_line::<SnailFishNumber>(P.example_path("_3")).unwrap();
        let expected = 3993;
        assert_eq!(expected, part_2_solution(snails));
    }

    #[test]
    fn input_part_2() {
        let snails = read_to_one_per_line::<SnailFishNumber>(P.input_path()).unwrap();
        let expected = 4490;
        assert_eq!(expected, part_2_solution(snails));
    }
}
