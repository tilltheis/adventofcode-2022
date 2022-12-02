extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        most_calories(all_calorie_groups_from_file("src/bin/day_01_input.txt"))
    );
}

fn all_calorie_groups_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut all_calorie_groups: Vec<Vec<i32>> = vec![];
    let mut calorie_group: Vec<i32> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            all_calorie_groups.push(calorie_group);
            calorie_group = vec![];
        } else {
            calorie_group.push(line.parse::<i32>().unwrap())
        }
    }
    all_calorie_groups.push(calorie_group);

    all_calorie_groups
}

fn most_calories(all_calorie_groups: Vec<Vec<i32>>) -> i32 {
    let mut sums = all_calorie_groups
        .iter()
        .map(|arr| arr.iter().sum())
        .collect::<Vec<i32>>();
    sums.sort_unstable_by(|x, y| y.cmp(x));
    sums.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::{all_calorie_groups_from_file, most_calories};

    #[test]
    fn all_calorie_groups_from_file_works() {
        assert_eq!(
            all_calorie_groups_from_file("src/bin/day_01_test_input.txt"),
            vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000]
            ]
        );
    }

    #[test]
    fn most_calories_works() {
        assert_eq!(
            most_calories(vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000]
            ]),
            45000
        );
    }
}
