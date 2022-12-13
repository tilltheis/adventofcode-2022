extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        count_visible_trees(&read_trees("src/bin/day_08_input.txt"))
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

fn count_visible_trees(trees: &Vec<Vec<u8>>) -> usize {
    // (x,y) = (0,0) = (left, top)
    fn count_in_direction(
        trees: &Vec<Vec<u8>>,
        mut visible_trees: HashSet<(usize, usize)>,
        start: (i32, i32),
        step: (i32, i32),
        jump: (i32, i32),
    ) -> HashSet<(usize, usize)> {
        for i in 0..trees.len() as i32 {
            let x = (start.0 + jump.0 * i) as usize;
            let y = (start.1 + jump.1 * i) as usize;
            let mut max = trees[y][x];
            visible_trees.insert((x, y));

            for j in 1..trees.len() as i32 {
                let x = (start.0 + jump.0 * i + step.0 * j) as usize;
                let y = (start.1 + jump.1 * i + step.1 * j) as usize;
                let height = trees[y][x];
                if height > max {
                    max = height;
                    visible_trees.insert((x, y));
                }
            }
        }

        visible_trees
    }

    let mut visible_trees = HashSet::new();
    visible_trees = count_in_direction(trees, visible_trees, (0, 0), (0, 1), (1, 0)); // top to bottom
    visible_trees = count_in_direction(
        trees,
        visible_trees,
        (trees.len() as i32 - 1, 0),
        (-1, 0),
        (0, 1),
    ); // right to left
    visible_trees = count_in_direction(
        trees,
        visible_trees,
        (0, trees.len() as i32 - 1),
        (0, -1),
        (1, 0),
    ); // bottom to top
    visible_trees = count_in_direction(trees, visible_trees, (0, 0), (1, 0), (0, 1)); // left to right

    visible_trees.len()
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
    fn count_visible_trees_works() {
        assert_eq!(count_visible_trees(&create_test_trees()), 21);
    }
}
