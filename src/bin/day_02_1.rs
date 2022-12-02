extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        total_score(&strategy_guide_from_file("src/bin/day_02_input.txt"))
    );
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn strategy_guide_from_file(path: &str) -> Vec<(Move, Move)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut strategy_guide: Vec<(Move, Move)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let their_move = char_to_move(line.chars().nth(0).unwrap());
        let our_move = char_to_move(line.chars().nth(2).unwrap());
        strategy_guide.push((their_move, our_move))
    }

    strategy_guide
}

fn char_to_move(char: char) -> Move {
    match char {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!(),
    }
}

fn move_score(a_move: &Move) -> i32 {
    match a_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn outcome_score(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    }
}

fn outcome(their_move: &Move, our_move: &Move) -> Outcome {
    match (their_move, our_move) {
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Rock, Move::Paper) => Outcome::Win,
        (Move::Rock, Move::Scissors) => Outcome::Loss,
        (Move::Paper, Move::Rock) => Outcome::Loss,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Paper, Move::Scissors) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Win,
        (Move::Scissors, Move::Paper) => Outcome::Loss,
        (Move::Scissors, Move::Scissors) => Outcome::Draw,
    }
}

fn total_score(strategy_guide: &Vec<(Move, Move)>) -> i32 {
    strategy_guide
        .iter()
        .map(|(their_move, our_move)| {
            move_score(our_move) + outcome_score(&outcome(their_move, our_move))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{strategy_guide_from_file, total_score, Move};

    #[test]
    fn strategy_guide_from_file_works() {
        assert_eq!(
            strategy_guide_from_file("src/bin/day_02_test_input.txt"),
            vec![
                (Move::Rock, Move::Paper),
                (Move::Paper, Move::Rock),
                (Move::Scissors, Move::Scissors)
            ]
        );
    }

    #[test]
    fn total_score_works() {
        assert_eq!(
            total_score(&vec![
                (Move::Rock, Move::Paper),
                (Move::Paper, Move::Rock),
                (Move::Scissors, Move::Scissors)
            ]),
            15
        );
    }
}
