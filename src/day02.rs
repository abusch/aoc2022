use std::cmp::Ordering;

use eyre::{eyre, Report, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn chose_shape(&self, outcome: Outcome) -> Shape {
        match (self, outcome) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = match (self, other) {
            (Shape::Rock, Shape::Rock) => Ordering::Equal,
            (Shape::Rock, Shape::Paper) => Ordering::Less,
            (Shape::Rock, Shape::Scissors) => Ordering::Greater,
            (Shape::Paper, Shape::Rock) => Ordering::Greater,
            (Shape::Paper, Shape::Paper) => Ordering::Equal,
            (Shape::Paper, Shape::Scissors) => Ordering::Less,
            (Shape::Scissors, Shape::Rock) => Ordering::Less,
            (Shape::Scissors, Shape::Paper) => Ordering::Greater,
            (Shape::Scissors, Shape::Scissors) => Ordering::Equal,
        };
        Some(res)
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Impossible")
    }
}

impl TryFrom<u8> for Shape {
    type Error = Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err(eyre!("Invalid shape".to_string())),
        }
    }
}

struct Round(Shape, Shape);

impl Round {
    pub fn score(&self) -> u64 {
        let round_score = match self.0.cmp(&self.1) {
            Ordering::Less => 6,    // I win
            Ordering::Equal => 3,   // Draw
            Ordering::Greater => 0, // I lost
        };
        round_score + self.1.score()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<u8> for Outcome {
    type Error = Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::Lose),
            b'Y' => Ok(Self::Draw),
            b'Z' => Ok(Self::Win),
            _ => Err(eyre!("Invalid outcome".to_string())),
        }
    }
}

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day02.txt")?;
    let letters = data
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let shape1 = bytes[0];
            let shape2 = bytes[2];
            (shape1, shape2)
        })
        .collect::<Vec<_>>();

    let rounds = letters
        .iter()
        .map(|(a, b)| {
            let shape1 = Shape::try_from(*a)?;
            let shape2 = Shape::try_from(*b)?;
            Ok(Round(shape1, shape2))
        })
        .collect::<Result<Vec<Round>>>()?;

    let total_score = rounds.iter().map(|r| r.score()).sum::<u64>();
    println!("Total score: {total_score}");

    let rounds = letters
        .iter()
        .map(|(a, b)| {
            let theirs = Shape::try_from(*a)?;
            let outcome = Outcome::try_from(*b)?;
            let ours = theirs.chose_shape(outcome);

            Ok(Round(theirs, ours))
        })
        .collect::<Result<Vec<Round>>>()?;

    let total_score = rounds.iter().map(|r| r.score()).sum::<u64>();
    println!("Total score: {total_score}");

    Ok(())
}
