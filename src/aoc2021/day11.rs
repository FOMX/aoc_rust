/// problem: https://adventofcode.com/2021/day/11
/// input: "https://adventofcode.com/2021/day/11/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 11,
    name: "Dumbo Octopus",
};
use std::collections::HashSet;

fn octopus_neighbours(octopus: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let (x, y) = *octopus;
    if x > 0 {
        neighbours.push((x - 1, y));
        if y > 0 {
            neighbours.push((x - 1, y - 1));
        }
        if y < 9 {
            neighbours.push((x - 1, y + 1));
        }
    }
    if x < 9 {
        neighbours.push((x + 1, y));
        if y > 0 {
            neighbours.push((x + 1, y - 1));
        }
        if y < 9 {
            neighbours.push((x + 1, y + 1));
        }
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y < 9 {
        neighbours.push((x, y + 1));
    }
    neighbours
}

fn flash(
    octopii_state: &mut Vec<Vec<usize>>,
    flashed: &mut HashSet<(usize, usize)>,
    flashing_octopus: &(usize, usize),
) {
    // recursive
    let neighbours = octopus_neighbours(&flashing_octopus);

    for (x, y) in neighbours {
        if flashed.contains(&(x, y)) {
            // already flashed
            continue;
        }
        octopii_state[x][y] += 1;
        if octopii_state[x][y] > 9 {
            flashed.insert((x, y));
            flash(octopii_state, flashed, &(x, y));
        }
    }
}

fn process_octopii(octopii_state: &mut Vec<Vec<usize>>) -> usize {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    // increment all the octopii and locate the flashing ones
    for (i, row) in octopii_state.iter_mut().enumerate() {
        for (j, state) in row.iter_mut().enumerate() {
            if *state == 9 {
                flashed.insert((i, j));
            }
            *state += 1;
        }
    }

    // for the original flashing octopii, instruct them to flash.
    // the secondary flashing octopii will be added to this set AFTER they flash
    for flashing_octopus in flashed.clone().iter() {
        flash(octopii_state, &mut flashed, flashing_octopus);
    }

    // reset the flashed octopii
    for flashed_octopus in flashed.iter() {
        let (x, y) = *flashed_octopus;
        octopii_state[x][y] = 0;
    }
    flashed.len()
}

pub fn part_1_solution(octopii_state: Vec<Vec<usize>>) -> usize {
    let mut octopii_state = octopii_state;
    let mut octopii_flashes = 0;
    for _day in 0..100 {
        octopii_flashes += process_octopii(&mut octopii_state);
    }
    octopii_flashes
}

pub fn part_2_solution(octopii_state: Vec<Vec<usize>>) -> usize {
    let mut octopii_state = octopii_state;
    for day in 1.. {
        if process_octopii(&mut octopii_state) == 100 {
            return day;
        };
    }
    0
}

#[cfg(test)]
mod test {
    use crate::read_to_vec_per_line;

    use super::*;

    #[test]
    fn example_part_1() {
        let octopii: Vec<Vec<usize>> = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).unwrap() as usize)
        })
        .unwrap();
        let expected = 1656;
        assert_eq!(expected, part_1_solution(octopii))
    }

    #[test]
    fn example2_part_1() {
        let octopii: Vec<Vec<usize>> = read_to_vec_per_line(P.example_path("_2"), |c| {
            Some(c.to_digit(10).unwrap() as usize)
        })
        .unwrap();
        let expected = 1004;
        assert_eq!(expected, part_1_solution(octopii))
    }

    #[test]
    fn input_part_1() {
        let octopii: Vec<Vec<usize>> =
            read_to_vec_per_line(P.input_path(), |c| Some(c.to_digit(10).unwrap() as usize))
                .unwrap();
        let expected = 1659;
        assert_eq!(expected, part_1_solution(octopii))
    }

    #[test]
    fn example_part_2() {
        let octopii: Vec<Vec<usize>> = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).unwrap() as usize)
        })
        .unwrap();
        let expected = 195;
        assert_eq!(expected, part_2_solution(octopii))
    }

    #[test]
    fn input_part_2() {
        let octopii: Vec<Vec<usize>> =
            read_to_vec_per_line(P.input_path(), |c| Some(c.to_digit(10).unwrap() as usize))
                .unwrap();
        let expected = 227;
        assert_eq!(expected, part_2_solution(octopii))
    }
}
