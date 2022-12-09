extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = read_file("src/bin/day_05_input.txt");
    let stacks = read_stacks(&mut lines);
    let moves = read_moves(lines);
    let topmost_crates = rearrange_cargo(stacks, moves);
    println!("{}", topmost_crates);
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|x| x.unwrap())
}

fn read_stacks(lines: &mut impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut stack_height = 0;

    let mut stack_lines: Vec<String> = vec![];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        stack_height += 1;
        stack_lines.push(line);
    }
    stack_height -= 1;

    let stack_count = (stack_lines.first().unwrap().len() + 1) / 4;

    stack_lines.remove(stack_lines.len() - 1);

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Vec::with_capacity(stack_height));
    }

    let mut reverse_stack_lines = stack_lines.iter().rev();
    while let Some(line) = reverse_stack_lines.next() {
        for i in 0..stack_count {
            let idx = i * 4 + 1;
            let krate = line.as_bytes()[idx] as char;
            if krate != ' ' {
                stacks[i].push(krate);
            }
        }
    }

    stacks
}

fn read_moves(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (usize, usize, usize)> {
    lines.map(|line| {
        let mut words = line.split(' ');
        words.next();
        let count = words.next().unwrap().parse().unwrap();
        words.next();
        let from = words.next().unwrap().parse().unwrap();
        words.next();
        let to = words.next().unwrap().parse().unwrap();
        (count, from, to)
    })
}

fn rearrange_cargo(
    mut stacks: Vec<Vec<char>>,
    moves: impl Iterator<Item = (usize, usize, usize)>,
) -> String {
    for (count, from, to) in moves {
        // the borrow checker doesn't know that 2 separate sub-vectors are not the same without splitting...
        let (from_stack, to_stack) = if from < to {
            let (first_stacks, last_stacks) = stacks.split_at_mut(to - 1);
            let from_stack = &mut first_stacks[from - 1];
            let to_stack = &mut last_stacks[0];
            (from_stack, to_stack)
        } else {
            let (first_stacks, last_stacks) = stacks.split_at_mut(from - 1);
            let from_stack = &mut last_stacks[0];
            let to_stack = &mut first_stacks[to - 1];
            (from_stack, to_stack)
        };

        let from_len = from_stack.len();
        to_stack.extend_from_slice(&from_stack[(from_len - count)..]);
        from_stack.truncate(from_len - count);
    }

    let mut topmost_crates = String::with_capacity(stacks.len());
    for stack in stacks {
        stack.last().map(|&krate| topmost_crates.push(krate));
    }

    topmost_crates
}

#[cfg(test)]
mod tests {
    use crate::{read_file, read_moves, read_stacks, rearrange_cargo};

    #[test]
    fn read_stacks_from_file_works() {
        assert_eq!(
            read_stacks(&mut read_file("src/bin/day_05_test_input.txt")),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn move_stacks_works() {
        let mut lines = read_file("src/bin/day_05_test_input.txt");
        read_stacks(&mut lines);
        let moves: Vec<(usize, usize, usize)> = read_moves(lines).collect();
        assert_eq!(moves, vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]);
    }

    #[test]
    fn rearrange_cargo_works() {
        let mut lines = read_file("src/bin/day_05_test_input.txt");
        let stacks = read_stacks(&mut lines);
        let moves = read_moves(lines);
        let topmost_crates = rearrange_cargo(stacks, moves);
        assert_eq!(topmost_crates, "MCD");
    }
}
