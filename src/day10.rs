use std::{fmt::Display, str::FromStr};

use eyre::{bail, ContextCompat, Report, Result};
use itertools::Itertools;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day10.txt")?;
    let program = data
        .lines()
        .map(|line| line.parse::<Inst>())
        .collect::<Result<Vec<_>>>()?;

    let mut cpu = Cpu::new();
    cpu.run(&program);

    println!("Part 1: {}", cpu.interesting_signals());

    println!("Part 2:\n{}", cpu.crt);
    Ok(())
}

struct Crt {
    pixels: [char; 40 * 6],
}

impl Crt {
    pub fn new() -> Self {
        Self {
            pixels: ['.'; 40 * 6],
        }
    }

    pub fn tick(&mut self, cycle: usize, sprite_pos: isize) {
        let row = cycle / 40;
        let pos = cycle % 40;

        if (sprite_pos - pos as isize).abs() <= 1 {
            self.draw(pos, row);
        }
    }

    pub fn draw(&mut self, x: usize, y: usize) {
        self.pixels[y * 40 + x] = '#';
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .pixels
            .chunks(40)
            .map(|line| line.iter().collect::<String>())
            .join("\n");
        write!(f, "{}", s)
    }
}

struct Cpu {
    x: isize,
    cycle: usize,
    signal: Vec<isize>,
    crt: Crt,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            signal: vec![1],
            crt: Crt::new(),
        }
    }

    pub fn run(&mut self, program: &[Inst]) {
        program.iter().for_each(|i| self.step(i));
    }

    pub fn step(&mut self, inst: &Inst) {
        let cycles = inst.cycles();
        (0..cycles).for_each(|_| self.cycle());

        match inst {
            Inst::AddX(v) => self.x += v,
            Inst::Noop => (),
        }
    }

    pub fn cycle(&mut self) {
        self.crt.tick(self.cycle, self.x);
        self.cycle += 1;
        self.signal.push(self.cycle as isize * self.x);
    }

    pub fn interesting_signals(&self) -> isize {
        self.signal
            .iter()
            .skip(20)
            .step_by(40)
            .take(6)
            .sum::<isize>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Inst {
    AddX(isize),
    Noop,
}

impl Inst {
    fn cycles(&self) -> usize {
        match self {
            Inst::AddX(_) => 2,
            Inst::Noop => 1,
        }
    }
}

impl FromStr for Inst {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            _ => {
                let (inst, v) = s.split_once(' ').context("Invalid instruction")?;
                if inst != "addx" {
                    bail!("Invalid instruction");
                }
                let v = v.parse::<isize>()?;
                Ok(Self::AddX(v))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let program = data
            .lines()
            .map(|line| line.parse::<Inst>())
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let mut cpu = Cpu::new();
        cpu.run(&program);

        assert_eq!(cpu.signal[20], 420);
        assert_eq!(cpu.signal[60], 1140);
        assert_eq!(cpu.signal[100], 1800);
        assert_eq!(cpu.signal[140], 2940);
        assert_eq!(cpu.signal[180], 2880);
        assert_eq!(cpu.signal[220], 3960);

        assert_eq!(cpu.interesting_signals(), 13140);

        let expected_crt = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let crt = format!("{}", cpu.crt);
        println!("expected:\n{expected_crt}\n\nactual:\n{crt}");

        assert_eq!(crt, expected_crt);
    }
}
