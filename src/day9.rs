use std::{collections::HashSet, ops::Sub, str::FromStr};

use eyre::{eyre, Report, Result};

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day9.txt")?;
    let moves = data.parse::<Moves>()?;

    let mut grid = Grid::<1>::new();
    grid.apply(&moves);
    let distinct_positions = grid.tail_positions.len();
    println!("Part 1: {distinct_positions} distinct tail positions with 2 knots");

    let mut grid = Grid::<9>::new();
    grid.apply(&moves);
    let distinct_positions = grid.tail_positions.len();
    println!("Part 2: {distinct_positions} distinct tail positions with 10 knots");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Pos(isize, isize);

impl Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Pos {
    pub fn move_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.1 += 1,
            Dir::Down => self.1 -= 1,
            Dir::Left => self.0 -= 1,
            Dir::Right => self.0 += 1,
        }
    }
}

#[derive(Debug)]
pub struct Grid<const N: usize> {
    head: Pos,
    // tail: Pos,
    tails: [Pos; N],
    tail_positions: HashSet<Pos>,
}

impl<const N: usize> Grid<N> {
    pub fn new() -> Self {
        let mut grid = Self {
            head: Pos::default(),
            tails: [Pos::default(); N],
            tail_positions: HashSet::default(),
        };

        grid.record_tail_pos();

        grid
    }

    pub fn apply(&mut self, moves: &Moves) {
        moves.0.iter().for_each(|m| self.apply_move(*m));
    }

    pub fn apply_move(&mut self, Move(dir, num): Move) {
        (0..num).for_each(|_| self.move_head(dir))
    }

    /// Move the head 1 unit into the given direction
    pub fn move_head(&mut self, dir: Dir) {
        self.head.move_dir(dir);

        // Move tail accordingly
        self.move_tail(self.head, 0);
        for i in 1..N {
            self.move_tail(self.tails[i - 1], i);
        }

        // Record new tail position
        self.record_tail_pos();
    }

    fn move_tail(&mut self, head_pos: Pos, tail_num: usize) {
        let tail = &mut self.tails[tail_num];
        let Pos(dx, dy) = head_pos - *tail;
        match (dx, dy) {
            // Touching
            (0, 0) => (),
            (x, y) if x.abs() <= 1 && y.abs() <= 1 => (),
            // 2 units directly up/down/left/right
            (0, 2) => tail.move_dir(Dir::Up),
            (0, -2) => tail.move_dir(Dir::Down),
            (2, 0) => tail.move_dir(Dir::Right),
            (-2, 0) => tail.move_dir(Dir::Left),
            // Other cases
            (dx, dy) => {
                let horizontal = if dx > 0 { Dir::Right } else { Dir::Left };
                let vertical = if dy > 0 { Dir::Up } else { Dir::Down };
                tail.move_dir(horizontal);
                tail.move_dir(vertical);
            }
        }
    }

    fn record_tail_pos(&mut self) {
        self.tail_positions.insert(*self.tails.last().unwrap());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(eyre!("Invalid direction")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move(Dir, usize);

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_once(' ').ok_or_else(|| eyre!("Invalid move"))?;
        let dir = dir.parse::<Dir>()?;
        let num = num.parse::<usize>()?;

        Ok(Self(dir, num))
    }
}

pub struct Moves(Vec<Move>);

impl FromStr for Moves {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .lines()
            .map(|line| line.parse::<Move>())
            .collect::<Result<Vec<Move>>>()?;

        Ok(Self(moves))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_knots() {
        let moves = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .parse::<Moves>()
        .unwrap();

        let mut grid = Grid::<1>::new();
        grid.apply(&moves);
        let count = grid.tail_positions.len();
        assert_eq!(count, 13);
    }

    #[test]
    fn test_10_knots() {
        let moves = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .parse::<Moves>()
            .unwrap();
        let mut grid = Grid::<9>::new();
        grid.apply(&moves);
        let count = grid.tail_positions.len();

        assert_eq!(count, 36);
    }
}
