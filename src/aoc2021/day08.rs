/// problem: https://adventofcode.com/2021/day/8
/// input: "https://adventofcode.com/2021/day/8/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 8,
    name: "Seven Segment Search",
};
use std::{collections::HashMap, str::FromStr};

// // TODO: try enums
// enum SegmentLocation {
//     Top,
//     TopLeft,
//     TopRight,
//     Middle,
//     BottomLeft,
//     BottomRight,
//     Bottom,
// }

pub fn print_vec_as_bits(v: &[usize]) {
    for item in v.iter() {
        print!("{:#08b}, ", item)
    }
    println!()
}

pub fn print_u8_as_bits(v: &[u8; 7]) {
    for item in v.iter() {
        print!("{:#08b}, ", item)
    }
    println!()
}

#[derive(Debug)]
pub struct SegmentBits {
    input: Vec<u8>,
    output: Vec<u8>, // could be enum
}

impl FromStr for SegmentBits {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('|').expect("should have |");
        let input: Vec<u8> = left
            .split_whitespace()
            .map(|s| {
                s.bytes()
                    .fold(0, |state, c| state ^ (1 << (c - b'a') as u8))
            })
            .collect();
        let output: Vec<u8> = right
            .split_whitespace()
            .map(|s| {
                s.chars()
                    .fold(0, |state, c| state ^ (1 << (c as u8 - b'a') as u8))
            })
            .collect();
        Ok(Self { input, output })
    }
}

impl SegmentBits {
    fn seg_len(&self, index: usize) -> usize {
        self.input.get(index).expect("out of bounds").count_ones() as usize
    }

    fn sum_output(&self) -> usize {
        // final result
        let mut word_to_num: HashMap<u8, char> = HashMap::new();
        let mut letter_count_map = [0u8; 7];
        let mut true_letter_map = [0u8; 7];
        let mut digit_array = [0u8; 10];
        for (i, word) in self.input.iter().enumerate() {
            for (j, count) in letter_count_map.iter_mut().enumerate() {
                *count += word >> j & 1;
            }
            match self.seg_len(i) {
                2 => {
                    word_to_num.insert(*word, '1');
                    true_letter_map[2] |= *word; // c
                    true_letter_map[5] |= *word; // f
                    digit_array[1] = 1;
                } // 1
                4 => {
                    word_to_num.insert(*word, '4');
                    // true_letter_map[1] |= *word; // b
                    // true_letter_map[3] |= *word; // d
                    digit_array[4] = *word;
                } // 4
                3 => {
                    word_to_num.insert(*word, '7');
                    true_letter_map[0] |= *word; // a
                    digit_array[7] = *word;
                } // 7
                7 => {
                    word_to_num.insert(*word, '8');
                    digit_array[8] = *word;
                } // 8
                _ => {}
            }
        }
        true_letter_map[0] &= !true_letter_map[2]; // a remove c and f
                                                   // true_letter_map[1] &= !true_letter_map[2]; // b remove c and f
        true_letter_map[3] &= !true_letter_map[2]; // d remove c and f
        true_letter_map[2] = 0; // c remove c and f
                                // print_u8_as_bits(&true_letter_map);
                                // println!("{:?}", letter_count_map);
        for (i, letter_count) in letter_count_map.iter().enumerate() {
            match letter_count {
                6 => true_letter_map[1] = (1 << i) as u8,  // b
                8 => true_letter_map[2] |= (1 << i) as u8, // c
                7 => {
                    //d or g
                    if ((digit_array[4] >> i) & 1) == 1 {
                        true_letter_map[3] = (1 << i) as u8; // d
                    } else {
                        true_letter_map[6] = (1 << i) as u8; // g
                    }
                }
                4 => true_letter_map[4] = (1 << i) as u8, // e
                9 => true_letter_map[5] = (1 << i) as u8, // f
                _ => {}
            }
        }

        true_letter_map[2] &= !true_letter_map[0]; // c remove a
                                                   // println!("{:?}", word_to_num);
                                                   // print_u8_as_bits(&letter_count_map);
                                                   // print_u8_as_bits(&true_letter_map);

        // we have 1 4 7 8
        word_to_num.insert(
            (digit_array[8] ^ true_letter_map[1]) ^ true_letter_map[5],
            '2',
        ); // 2
        word_to_num.insert(
            (digit_array[8] ^ true_letter_map[1]) ^ true_letter_map[4],
            '3',
        ); // 3
        word_to_num.insert(
            (digit_array[8] ^ true_letter_map[2]) ^ true_letter_map[4],
            '5',
        ); // 5
        word_to_num.insert(digit_array[8] ^ true_letter_map[2], '6'); // 6
        word_to_num.insert(digit_array[8] ^ true_letter_map[4], '9'); // 9
        word_to_num.insert(digit_array[8] ^ true_letter_map[3], '0'); // 0

        self.output
            .iter()
            .map(|word| word_to_num[word])
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Segment {
    pub input: Vec<String>, // vec!["ada", "sdasd", "asdasd"]
    pub output: Vec<String>,
}

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "split at |"
        let (left, right) = s.split_once('|').expect("should have |");
        let input: Vec<String> = left.split_whitespace().map(|s| s.to_owned()).collect();
        let output: Vec<String> = right.split_whitespace().map(|s| s.to_owned()).collect();
        Ok(Self { input, output })
    }
}

impl Segment {
    pub fn count_1478(&self) -> usize {
        self.output
            .iter()
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count()
    }
}

pub fn part_1_solution(segments: Vec<Segment>) -> usize {
    segments
        .iter()
        .map(|segment| segment.count_1478())
        .sum::<usize>()
}

pub fn part_2_solution(segments: Vec<SegmentBits>) -> usize {
    segments
        .iter()
        .map(|segment| segment.sum_output())
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn example_part_1() {
        let segments = read_to_one_per_line::<Segment>(P.example_path("_1")).unwrap();
        let expected = 26;
        assert_eq!(expected, part_1_solution(segments));
    }

    #[test]
    fn input_part_1() {
        let segments = read_to_one_per_line::<Segment>(P.input_path()).unwrap();
        let expected = 381;
        assert_eq!(expected, part_1_solution(segments));
    }

    #[test]
    fn example_part_2() {
        let segments = read_to_one_per_line::<SegmentBits>(P.example_path("_1")).unwrap();
        println!("{:?}", segments[0]);
        let expected = 61229;
        assert_eq!(expected, part_2_solution(segments));
    }

    #[test]
    fn input_part_2() {
        let segments = read_to_one_per_line::<SegmentBits>(P.input_path()).unwrap();
        println!("{:?}", segments[0]);
        let expected = 1023686;
        assert_eq!(expected, part_2_solution(segments));
    }
}
