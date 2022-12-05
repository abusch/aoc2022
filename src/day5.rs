use std::{collections::VecDeque, fmt::Display};

use color_eyre::{eyre::ContextCompat, Result};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Default)]
struct Stacks([VecDeque<u8>; 9]);

impl Stacks {
    pub fn top_crates(&self) -> String {
        let mut res = String::new();
        (0..9).for_each(|i| {
            res.push(self.0[i].back().cloned().unwrap_or(b' ').into());
        });
        res
    }

    pub fn apply_move1(&mut self, &Move(num, from, to): &Move) {
        for _ in 0..num {
            let c = self.0[from - 1].pop_back().expect("Invalid input");
            self.0[to - 1].push_back(c);
        }
    }

    pub fn apply_move2(&mut self, &Move(num, from, to): &Move) {
        let len = self.0[from - 1].len();
        let bounds_from = len - num;
        // Clippy suggests removing collect(), but actually can't here as it would require 2
        // mutable references to `self.0` at the same time. `rustc` doesn't know that `from` !=
        // `to` :(
        #[allow(clippy::needless_collect)]
        let range = self.0[from - 1].drain(bounds_from..).collect::<Vec<_>>();
        self.0[to - 1].extend(range.into_iter());
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            write!(f, "Stack {}: ", i + 1)?;
            for c in self.0[i].iter() {
                write!(f, "[{}]", *c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day5.txt")?;

    // Parse initial stacks configuration
    let mut orig = Stacks::default();
    for line in data.lines().take(8) {
        let bytes = line.as_bytes();
        (0..9).for_each(|stack| {
            let idx = stack * 4 + 1;
            if bytes[idx] != b' ' {
                orig.0[stack].push_front(bytes[idx]);
            }
        });
    }

    // Parse list of moves
    let moves = data
        .lines()
        .skip(10)
        .map(parse_move)
        .collect::<Result<Vec<_>>>()?;

    // Part 1
    let mut stacks = orig.clone();
    moves.iter().for_each(|m| stacks.apply_move1(m));
    println!("Part 1: {}", stacks.top_crates());

    // Part 2
    let mut stacks = orig;
    moves.iter().for_each(|m| stacks.apply_move2(m));
    println!("Part 2: {}", stacks.top_crates());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Move(usize, usize, usize);

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap());
fn parse_move(line: &str) -> Result<Move> {
    let captures = REGEX.captures(line).context("Bad input")?;
    let num = captures
        .get(1)
        .and_then(|n| n.as_str().parse::<usize>().ok())
        .context("Bad input")?;
    let from = captures
        .get(2)
        .and_then(|n| n.as_str().parse::<usize>().ok())
        .context("Bad input")?;
    let to = captures
        .get(3)
        .and_then(|n| n.as_str().parse::<usize>().ok())
        .context("Bad input")?;
    Ok(Move(num, from, to))
}
