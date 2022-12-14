/// problem: https://adventofcode.com/2021/day/14
/// input: "https://adventofcode.com/2021/day/14/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 14,
    name: "Extended Polymerization",
};
use itertools::Itertools;
use std::collections::HashMap;

pub fn expand_polymer(polymer: String, polymer_map: &HashMap<String, char>) -> String {
    let chars: Vec<char> = polymer.chars().collect();

    let mut expanded = String::new();
    expanded.push(chars[0]); // implicit copy
    for pair in chars.windows(2) {
        let mut key = String::from(pair[0]);
        key.push(pair[1]); // i have no idea how to manipulate a slice of chars
        let middle = polymer_map.get(key.as_str()).expect("missing polymer pair");
        expanded.push(*middle);
        expanded.push(pair[1]);
    }
    expanded
}

pub fn expand_polymer_counts(
    polymer_pair_counts: HashMap<(char, char), usize>,
    polymer_output_mapping: &HashMap<(char, char), ((char, char), (char, char))>,
) -> HashMap<(char, char), usize> {
    let mut expanded_pair_counts = HashMap::new();
    for (pair, count) in polymer_pair_counts {
        let (left, right) = polymer_output_mapping.get(&pair).unwrap();
        *expanded_pair_counts.entry(*left).or_insert(0usize) += count;
        *expanded_pair_counts.entry(*right).or_insert(0usize) += count;
    }
    expanded_pair_counts
}

pub fn part_1_solution(polymer: String, polymer_map: HashMap<String, char>) -> usize {
    let mut polymer_output_mapping: HashMap<(char, char), ((char, char), (char, char))> =
        HashMap::new();
    for (pair, middle) in polymer_map {
        let mut pair = pair.chars();
        let left = pair.next().unwrap();
        let right = pair.next().unwrap();
        polymer_output_mapping.insert((left, right), ((left, middle), (middle, right)));
    }

    let mut polymer_pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer.chars().tuple_windows() {
        *polymer_pair_counts.entry(pair).or_insert(0usize) += 1;
    }

    for _ in 0..10 {
        polymer_pair_counts = expand_polymer_counts(polymer_pair_counts, &polymer_output_mapping);
    }

    let mut count_map = HashMap::new();
    for ((left, right), count) in polymer_pair_counts {
        *count_map.entry(left).or_insert(0usize) += count;
        *count_map.entry(right).or_insert(0usize) += count;
    }
    for c in [
        polymer.chars().next().unwrap(),
        polymer.chars().last().unwrap(),
    ] {
        *count_map.entry(c).or_insert(0) += 1;
    }

    let mut max_score = 0;
    let mut min_score = usize::MAX;
    for (_, v) in count_map.iter() {
        max_score = max_score.max(*v);
        min_score = min_score.min(*v);
    }
    max_score / 2 - min_score / 2 // need to divide by two as neighbours are counted twice
}

pub fn part_1_solution_naive(polymer: String, polymer_map: HashMap<String, char>) -> usize {
    let mut polymer = polymer;
    for _ in 0..10 {
        polymer = expand_polymer(polymer, &polymer_map);
    }
    let mut count_map = HashMap::new();
    for ch in polymer.chars() {
        *count_map.entry(ch).or_insert(0) += 1;
    }
    let mut max_score = 0;
    let mut min_score = usize::MAX;
    for (_, v) in count_map.iter() {
        max_score = max_score.max(*v);
        min_score = min_score.min(*v);
    }
    max_score - min_score
}

pub fn part_2_solution(polymer: String, polymer_map: HashMap<String, char>) -> usize {
    let mut polymer_output_mapping: HashMap<(char, char), ((char, char), (char, char))> =
        HashMap::new();
    for (pair, middle) in polymer_map {
        let mut pair = pair.chars();
        let left = pair.next().unwrap();
        let right = pair.next().unwrap();
        polymer_output_mapping.insert((left, right), ((left, middle), (middle, right)));
    }

    let mut polymer_pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer.chars().tuple_windows() {
        *polymer_pair_counts.entry(pair).or_insert(0usize) += 1;
    }

    for _ in 0..40 {
        polymer_pair_counts = expand_polymer_counts(polymer_pair_counts, &polymer_output_mapping);
    }

    let mut count_map = HashMap::new();
    for ((left, right), count) in polymer_pair_counts {
        *count_map.entry(left).or_insert(0usize) += count;
        *count_map.entry(right).or_insert(0usize) += count;
    }

    for c in [
        polymer.chars().next().unwrap(),
        polymer.chars().last().unwrap(),
    ] {
        *count_map.entry(c).or_insert(0) += 1;
    }

    let mut max_score = 0;
    let mut min_score = usize::MAX;
    for (_, v) in count_map.iter() {
        max_score = max_score.max(*v);
        min_score = min_score.min(*v);
    }
    max_score / 2 - min_score / 2
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::fs::read_to_string;
    fn read_day14(path: impl AsRef<std::path::Path>) -> Result<(String, HashMap<String, char>)> {
        let file = read_to_string(path)?; // need to create this binding as lines takes a reference to a string
        let mut lines = file.lines();

        let polymer = lines.next().expect("chars").to_owned();
        lines.next(); // skip empty line

        let mut polymer_map = HashMap::new();
        while let Some(next_line) = lines.next() {
            if next_line.is_empty() {
                break;
            }
            let (key, val) = next_line
                .split_once(" -> ")
                .expect("unable to split at -> ");
            polymer_map.insert(key.to_owned(), val.chars().next().unwrap());
        }

        Ok((polymer, polymer_map))
    }

    #[test]
    fn example_part_1() {
        let (polymer, polymer_map) = read_day14(P.example_path("_1")).unwrap();
        let expected = 1588;
        assert_eq!(expected, part_1_solution(polymer, polymer_map))
    }

    #[test]
    fn example_small_part_1() {
        let (polymer, polymer_map) = read_day14(P.example_path("_small")).unwrap();
        let expected = 1;
        assert_eq!(expected, part_1_solution(polymer, polymer_map))
    }

    #[test]
    fn input_part_1() {
        let (polymer, polymer_map) = read_day14(P.input_path()).unwrap();
        let expected = 2509;
        assert_eq!(expected, part_1_solution(polymer, polymer_map))
    }

    #[test]
    fn example_part_2() {
        let (polymer, polymer_map) = read_day14(P.example_path("_1")).unwrap();

        let expected = 2188189693529;
        assert_eq!(expected, part_2_solution(polymer, polymer_map))
    }

    #[test]
    fn example_small_part_2() {
        let (polymer, polymer_map) = read_day14(P.example_path("_small")).unwrap();
        let expected = 1;
        assert_eq!(expected, part_2_solution(polymer, polymer_map))
    }

    #[test]
    fn input_part_2() {
        let (polymer, polymer_map) = read_day14(P.input_path()).unwrap();
        let expected = 2827627697643;
        assert_eq!(expected, part_2_solution(polymer, polymer_map))
    }
}
