/// problem: https://adventofcode.com/2022/day/1
/// input: https://adventofcode.com/2022/day/1/input
use crate::Problem;

const P: Problem = Problem {
    year: 2022,
    day: 1,
    name: "Calorie Counting",
};

fn pt1(data: &str) -> usize {
    let elves = parse(data);
    match elves.iter().max() {
        Some(v) => *v,
        None => unreachable!(),
    }
}

fn pt2(data: &str) -> usize {
    let mut elves = parse(data);
    elves.sort();
    elves.iter().rev().take(3).sum()
}

fn parse(data: &str) -> Vec<usize> {
    data.replace('\r', "")
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn pt1_example() {
        let data = read_to_string(P.example_path("_1")).unwrap();
        let expected = 24000;
        assert_eq!(pt1(&data), expected);
    }

    #[test]
    fn pt1_input() {
        let data = read_to_string(P.input_path()).unwrap();
        let expected = 67633;

        assert_eq!(pt1(&data), expected);
    }
    #[test]
    fn pt2_example() {
        let data = read_to_string(P.example_path("_1")).unwrap();
        let expected = 45000;
        assert_eq!(pt2(&data), expected);
    }

    #[test]
    fn pt2_input() {
        let data = read_to_string(P.input_path()).unwrap();
        let expected = 199628;
        assert_eq!(pt2(&data), expected);
    }

    #[test]
    pub fn test_string() {
        let mut s = "apple".chars(); //  ['a' 'p' 'p' 'l' 'e']

        // invalid
        // let ap = s.take(2).collect::<String>();
        // let pl = s.take(2).collect::<String>();

        // // valid
        let t = &mut s;
        let ap = t.take(2).collect::<String>();
        let pl = t.take(2).collect::<String>();

        println!("ap: {:?} pl: {:?}", ap, pl);

        // let b = &mut bin_chars; // TODO: why the fuck do i need to do this on a ref
    }
}
