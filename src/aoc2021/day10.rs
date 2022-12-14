/// problem: https://adventofcode.com/2021/day/10
/// input: "https://adventofcode.com/2021/day/10/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 10,
    name: "Syntax Scoring",
};
use std::collections::HashMap;

const OPENNERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];

fn get_corrupted_token(chunk: &str) -> Option<char> {
    let corresponding_openner = CLOSERS
        .iter()
        .zip(OPENNERS.iter())
        .collect::<HashMap<_, _>>();

    let mut token_stack = String::new();
    for token in chunk.chars() {
        if OPENNERS.contains(&token) {
            token_stack.push(token);
        } else if CLOSERS.contains(&token) {
            let expected_opening_token = corresponding_openner[&token];
            if let Some(opening_token) = token_stack.chars().last() {
                if expected_opening_token != &opening_token {
                    return Some(token);
                } else {
                    token_stack.pop();
                }
            } else {
                panic!("ran out of tokens");
            }
        } else {
            panic!("unexpected token {}", token);
        }
    }
    None
}

fn get_missing_tokens(chunk: &str) -> Option<String> {
    let corresponding_closer = OPENNERS
        .iter()
        .zip(CLOSERS.iter())
        .collect::<HashMap<_, _>>();
    let corresponding_openner = CLOSERS
        .iter()
        .zip(OPENNERS.iter())
        .collect::<HashMap<_, _>>();

    let mut token_stack = String::new();
    for token in chunk.chars() {
        if OPENNERS.contains(&token) {
            token_stack.push(token);
        } else if CLOSERS.contains(&token) {
            // we can assume it matches
            let expected_opening_token = corresponding_openner[&token];
            if let Some(opening_token) = token_stack.chars().last() {
                if expected_opening_token != &opening_token {
                    panic!("the token should match");
                } else {
                    token_stack.pop();
                }
            } else {
                panic!("ran out of tokens");
            }
        } else {
            panic!("unexpected token {}", token);
        }
    }
    if !token_stack.is_empty() {
        Some(
            token_stack
                .chars()
                .rev()
                .map(|opening_token| corresponding_closer[&opening_token])
                .collect(),
        )
    } else {
        None
    }
}

pub fn part_1_solution(chunks: Vec<String>) -> usize {
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    chunks
        .iter()
        .filter_map(|chunk| get_corrupted_token(chunk).map(|token| points[&token]))
        .sum()
}

pub fn part_2_solution(chunks: Vec<String>) -> usize {
    let points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let non_corrupted_chunks = chunks
        .iter()
        .filter(|chunk| match get_corrupted_token(chunk) {
            None => true,
            _ => false,
        });

    let mut scores: Vec<usize> = non_corrupted_chunks
        .filter_map(|chunk| {
            if let Some(missing_tokens) = get_missing_tokens(&chunk) {
                Some(
                    missing_tokens
                        .chars()
                        .fold(0, |acc, token| acc * 5 + points[&token]), // calculate the scores
                )
            } else {
                None
            }
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn example_part_1() {
        let chunks = read_to_one_per_line::<String>(P.example_path("_1")).unwrap();
        let expected = 26397;
        assert_eq!(expected, part_1_solution(chunks))
    }

    #[test]
    fn input_part_1() {
        let chunks = read_to_one_per_line::<String>(P.input_path()).unwrap();
        let expected = 296535;
        assert_eq!(expected, part_1_solution(chunks))
    }

    #[test]
    fn example_part_2() {
        let chunks = read_to_one_per_line::<String>(P.example_path("_1")).unwrap();
        let expected = 288957;
        assert_eq!(expected, part_2_solution(chunks))
    }

    #[test]
    fn input_part_2() {
        let chunks = read_to_one_per_line::<String>(P.input_path()).unwrap();
        let expected = 4245130838;
        assert_eq!(expected, part_2_solution(chunks))
    }
}
