/// problem: https://adventofcode.com/2021/day/9
/// input: "https://adventofcode.com/2021/day/9/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 9,
    name: "Smoke Basin",
};
use std::{collections::VecDeque, usize::MAX};

struct Neighbours(Vec<(usize, usize)>);

impl Neighbours {
    fn from_node(node: &(usize, usize)) -> Self {
        let (x, y) = (node.0.to_owned(), node.1.to_owned()); // does this copy or clone?
        let mut neighbours = vec![(x + 1, y), (x, y + 1)];
        if x != 0 {
            neighbours.push((x - 1, y));
        }
        if y != 0 {
            neighbours.push((x, y - 1));
        }
        Self(neighbours)
    }
}

impl IntoIterator for Neighbours {
    type Item = (usize, usize);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn bfs(
    cave_depths: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
) -> usize {
    visited[start.0][start.1] = true;
    if cave_depths[start.0][start.1] == 9 {
        return 0;
    }

    let mut basin_nodes = VecDeque::from([start]);
    let mut basin_size = 0;
    while let Some(next_node) = basin_nodes.pop_front() {
        basin_size += 1;
        for neighbour in Neighbours::from_node(&next_node) {
            if let Some(row) = visited.get_mut(neighbour.0) {
                if let Some(node_is_visited) = row.get_mut(neighbour.1) {
                    if !*node_is_visited {
                        *node_is_visited = true; // could use indexing as well
                        if cave_depths[neighbour.0][neighbour.1] != 9 {
                            // bounds checking is done here
                            basin_nodes.push_back(neighbour);
                        }
                    }
                }
            }
        }
    }
    basin_size
}

// fn pad_vec(v: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
//     let mut v = v;
//     let push_val = 9;

//     for row in v.iter_mut() {
//         row.push(push_val);
//         row.insert(0, push_val)
//     }
//     v.push(vec![push_val; v[0].len()]);
//     v.insert(0, vec![push_val; v[0].len()]);
//     v
// }

pub fn part_1_solution(cave_depths: Vec<Vec<usize>>) -> usize {
    let mut lowest = 0;
    let max_row_index = cave_depths.len() - 1;
    let max_col_index = cave_depths[0].len() - 1;

    for row in 0..cave_depths.len() {
        for col in 0..cave_depths[0].len() {
            let curr = cave_depths[row][col];
            let left = if col == 0 {
                MAX
            } else {
                cave_depths[row][col - 1]
            };
            let up = if row == 0 {
                MAX
            } else {
                cave_depths[row - 1][col]
            };
            let right = if col == max_col_index {
                MAX
            } else {
                cave_depths[row][col + 1]
            };
            let down = if row == max_row_index {
                MAX
            } else {
                cave_depths[row + 1][col]
            };

            if curr < left && curr < up && curr < right && curr < down {
                lowest += curr + 1;
            }
        }
    }
    lowest
}

pub fn part_2_solution(cave_depths: Vec<Vec<usize>>) -> usize {
    let mut visited = vec![vec![false; cave_depths[0].len()]; cave_depths.len()];
    let mut scores: Vec<usize> = Vec::new();
    for row in 0..cave_depths.len() {
        for col in 0..cave_depths[0].len() {
            if !visited[row][col] {
                scores.push(bfs(&cave_depths, &mut visited, (row, col)));
            }
        }
    }
    scores.sort();
    // not handling single basin result because the production cases don't have them
    scores.pop().unwrap() * scores.pop().unwrap() * scores.pop().unwrap()
}

#[cfg(test)]
mod test {
    use crate::read_to_vec_per_line;

    use super::*;

    #[test]
    fn example_part_1() {
        let cave_depths = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).unwrap() as usize)
        })
        .unwrap();
        let expected = 15;
        assert_eq!(expected, part_1_solution(cave_depths))
    }

    #[test]
    fn input_part_1() {
        let cave_depths =
            read_to_vec_per_line(P.input_path(), |c| Some(c.to_digit(10).unwrap() as usize))
                .unwrap();
        let expected = 560;
        assert_eq!(expected, part_1_solution(cave_depths))
    }

    #[test]
    fn example_part_2() {
        let cave_depths = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).unwrap() as usize)
        })
        .unwrap();
        let expected = 1134;
        assert_eq!(expected, part_2_solution(cave_depths))
    }

    #[test]
    fn input_part_2() {
        let cave_depths =
            read_to_vec_per_line(P.input_path(), |c| Some(c.to_digit(10).unwrap() as usize))
                .unwrap();
        let expected = 959136;
        assert_eq!(expected, part_2_solution(cave_depths))
    }
}
