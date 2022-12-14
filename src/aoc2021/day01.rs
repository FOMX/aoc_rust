/// problem: https://adventofcode.com/2021/day/1
/// input: "https://adventofcode.com/2021/day/1/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 1,
    name: "Sonar Sweep",
};

pub fn basic_solution(data: Vec<usize>) -> usize {
    let increments = data
        .windows(2)
        .map(|window| usize::from(window[1] > window[0]))
        .sum();
    increments
}

pub fn n_window_solution(data: Vec<usize>, window_size: usize) -> usize {
    let increments = data
        .windows(window_size + 1)
        .map(|window| usize::from(window[window_size] > window[0]))
        .sum();
    increments
}

#[cfg(test)]
mod test {
    use crate::read_to_one_per_line;

    use super::*;

    #[test]
    fn basic() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;

        assert_eq!(basic_solution(data), expected);
    }

    #[test]
    fn basic_2() {
        let data = read_to_one_per_line::<usize>(P.example_path("_1"))
            .ok()
            .unwrap();
        let expected = 7;
        assert_eq!(basic_solution(data), expected);
    }

    #[test]
    fn input_1() {
        let data = read_to_one_per_line::<usize>(P.input_path()).ok().unwrap();
        let expected = 1581;
        assert_eq!(basic_solution(data), expected);
    }

    #[test]
    fn input_1_method_2() {
        let data = read_to_one_per_line::<usize>(P.input_path()).ok().unwrap();
        let expected = 1581;
        let window_size = 1;
        assert_eq!(n_window_solution(data, window_size), expected);
    }

    #[test]
    fn input_2_method_2_window_3() {
        let data = read_to_one_per_line::<usize>(P.input_path()).ok().unwrap();
        let expected = 1618;
        let window_size = 3;
        assert_eq!(n_window_solution(data, window_size), expected);
    }
}
