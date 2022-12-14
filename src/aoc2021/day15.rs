/// problem: https://adventofcode.com/2021/day/15
/// input: "https://adventofcode.com/2021/day/15/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 15,
    name: "Chiton",
};

use itertools::Itertools;

fn expand_graph_5_times(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let size = graph.len();
    assert_eq!(size, graph[0].len());

    let mut expanded_graph: Vec<Vec<usize>> = vec![vec![0; size * 5]; size * 5];

    for (row, col) in (0..5).cartesian_product(0..5) {
        for (x, y) in (0..size).cartesian_product(0..size) {
            let new_x = row * size + x;
            let new_y = col * size + y;
            let val = ((graph[x][y] + row + col - 1) % 9) + 1;
            expanded_graph[new_x][new_y] = val;
        }
    }
    expanded_graph
}

fn get_neighbours(coordinate: (usize, usize), xmax: usize, ymax: usize) -> Vec<(usize, usize)> {
    let (x, y) = coordinate;
    let mut neighbours = Vec::new();
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if x < xmax {
        neighbours.push((x + 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y < ymax {
        neighbours.push((x, y + 1));
    }
    neighbours
}

/// return the shortest path as per djikstra
fn djikstra(
    graph: &Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<usize>> {
    // start is usually 0,0

    let xmax = graph.len() - 1;
    let ymax = graph[0].len() - 1;

    // initialize the distances
    let mut distances = vec![vec![usize::MAX; ymax + 1]; xmax + 1];
    distances[start.0][start.1] = graph[start.0][start.1];

    // initialize the visited
    let mut visited = vec![vec![false; ymax + 1]; xmax + 1];
    visited[start.0][start.1] = true;

    let mut frontier = get_neighbours(start, xmax, ymax); // TODO: use a queue/heap
    for node in frontier.iter() {
        distances[node.0][node.1] = distances[start.0][start.1] + graph[node.0][node.1];
    }
    let mut next_nearest_node = start;
    while next_nearest_node != end {
        // get next best candidate
        let mut min_dist = usize::MAX;
        next_nearest_node = start;
        for node in frontier.iter() {
            if distances[node.0][node.1] < min_dist {
                min_dist = distances[node.0][node.1];
                next_nearest_node = *node;
            }
        }
        if next_nearest_node == start {
            panic!("nearest node not found");
        }

        // visit node
        visited[next_nearest_node.0][next_nearest_node.1] = true;
        // add to frontier and update distances
        for node in get_neighbours(next_nearest_node, xmax, ymax) {
            if distances[node.0][node.1]
                > distances[next_nearest_node.0][next_nearest_node.1] + graph[node.0][node.1]
            {
                distances[node.0][node.1] =
                    distances[next_nearest_node.0][next_nearest_node.1] + graph[node.0][node.1];
                if distances[node.0][node.1] < graph[node.0][node.1] {
                    println!("shouldn't be able to get here");
                }
            }
            if !visited[node.0][node.1] {
                frontier.push(node);
            }
        }
        frontier.retain(|node| *node != next_nearest_node);
    }

    distances
}

pub fn part_1_solution(chiton_heights: Vec<Vec<usize>>) -> usize {
    let start = (0, 0);
    let end = (chiton_heights.len() - 1, chiton_heights[0].len() - 1);

    let distances = djikstra(&chiton_heights, start, end);
    distances[end.0][end.1] - distances[start.0][start.1] // apparently the start doesn't count
}

pub fn part_2_solution(chiton_heights: Vec<Vec<usize>>) -> usize {
    let expanded_chiton_heights = expand_graph_5_times(chiton_heights);
    let start = (0, 0);
    let end = (
        expanded_chiton_heights.len() - 1,
        expanded_chiton_heights[0].len() - 1,
    );

    let distances = djikstra(&expanded_chiton_heights, start, end);
    distances[end.0][end.1] - distances[start.0][start.1] // apparently the start doesn't count
}

#[cfg(test)]
mod test {
    use crate::read_to_vec_per_line;

    use super::*;

    #[test]
    fn test_neighbours() {
        let xmax = 12;
        let ymax = 10;
        let node = (0, 0);
        let expected = vec![(1, 0), (0, 1)];
        assert_eq!(expected, get_neighbours(node, xmax, ymax));

        let node = (1, 1);
        let expected = vec![(0, 1), (2, 1), (1, 0), (1, 2)];
        assert_eq!(expected, get_neighbours(node, xmax, ymax));

        let node = (12, 10);
        let expected = vec![(11, 10), (12, 9)];
        assert_eq!(expected, get_neighbours(node, xmax, ymax));
    }

    #[test]
    fn test_expand_graph_5_times() {
        let input = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
            vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
            vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
            vec![3, 3, 4, 4, 5, 5, 6, 6, 7, 7],
            vec![3, 3, 4, 4, 5, 5, 6, 6, 7, 7],
            vec![4, 4, 5, 5, 6, 6, 7, 7, 8, 8],
            vec![4, 4, 5, 5, 6, 6, 7, 7, 8, 8],
            vec![5, 5, 6, 6, 7, 7, 8, 8, 9, 9],
            vec![5, 5, 6, 6, 7, 7, 8, 8, 9, 9],
        ];
        assert_eq!(expected, expand_graph_5_times(input));

        let input = vec![vec![9, 1], vec![1, 1]];
        let expected = vec![
            vec![9, 1, 1, 2, 2, 3, 3, 4, 4, 5],
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
            vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6],
            vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
            vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 7],
            vec![3, 3, 4, 4, 5, 5, 6, 6, 7, 7],
            vec![3, 4, 4, 5, 5, 6, 6, 7, 7, 8],
            vec![4, 4, 5, 5, 6, 6, 7, 7, 8, 8],
            vec![4, 5, 5, 6, 6, 7, 7, 8, 8, 9],
            vec![5, 5, 6, 6, 7, 7, 8, 8, 9, 9],
        ];
        assert_eq!(expected, expand_graph_5_times(input));
    }

    #[test]
    fn example_part_1() {
        let input = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 40;
        assert_eq!(expected, part_1_solution(input));
    }

    #[test]
    fn custom_example_part_1() {
        let input = vec![
            vec![1, 10, 1, 1, 1],
            vec![1, 1, 1, 10, 1],
            vec![10, 10, 10, 10, 1],
            vec![10, 10, 10, 10, 1],
            vec![10, 10, 10, 10, 1],
        ];
        let expected = 10;
        assert_eq!(expected, part_1_solution(input));
    }

    #[test]
    fn input_part_1() {
        let input = read_to_vec_per_line(P.input_path(), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 487;
        assert_eq!(expected, part_1_solution(input));
    }

    #[test]
    fn example_part_2() {
        let input = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 315;
        assert_eq!(expected, part_2_solution(input));
    }

    #[test]
    fn input_part_2() {
        // TODO: improve performance
        let input = read_to_vec_per_line(P.input_path(), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 2821;
        assert_eq!(expected, part_2_solution(input));
    }
}
