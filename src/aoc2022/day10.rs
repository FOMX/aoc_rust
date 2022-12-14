/// problem: https://adventofcode.com/2022/day/10
/// input: https://adventofcode.com/2022/day/10/input
use std::str::FromStr;

use itertools::Itertools;

use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 10,
    name: "Cathode-Ray Tube",
};

pub enum Command {
    Noop,
    Addx(isize),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut splits = s.split(' ');
        match splits.next().unwrap() {
            "noop" => Ok(Command::Noop),
            "addx" => Ok(Command::Addx(
                splits.next().unwrap().parse::<isize>().unwrap(),
            )),
            e => Err(anyhow::format_err!("unexpected val: {}", e)),
        }
    }
}

pub fn part_1_solution(commands: Vec<Command>) -> isize {
    let mut curr_score = 1;
    let mut scores = vec![curr_score];

    for command in commands.iter() {
        match command {
            Command::Noop => {
                scores.push(curr_score);
            }
            Command::Addx(val) => {
                scores.push(curr_score);
                scores.push(curr_score);
                curr_score += val;
            }
        }
    }
    let ret = scores
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut ret = if (s - ((i as isize - 1) % 40) as isize).abs() <= 1 {
                "#"
            } else {
                " "
            }
            .to_owned();
            if (i as isize) % 40 == 0 {
                ret.push('\n')
            }
            ret
        })
        .collect::<String>();
    println!("{}", ret);
    scores
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if (20..400).step_by(40).contains(&i) {
                Some(s * i as isize)
            } else {
                None
            }
        })
        .sum()
}

pub fn update(
    screen: &mut Vec<Vec<char>>,
    scores: &mut Vec<isize>,
    cycle: usize,
    curr_score: isize,
) {
    if cycle == 20 || ((cycle as isize - 20) % 40 == 0) {
        scores.push(curr_score);
    }
    let row = (cycle - 1) / 40;
    let col = (cycle - 1) % 40;
    screen[row][col] = if (curr_score - ((cycle - 1) % 40) as isize).abs() <= 1 {
        '#'
    } else {
        ' '
    };
}
pub fn part_2_solution(commands: Vec<Command>) {
    let mut curr_score = 1;
    let mut scores = vec![curr_score];

    let mut cycle = 0;
    let mut screen = vec![vec!['?'; 40]; 6];

    for command in commands.iter() {
        match command {
            Command::Noop => {
                cycle += 1;
                update(&mut screen, &mut scores, cycle, curr_score);
            }
            Command::Addx(val) => {
                cycle += 1;
                update(&mut screen, &mut scores, cycle, curr_score);
                cycle += 1;
                update(&mut screen, &mut scores, cycle, curr_score);
                curr_score += val;
            }
        }
    }
    for row in screen.iter() {
        let s = row.iter().join("");
        println!("{:?}", s);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::read_to_one_per_line;

    #[test]
    fn example_part_1() {
        let commands = read_to_one_per_line::<Command>(P.example_path("_1")).unwrap();
        let expected = 13140;
        assert_eq!(expected, part_1_solution(commands))
    }

    #[test]
    fn input_part_1() {
        let commands = read_to_one_per_line::<Command>(P.input_path()).unwrap();
        let expected = 12460;
        assert_eq!(expected, part_1_solution(commands))
    }

    #[test]
    fn example_part_2() {
        let commands = read_to_one_per_line::<Command>(P.example_path("_1")).unwrap();

        part_2_solution(commands);
    }

    #[test]
    fn input_part_2() {
        let commands = read_to_one_per_line::<Command>(P.input_path()).unwrap();
        part_2_solution(commands);
    }
}
