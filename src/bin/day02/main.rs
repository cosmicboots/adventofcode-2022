#![allow(dead_code)]

use std::{fs::File, io::Read};

const PART2: bool = true;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    
    fn beats(&self) -> Self {
        match self {
            Move::Rock => Self::Scissors,
            Move::Paper => Self::Rock,
            Move::Scissors => Self::Paper,
        }
    }

    fn beat_by(&self) -> Self {
        match self {
            Move::Rock => Self::Paper,
            Move::Paper => Self::Scissors,
            Move::Scissors => Self::Rock,
        }
    }
}

impl From<&str> for Move {
    fn from(m: &str) -> Self {
        match m {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            c => panic!("Invalid move: {}", c),
        }
    }
}

#[derive(Debug)]
struct Turn {
    their: Move,
    mine: Move,
}

impl Turn {
    fn win(&self) -> Option<bool> {
        if self.mine == self.their {
            return None;
        }
        let target = match self.mine {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        };
        Some(target == self.their)
    }

    fn play(&self) -> usize {
        let win = self.win(); // False is opponent win, None is tie
        let win_score = match win {
            Some(x) => match x {
                true => 6,
                false => 0,
            },
            None => 3,
        };
        win_score + self.mine.get_score()
    }
}

impl From<&str> for Turn {
    fn from(turn: &str) -> Self {
        let their: Move;
        let mine: Move;
        if PART2 {
            let moves: Vec<&str> = turn.trim().split(' ').collect();
            their = Move::from(moves[0]);
            mine = match moves[1] {
                "X" => their.beats(),
                "Y" => their,
                "Z" => their.beat_by(),
                x => panic!("Invalid move: {}", x),
            }
        } else {
            let moves: Vec<Move> = turn
                .trim()
                .split(' ')
                .map(|e| Move::from(e.trim()))
                .collect();
            their = moves[0];
            mine = moves[1];
        }
        Self { their, mine }
    }
}

#[derive(Debug)]
struct Game {
    turns: Vec<Turn>,
}

impl From<&str> for Game {
    fn from(game: &str) -> Self {
        let turns = game.trim().split('\n').map(|e| Turn::from(e)).collect();
        Self { turns }
    }
}

impl Game {
    fn check_my_score(&self) -> usize {
        self.turns.iter().map(|e| e.play()).sum()
    }
}

fn main() {
    println!("Hello from day02!");

    let mut file = File::open("src/bin/day02/input.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let game = Game::from(text.as_str());

    println!("Predicted game score: {:?}", game.check_my_score());
}
