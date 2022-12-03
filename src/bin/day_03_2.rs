extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        total_badge_priorities(&read_file("src/bin/day_03_input.txt"))
    );
}

fn read_file(path: &str) -> Vec<(String, String, String)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut all_group_backpacks: Vec<(String, String, String)> = vec![];
    let mut lines = reader.lines().peekable();
    while lines.peek().is_some() {
        all_group_backpacks.push((
            lines.next().unwrap().unwrap(),
            lines.next().unwrap().unwrap(),
            lines.next().unwrap().unwrap(),
        ));
    }

    all_group_backpacks
}

fn item_priority(item: char) -> i32 {
    // uppercase letters in ascii table come first
    if (item as u32) < ('a' as u32) {
        ((item as u32) - ('A' as u32) + 27) as i32
    } else {
        ((item as u32) - ('a' as u32) + 1) as i32
    }
}

fn total_badge_priorities(all_group_backpacks: &Vec<(String, String, String)>) -> i32 {
    all_group_backpacks
        .iter()
        .map(|(backpack1, backpack2, backpack3)| {
            item_priority(
                backpack1
                    .chars()
                    .find(|&c| backpack2.contains(c) && backpack3.contains(c))
                    .unwrap(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{item_priority, read_file, total_badge_priorities};

    #[test]
    fn read_file_works() {
        assert_eq!(
            read_file("src/bin/day_03_test_input.txt"),
            vec![
                (
                    "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                    "PmmdzqPrVvPwwTWBwg".to_string()
                ),
                (
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                    "ttgJtRGJQctTZtZT".to_string(),
                    "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
                ),
            ]
        );
    }

    #[test]
    fn item_priority_works() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }

    #[test]
    fn total_badge_priorities_works() {
        assert_eq!(
            total_badge_priorities(&vec![
                (
                    "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                    "PmmdzqPrVvPwwTWBwg".to_string()
                ),
                (
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                    "ttgJtRGJQctTZtZT".to_string(),
                    "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
                ),
            ]),
            70
        );
    }
}
