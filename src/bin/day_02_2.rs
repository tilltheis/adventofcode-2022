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

fn strategy_guide_from_file(path: &str) -> Vec<(Move, Outcome)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut strategy_guide: Vec<(Move, Outcome)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let their_move = char_to_move(line.chars().nth(0).unwrap());
        let desired_outcome = char_to_outcome(line.chars().nth(2).unwrap());
        strategy_guide.push((their_move, desired_outcome))
    }

    strategy_guide
}

fn char_to_move(char: char) -> Move {
    match char {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!(),
    }
}

fn char_to_outcome(char: char) -> Outcome {
    match char {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
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

fn decide_our_move(their_move: &Move, desired_outcome: &Outcome) -> Move {
    match (their_move, desired_outcome) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Loss) => Move::Scissors,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Loss) => Move::Paper,
        (Move::Scissors, Outcome::Draw) => Move::Scissors,
    }
}

fn total_score(strategy_guide: &Vec<(Move, Outcome)>) -> i32 {
    strategy_guide
        .iter()
        .map(|(their_move, desired_outcome)| {
            let our_move = decide_our_move(their_move, desired_outcome);
            move_score(&our_move) + outcome_score(&outcome(their_move, &our_move))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{strategy_guide_from_file, total_score, Move, Outcome};

    #[test]
    fn strategy_guide_from_file_works() {
        assert_eq!(
            strategy_guide_from_file("src/bin/day_02_test_input.txt"),
            vec![
                (Move::Rock, Outcome::Draw),
                (Move::Paper, Outcome::Loss),
                (Move::Scissors, Outcome::Win)
            ]
        );
    }

    #[test]
    fn total_score_works() {
        assert_eq!(
            total_score(&vec![
                (Move::Rock, Outcome::Draw),
                (Move::Paper, Outcome::Loss),
                (Move::Scissors, Outcome::Win)
            ]),
            12
        );
    }
}
