extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        total_priorities(&read_file("src/bin/day_03_input.txt"))
    );
}

fn read_file(path: &str) -> Vec<(String, String)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut all_backpacks: Vec<(String, String)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let compartment1 = line.chars().take(line.len() / 2).collect();
        let compartment2 = line.chars().skip(line.len() / 2).collect();
        all_backpacks.push((compartment1, compartment2))
    }

    all_backpacks
}

fn item_priority(item: char) -> i32 {
    // uppercase letters in ascii table come first
    if (item as u32) < ('a' as u32) {
        ((item as u32) - ('A' as u32) + 27) as i32
    } else {
        ((item as u32) - ('a' as u32) + 1) as i32
    }
}

fn total_priorities(all_backpacks: &Vec<(String, String)>) -> i32 {
    all_backpacks
        .iter()
        .map(|(compartment1, compartment2)| {
            item_priority(
                compartment1
                    .chars()
                    .find(|&c| compartment2.contains(c))
                    .unwrap(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{item_priority, read_file, total_priorities};

    #[test]
    fn read_file_works() {
        assert_eq!(
            read_file("src/bin/day_03_test_input.txt"),
            vec![
                ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
                (
                    "jqHRNqRjqzjGDLGL".to_string(),
                    "rsFMfFZSrLrFZsSL".to_string()
                ),
                ("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
                ("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()),
                ("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
                ("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()),
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
    fn total_priorities_works() {
        assert_eq!(
            total_priorities(&vec![
                ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
                (
                    "jqHRNqRjqzjGDLGL".to_string(),
                    "rsFMfFZSrLrFZsSL".to_string()
                ),
                ("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
                ("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()),
                ("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
                ("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()),
            ]),
            157
        );
    }
}
