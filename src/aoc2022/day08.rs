/// problem: https://adventofcode.com/2022/day/8
/// input: https://adventofcode.com/2022/day/8/input
use crate::Problem;
const P: Problem = Problem {
    year: 2022,
    day: 8,
    name: "Treetop Tree House",
};
fn get_externally_visible(tree_heights: &Vec<Vec<usize>>) -> Vec<Vec<bool>> {
    let row_count = tree_heights.len();
    let col_count = tree_heights[0].len();
    let mut visibilities = vec![vec![false; col_count]; row_count];

    let max_row = row_count - 1;
    let max_col = col_count - 1;
    for row in 0..row_count {
        for col in 0..col_count {
            // north
            let mut is_visible = row == 0 || col == 0 || row == max_row || col == max_col;
            let h = tree_heights[row][col];

            let mut row_temp = row;
            while row_temp > 0 {
                row_temp -= 1;
                if tree_heights[row_temp][col] >= h {
                    break;
                }
                if row_temp == 0 {
                    is_visible = true;
                }
            }

            let mut row_temp = row;
            while row_temp < max_row {
                row_temp += 1;
                if tree_heights[row_temp][col] >= h {
                    break;
                }
                if row_temp == max_row {
                    is_visible = true;
                }
            }

            let mut col_temp = col;
            while col_temp > 0 {
                col_temp -= 1;
                if tree_heights[row][col_temp] >= h {
                    break;
                }
                if col_temp == 0 {
                    is_visible = true;
                }
            }

            let mut col_temp = col;
            while col_temp < max_col {
                col_temp += 1;
                if tree_heights[row][col_temp] >= h {
                    break;
                }
                if col_temp == max_col {
                    is_visible = true;
                }
            }
            visibilities[row][col] = is_visible;
        }
    }
    visibilities
}

fn get_scenic_scores(tree_heights: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let row_count = tree_heights.len();
    let col_count = tree_heights[0].len();
    let mut scores = vec![vec![0; col_count]; row_count];

    for row in 0..row_count {
        for col in 0..col_count {
            // north
            let mut score = 1;
            let h = tree_heights[row][col];

            let mut row_temp = row;
            let mut score_temp = 0;
            while row_temp > 0 {
                row_temp -= 1;
                score_temp += 1;
                if tree_heights[row_temp][col] >= h {
                    break;
                }
            }
            score *= score_temp;

            let mut row_temp = row;
            let mut score_temp = 0;
            while row_temp < row_count - 1 {
                row_temp += 1;
                score_temp += 1;
                if tree_heights[row_temp][col] >= h {
                    break;
                }
            }
            score *= score_temp;

            let mut col_temp = col;
            let mut score_temp = 0;
            while col_temp > 0 {
                col_temp -= 1;
                score_temp += 1;
                if tree_heights[row][col_temp] >= h {
                    break;
                }
            }
            score *= score_temp;

            let mut col_temp = col;
            let mut score_temp = 0;
            while col_temp < col_count - 1 {
                col_temp += 1;
                score_temp += 1;
                if tree_heights[row][col_temp] >= h {
                    break;
                }
            }
            score *= score_temp;
            scores[row][col] = score;
        }
    }
    scores
}

pub fn part_1_solution(tree_heights: Vec<Vec<usize>>) -> usize {
    // initialize the visited
    let visibility = get_externally_visible(&tree_heights);
    visibility.iter().flatten().filter(|&f| *f).count()
}

pub fn part_2_solution(tree_heights: Vec<Vec<usize>>) -> usize {
    let scores = get_scenic_scores(&tree_heights);
    *scores.iter().flatten().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::read_to_vec_per_line;

    #[test]
    fn example_part_1() {
        let input = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 21;
        assert_eq!(expected, part_1_solution(input))
    }

    #[test]
    fn input_part_1() {
        let input = read_to_vec_per_line(P.input_path(), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 1807;
        assert_eq!(expected, part_1_solution(input))
    }

    #[test]
    fn example_part_2() {
        let input = read_to_vec_per_line(P.example_path("_1"), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 8;
        assert_eq!(expected, part_2_solution(input))
    }

    #[test]
    fn input_part_2() {
        let input = read_to_vec_per_line(P.input_path(), |c| {
            Some(c.to_digit(10).expect("should be usize") as usize)
        })
        .expect("unable to open file");
        let expected = 480000;
        assert_eq!(expected, part_2_solution(input))
    }
}
