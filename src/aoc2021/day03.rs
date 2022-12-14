/// problem: https://adventofcode.com/2021/day/3
/// input: "https://adventofcode.com/2021/day/3/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 3,
    name: "Binary Diagnostic",
};
pub fn part_1_solution(input: Vec<i32>, n_bits: usize) -> i32 {
    let mut bit_counts = (0..n_bits).map(|_| 0).collect::<Vec<i32>>();

    for i in input.iter() {
        for (b, bit_count) in bit_counts.iter_mut().enumerate() {
            *bit_count += (i >> (n_bits - 1 - b)) & 1;
        }
    }
    // [0, 0, 0, 0, 0, 7, 5, 8, 7, 5]
    let threshold = (input.len() as i32 + 1) / 2;
    // TODO: how to break ties
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (b, bit_count) in bit_counts.iter().enumerate() {
        if bit_count > &threshold {
            gamma_rate += 1 << n_bits - 1 - b
        } else {
            epsilon_rate += 1 << n_bits - 1 - b
        };
    }
    gamma_rate * epsilon_rate
}

fn most_common_bit(input: &Vec<i32>, right_index: usize) -> Option<i32> {
    // TODO: consider bool
    let threshold: i32 = (input.len() as i32 + 1) / 2;
    let ones: i32 = input.iter().map(|val| (val >> right_index) & 1).sum();
    if ones > threshold {
        Some(1)
    } else if ones < threshold {
        Some(0)
    } else {
        None
    }
}

fn filter_by_bit(mut input: Vec<i32>, max_bit_index: usize, do_keep: bool) -> i32 {
    let mut index: usize = 0;
    while input.len() > 1 {
        let target_bit = if do_keep {
            match most_common_bit(&input, max_bit_index - 1 - index) {
                Some(0) => 0,
                Some(1) => 1,
                None => 1,
                Some(_) => unreachable!(),
            }
        } else {
            match most_common_bit(&input, max_bit_index - 1 - index) {
                Some(1) => 0,
                Some(0) => 1,
                None => 0,
                Some(_) => unreachable!(),
            }
        };
        input.retain(|val| ((val >> max_bit_index - 1 - index) & 1) == target_bit);
        index += 1;
    }
    input[0]
}

/// O(mxn)
/// m: 2*number of bits
pub fn part_2_solution(input: Vec<i32>, n_bits: usize) -> i32 {
    let oxygen_rating = filter_by_bit(input.clone(), n_bits, true);
    let c02_scrubber_rating = filter_by_bit(input, n_bits, false);
    oxygen_rating * c02_scrubber_rating
}

#[cfg(test)]
mod test {
    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn example_part1() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let expected = 198;
        assert_eq!(expected, part_1_solution(input, 5))
    }

    #[test]
    fn example_part1_from_file() {
        let raw_input = match read_to_one_per_line::<String>(P.example_path("_1")) {
            Ok(data) => data,
            Err(_) => panic!("missing file"),
        };
        let bit_count = raw_input[0].chars().count();
        let input = raw_input
            .into_iter()
            .map(|binary_string| {
                i32::from_str_radix(&binary_string, 2).expect("Not a binary number!")
            })
            .collect::<Vec<i32>>();
        let expected = 198;
        assert_eq!(expected, part_1_solution(input, bit_count))
    }

    #[test]
    fn input_part1() {
        let raw_input = read_to_one_per_line::<String>(P.input_path()).ok().unwrap();
        let bit_count = raw_input[0].chars().count();
        let input = raw_input
            .into_iter()
            .map(|binary_string| {
                i32::from_str_radix(&binary_string, 2).expect("Not a binary number!")
            })
            .collect::<Vec<i32>>();
        let expected = 1071734;
        assert_eq!(expected, part_1_solution(input, bit_count))
    }

    #[test]
    fn example_part2_from_file() {
        let raw_input = read_to_one_per_line::<String>(P.example_path("_1"))
            .ok()
            .unwrap();
        let bit_count = raw_input[0].chars().count();
        let input = raw_input
            .into_iter()
            .map(|binary_string| {
                i32::from_str_radix(&binary_string, 2).expect("Not a binary number!")
            })
            .collect::<Vec<i32>>();
        let expected = 230;
        assert_eq!(expected, part_2_solution(input, bit_count))
    }

    #[test]
    fn input_part2_from_file() {
        let raw_input = read_to_one_per_line::<String>(P.input_path()).ok().unwrap();
        let bit_count = raw_input[0].chars().count();
        let input = raw_input
            .into_iter()
            .map(|binary_string| {
                i32::from_str_radix(&binary_string, 2).expect("Not a binary number!")
            })
            .collect::<Vec<i32>>();
        let expected = 6124992;
        assert_eq!(expected, part_2_solution(input, bit_count))
    }

    #[test]
    fn testing_most_common_bit() {
        let raw_input = read_to_one_per_line::<String>(P.example_path("_1"))
            .ok()
            .unwrap();
        let bit_count = raw_input[0].chars().count();
        let input = raw_input
            .into_iter()
            .map(|binary_string| {
                i32::from_str_radix(&binary_string, 2).expect("Not a binary number!")
            })
            .collect::<Vec<i32>>();

        let res = [
            most_common_bit(&input, bit_count - 1 - 0).unwrap(),
            most_common_bit(&input, bit_count - 1 - 1).unwrap(),
            most_common_bit(&input, bit_count - 1 - 2).unwrap(),
            most_common_bit(&input, bit_count - 1 - 3).unwrap(),
            most_common_bit(&input, bit_count - 1 - 4).unwrap(),
        ];

        println!("{:?}", res);
        // assert_eq!(expected, part_1_solution(input, bit_count))
    }
    #[test]
    fn testing_filter() {
        let raw_input = read_to_one_per_line::<String>(P.example_path("_1"))
            .ok()
            .unwrap();
        let bit_count = raw_input[0].chars().count();
        let input = raw_input
            .into_iter()
            .map(|binary_string| {
                i32::from_str_radix(&binary_string, 2).expect("Not a binary number!")
            })
            .collect::<Vec<i32>>();

        // let oxygen = filter_by_bit(input.clone(), bit_count, true);
        // println!("{:?}", oxygen);
        let c02 = filter_by_bit(input.clone(), bit_count, false);
        println!("{:?}", c02);
        // assert_eq!(expected, part_1_solution(input, bit_count))
    }
}
