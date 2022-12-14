extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        highest_scenic_score(&read_trees("src/bin/day_08_input.txt"))
    );
}

fn read_trees(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|x| x.unwrap()).peekable();

    let size = lines.peek().unwrap().len();
    let mut trees: Vec<Vec<u8>> = Vec::with_capacity(size);

    while let Some(line) = lines.next() {
        trees.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    trees
}

fn highest_scenic_score(trees: &Vec<Vec<u8>>) -> usize {
    (0..trees.len())
        .flat_map(|y| (0..trees.len()).map(move |x| scenic_score(trees, (x, y))))
        .max()
        .unwrap()
}

fn scenic_score(trees: &Vec<Vec<u8>>, tree: (usize, usize)) -> usize {
    // (x,y) = (0,0) = (left, top)
    fn count_in_direction(trees: &Vec<Vec<u8>>, start: (usize, usize), step: (i32, i32)) -> usize {
        let max = trees[start.1][start.0];

        let mut i = 1;
        loop {
            let x = (start.0 as i32 + step.0 * i) as usize;
            let y = (start.1 as i32 + step.1 * i) as usize;

            if x >= trees.len() || y >= trees.len() {
                break;
            }

            i += 1;

            if trees[y][x] >= max {
                break;
            }
        }

        (i - 1) as usize
    }

    let mut score = 1;
    score *= count_in_direction(trees, tree, (0, 1)); // top to bottom
    score *= count_in_direction(trees, tree, (-1, 0)); // right to left
    score *= count_in_direction(trees, tree, (0, -1)); // bottom to top
    score *= count_in_direction(trees, tree, (1, 0)); // left to right

    score
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn create_test_trees() -> Vec<Vec<u8>> {
        vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]
    }

    #[test]
    fn read_trees_works() {
        assert_eq!(
            read_trees("src/bin/day_08_test_input.txt"),
            create_test_trees()
        );
    }

    #[test]
    fn scenic_score_works1() {
        assert_eq!(scenic_score(&create_test_trees(), (2, 1)), 4);
    }

    #[test]
    fn scenic_score_works2() {
        assert_eq!(scenic_score(&create_test_trees(), (2, 3)), 8);
    }

    #[test]
    fn highest_scenic_score_works() {
        assert_eq!(highest_scenic_score(&create_test_trees()), 8);
    }
}
