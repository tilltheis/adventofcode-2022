extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() {
    println!(
        "{}",
        overlapping_assignment_count(&read_file_into_section_assignments(
            "src/bin/day_04_input.txt"
        ))
    );
}

fn read_file_into_section_assignments(
    path: &str,
) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut all_section_assignments: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(',');
        let assignment1 = parse_section_assignment(split.next().unwrap());
        let assignment2 = parse_section_assignment(split.next().unwrap());
        all_section_assignments.push((assignment1, assignment2));
    }

    all_section_assignments
}

fn parse_section_assignment(string: &str) -> RangeInclusive<u32> {
    let mut split = string.split('-');
    let start = split.next().unwrap().parse().unwrap();
    let end = split.next().unwrap().parse().unwrap();
    start..=end
}

fn overlapping_assignment_count(
    all_section_assignments: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
) -> usize {
    all_section_assignments
        .iter()
        .filter(|(assignment1, assignment2)| {
            (assignment1.start() >= assignment2.start() && assignment1.start() <= assignment2.end())
                || (assignment2.start() >= assignment1.start()
                    && assignment2.start() <= assignment1.end())
                || (assignment1.start() <= assignment2.end()
                    && assignment1.end() >= assignment2.end())
                || (assignment2.start() <= assignment1.end()
                    && assignment2.end() >= assignment1.end())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{overlapping_assignment_count, read_file_into_section_assignments};

    #[test]
    fn read_file_into_section_assignments_works() {
        assert_eq!(
            read_file_into_section_assignments("src/bin/day_04_test_input.txt"),
            vec![
                ((2..=4), (6..=8)),
                ((2..=3), (4..=5)),
                ((5..=7), (7..=9)),
                ((2..=8), (3..=7)),
                ((6..=6), (4..=6)),
                ((2..=6), (4..=8)),
            ]
        );
    }

    #[test]
    fn fully_contained_assignment_count_works() {
        assert_eq!(
            overlapping_assignment_count(&vec![
                ((2..=4), (6..=8)),
                ((2..=3), (4..=5)),
                ((5..=7), (7..=9)),
                ((2..=8), (3..=7)),
                ((6..=6), (4..=6)),
                ((2..=6), (4..=8)),
            ]),
            4
        );
    }
}
