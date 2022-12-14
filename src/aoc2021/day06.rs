/// problem: https://adventofcode.com/2021/day/6
/// input: "https://adventofcode.com/2021/day/6/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 6,
    name: "Lanternfish",
};

use std::collections::VecDeque;

pub fn part_1_solution(initial_state: Vec<usize>) -> usize {
    let mut fish_counts = VecDeque::from([0; 9]); // [0, 0, 0, 0, 0, 0, 0, 0, 0]

    // for the fish ages in the initial state, increment the fishcount
    // initial_state = [3, 4, 3, 1, 2]
    // fish_counts = [0, 1, 1, 2, 1, 0, 0, 0, 0]
    for fish_age in initial_state {
        fish_counts[fish_age] += 1;
    }
    let mut baby_fish;
    for _ in 0..80 {
        baby_fish = fish_counts.pop_front().unwrap();
        fish_counts.push_back(baby_fish); // baby fish
        fish_counts[6] += baby_fish; // adults reset to day 6
    }
    fish_counts.iter().sum()
}

pub fn part_2_solution(initial_state: Vec<usize>) -> usize {
    let mut fish_counts = VecDeque::from([0; 9]);

    for fish_age in initial_state {
        fish_counts[fish_age] += 1;
    }
    let mut baby_fish;
    for _ in 0..256 {
        baby_fish = fish_counts.pop_front().unwrap();
        fish_counts.push_back(baby_fish);
        fish_counts[6] += baby_fish;
    }
    fish_counts.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn example_part_1() {
        let initial_state = read_to_string(P.example_path("_1"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 5934;
        assert_eq!(expected, part_1_solution(initial_state));
    }

    #[test]
    fn input_part_1() {
        let initial_state = read_to_string(P.input_path())
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 386640;
        assert_eq!(expected, part_1_solution(initial_state));
    }

    #[test]
    fn example_part_2() {
        let initial_state = read_to_string(P.example_path("_1"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 26984457539;
        assert_eq!(expected, part_2_solution(initial_state));
    }

    #[test]
    fn input_part_2() {
        let initial_state = read_to_string(P.input_path())
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 1733403626279;
        assert_eq!(expected, part_2_solution(initial_state));
    }
}
