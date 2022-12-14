use crate::Problem;
/// problem: https://adventofcode.com/2022/day/6
/// input: https://adventofcode.com/2022/day/6/input
use std::collections::HashSet;
const P: Problem = Problem {
    year: 2022,
    day: 6,
    name: "Tuning Trouble",
};
fn unique_len_windows(s: String, target_len: usize) -> Option<usize> {
    for (i, char_slice) in s.as_bytes().windows(target_len).enumerate() {
        let hs: HashSet<&u8> = HashSet::from_iter(char_slice);
        if hs.len() == target_len {
            return Some(target_len + i);
        }
    }
    None
}

fn _unique_len_vec(s: String, target_len: usize) -> Option<usize> {
    for (i, char_slice) in s.as_bytes().windows(target_len).enumerate() {
        let hs: HashSet<&u8> = HashSet::from_iter(char_slice);
        if hs.len() == target_len {
            return Some(target_len + i);
        }
    }
    None
}

pub fn part_1_solution(input: String) -> usize {
    unique_len_windows(input, 4).unwrap()
}
pub fn part_2_solution(input: String) -> usize {
    unique_len_windows(input, 14).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn example_part_1() {
        let input = read_to_string(P.example_path("_1"))
            .unwrap()
            .trim()
            .to_owned();
        let expected = 7;
        assert_eq!(expected, part_1_solution(input));
    }

    #[test]
    fn examples_part_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned();
        let expected = 5;
        assert_eq!(expected, part_1_solution(input));

        let input = "nppdvjthqldpwncqszvftbrmjlhg".to_owned();
        let expected = 6;
        assert_eq!(expected, part_1_solution(input))
    }

    #[test]
    fn input_part_1() {
        let input = read_to_string(P.input_path()).unwrap().trim().to_owned();
        let expected = 1965;
        assert_eq!(expected, part_1_solution(input));
    }

    #[test]
    fn examples_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned();
        let expected = 19;
        assert_eq!(expected, part_2_solution(input));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned();
        let expected = 23;
        assert_eq!(expected, part_2_solution(input))
    }

    #[test]
    fn input_part_2() {
        let input = read_to_string(P.input_path()).unwrap().trim().to_owned();
        let expected = 2773;
        assert_eq!(expected, part_2_solution(input));
    }

    #[test]
    fn bytes() {
        let input = read_to_string(P.input_path()).unwrap().trim().to_owned();
        let bytes = input.bytes();
        let chars = input.chars();
        assert_eq!(bytes.len(), chars.count());
    }
}
