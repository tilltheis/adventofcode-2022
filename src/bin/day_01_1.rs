extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    println!("{}", go("src/bin/day_01_input.txt"));
}

fn go(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Lines<BufReader<File>> = reader.lines();

    let mut max = 0;
    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            max = max.max(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::go;

    #[test]
    fn go_works() {
        assert_eq!(go("src/bin/day_01_test_input.txt"), 24000);
    }
}
