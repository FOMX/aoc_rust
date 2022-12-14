/// problem: https://adventofcode.com/2021/day/7
/// input: "https://adventofcode.com/2021/day/7/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 7,
    name: "The Treachery of Whales",
};

fn triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

fn reverse_triangle_number(n: usize) -> usize {
    ((2 * n) as f32).sqrt() as usize
}

fn median<T: Ord + Clone>(v: &[T]) -> T {
    let mut vs = v.to_owned();
    vs.sort();
    vs[vs.len() / 2].to_owned()
}

pub fn part_1_solution(crab_positions: Vec<usize>) -> usize {
    let pivot = median(&crab_positions);
    crab_positions
        .iter()
        .map(|c| c.abs_diff(pivot))
        .sum::<usize>()
}

pub fn part_2_solution_attempt(crab_positions: Vec<usize>) -> usize {
    // let mut pivot = median(&crab_positions);
    // pivot = pivot + triangle_number(pivot);
    let pivot = reverse_triangle_number(median(
        &crab_positions
            .iter()
            .map(|n| triangle_number(*n))
            .collect::<Vec<usize>>(),
    ));
    println!("{}", pivot);
    crab_positions
        .iter()
        .map(|c| triangle_number(c.abs_diff(pivot)))
        .sum::<usize>()
}

pub fn part_2_solution(crab_positions: Vec<usize>) -> usize {
    // let mut pivot = median(&crab_positions);
    // pivot = pivot + triangle_number(pivot);
    (0..(*crab_positions.iter().max().unwrap()))
        .map(|p| {
            crab_positions
                .iter()
                .map(|&c| triangle_number(c.abs_diff(p)))
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn example_part_1() {
        let crab_positions = read_to_string(P.example_path("_1"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 37;
        assert_eq!(expected, part_1_solution(crab_positions));
    }

    #[test]
    fn input_part_1() {
        let crab_positions = read_to_string(P.input_path())
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 353800;
        assert_eq!(expected, part_1_solution(crab_positions));
    }

    #[test]
    fn example_part_2() {
        let crab_positions = read_to_string(P.example_path("_1"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 168;
        assert_eq!(expected, part_2_solution(crab_positions));
    }

    #[test]
    fn input_part_2() {
        let crab_positions = read_to_string(P.input_path())
            .unwrap()
            .trim() // get rid of trailing blank line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let expected = 98119739;
        assert_eq!(expected, part_2_solution(crab_positions));
    }
}
