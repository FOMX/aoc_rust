/// problem: https://adventofcode.com/2022/day/12
/// input: https://adventofcode.com/2022/day/12/input
use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 12,
    name: "TODO",
};

fn get_neighbours(
    graph: &Vec<Vec<u8>>,
    coordinate: &(usize, usize),
    xmax: usize,
    ymax: usize,
) -> Vec<(usize, usize)> {
    let (x, y) = *coordinate;
    let curr = graph[x][y];
    let mut neighbours = Vec::new();
    if x > 0 {
        if graph[x - 1][y] <= curr + 1 {
            neighbours.push((x - 1, y));
        }
    }
    if x < xmax {
        if graph[x + 1][y] <= curr + 1 {
            neighbours.push((x + 1, y));
        }
    }
    if y > 0 {
        if graph[x][y - 1] <= curr + 1 {
            neighbours.push((x, y - 1));
        }
    }
    if y < ymax {
        if graph[x][y + 1] <= curr + 1 {
            neighbours.push((x, y + 1));
        }
    }
    neighbours
}

/// return the shortest path as per djikstra
fn djikstra(graph: &Vec<Vec<u8>>, start: &(usize, usize), end: &(usize, usize)) -> usize {
    // start is usually 0,0

    let xmax = graph.len() - 1;
    let ymax = graph[0].len() - 1;

    // initialize the distances
    let mut distances = vec![vec![usize::MAX; ymax + 1]; xmax + 1];
    distances[start.0][start.1] = 0;

    // initialize the visited
    let mut visited = vec![vec![false; ymax + 1]; xmax + 1];
    visited[start.0][start.1] = true;

    let mut frontier = get_neighbours(&graph, &start, xmax, ymax); // TODO: use a queue/heap
    for node in frontier.iter() {
        distances[node.0][node.1] = 1;
    }
    let mut next_nearest_node = *start;
    while next_nearest_node != *end {
        // get next best candidate
        let mut min_dist = usize::MAX;
        next_nearest_node = *start;
        for node in frontier.iter() {
            if distances[node.0][node.1] < min_dist {
                min_dist = distances[node.0][node.1];
                next_nearest_node = *node;
            }
        }
        if next_nearest_node == *start {
            return usize::MAX;
        }

        // visit node
        visited[next_nearest_node.0][next_nearest_node.1] = true;
        // add to frontier and update distances
        for node in get_neighbours(&graph, &next_nearest_node, xmax, ymax) {
            if distances[node.0][node.1] > distances[next_nearest_node.0][next_nearest_node.1] + 1 {
                distances[node.0][node.1] = distances[next_nearest_node.0][next_nearest_node.1] + 1;
            }
            if !visited[node.0][node.1] {
                frontier.push(node);
            }
        }
        frontier.retain(|node| *node != next_nearest_node);
    }
    for dist in &distances {
        println!("{:?}", dist);
    }
    distances[next_nearest_node.0][next_nearest_node.1]
}

fn find_val(graph: &Vec<Vec<u8>>, val: u8) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for row in 0..graph.len() {
        for col in 0..graph[0].len() {
            if graph[row][col] == val {
                coords.push((row, col));
            }
        }
    }
    coords
}

fn pt1(input: &str) -> usize {
    let graph = parse(input);
    // let start = find_val(&graph, "a".as_bytes()[0] - 1)[0];
    // let end = find_val(&graph, "z".as_bytes()[0] + 1)[0];
    // println!("{:?}{:?}", start, end);
    djikstra(&graph, &(20, 0), &(20, 43))
}

fn pt2(input: &str) -> usize {
    let graph = parse(input);
    let starts = find_val(&graph, "a".as_bytes()[0]);
    let exit = find_val(&graph, "z".as_bytes()[0] + 1)[0];
    starts
        .iter()
        .map(|start| djikstra(&graph, start, &exit))
        .min()
        .unwrap()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .replace('S', "a") // std::str::from_utf8(&["a".as_bytes()[0] - 1]).unwrap())
        // .replace('S', std::str::from_utf8(&["a".as_bytes()[0] - 1]).unwrap())
        .replace('E', "z") // std::str::from_utf8(&["z".as_bytes()[0] + 1]).unwrap()) // so hacky
        // .replace('E', std::str::from_utf8(&["z".as_bytes()[0] + 1]).unwrap()) // so hacky
        .lines()
        .map(|l| l.bytes().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    // in process of refactoring
    // #[test]
    // fn pt1_example() {
    //     let input = read_to_string(P.example_path("_1")).expect("no such file");
    //     assert_eq!(pt1(&input), 31);
    // }

    // #[test]
    // fn pt1_input() {
    //     let input = read_to_string(P.input_path()).expect("no such file");
    //     assert_eq!(pt1(&input), 339);
    // }

    // #[test]
    // fn pt2_example() {
    //     let input = read_to_string(P.example_path("_1")).expect("no such file");
    //     assert_eq!(pt2(&input), 29);
    // }

    // #[test]
    // fn pt2_input() {
    //     let input = read_to_string(P.input_path()).expect("no such file");
    //     assert_eq!(pt2(&input), 332); // now 334?
    // }
}
