use std::{cell::RefCell, collections::VecDeque, str::FromStr};

use eyre::{bail, Context, ContextCompat, Report, Result};
use once_cell::sync::OnceCell;
use regex::Regex;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day11.txt")?;

    {
        let mut monkeys = data.parse::<Monkeys>()?;
        (0..20).for_each(|_| monkeys.round(true));
        let monkey_business_level = monkeys.monkey_business_level();
        println!("Part 1: monkey_business_level = {monkey_business_level}");
    }

    {
        let mut monkeys = data.parse::<Monkeys>()?;
        (0..10000).for_each(|_| monkeys.round(false));
        let monkey_business_level = monkeys.monkey_business_level();
        println!("Part 2: monkey_business_level = {monkey_business_level}");
    }

    Ok(())
}

pub struct Monkeys(Vec<Monkey>);

impl FromStr for Monkeys {
    type Err = Report;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let lines = data.lines().collect::<Vec<_>>();

        let monkeys = lines
            .chunks(7)
            .map(Monkey::parse)
            .collect::<Result<Vec<_>>>()?;
        let monkeys = Monkeys(monkeys);
        Ok(monkeys)
    }
}

static FACTORS: OnceCell<usize> = OnceCell::new();
impl Monkeys {
    pub fn round(&mut self, with_relief: bool) {
        let factors =
            FACTORS.get_or_init(|| self.0.iter().map(|m| m.div_test.0).product::<usize>());

        for monkey in &self.0 {
            while let Some(Item(worry_level)) = &monkey.pop_item() {
                let mut new = monkey.operation.eval(*worry_level);
                if with_relief {
                    new /= 3;
                } else {
                    new %= factors;
                }
                let throw_to = monkey.check_worry_level(new);
                self.0[throw_to].send_item(Item(new));
            }
        }
    }

    pub fn monkey_business_level(&mut self) -> usize {
        let mut inspections = self
            .0
            .iter()
            .map(|m| *m.num_inspections.borrow())
            .collect::<Vec<_>>();
        inspections.sort();
        inspections.reverse();
        inspections.into_iter().take(2).product::<usize>()
    }
}

fn parse_monkey_id(line: &str) -> Result<usize> {
    let id = line
        .strip_prefix("Monkey ")
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse::<usize>()?;
    Ok(id)
}

fn parse_items(line: &str) -> Result<VecDeque<Item>> {
    let numbers = line
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|n| {
            n.parse::<usize>()
                .map(Item)
                .context("Failed to parse number")
        })
        .collect::<Result<VecDeque<Item>>>()?;
    Ok(numbers)
}

pub struct Monkey {
    pub id: usize,
    pub starting_items: RefCell<VecDeque<Item>>,
    pub operation: Operation,
    pub div_test: DivisibilityTest,
    pub throw_to_if_true: usize,
    pub throw_to_if_false: usize,
    pub num_inspections: RefCell<usize>,
}

impl Monkey {
    pub fn parse(lines: &[&str]) -> Result<Self> {
        let id = parse_monkey_id(lines[0])?;
        let items = parse_items(lines[1])?;
        let operation = lines[2].parse::<Operation>()?;
        let div_test = lines[3].parse::<DivisibilityTest>()?;
        let if_true = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .context("Invalid if true")?
            .parse::<usize>()?;
        let if_false = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .context("Invalid if false")?
            .parse::<usize>()?;

        Ok(Self {
            id,
            starting_items: RefCell::new(items),
            operation,
            div_test,
            throw_to_if_true: if_true,
            throw_to_if_false: if_false,
            num_inspections: RefCell::new(0),
        })
    }

    pub fn check_worry_level(&self, worry_level: usize) -> usize {
        if worry_level % self.div_test.0 == 0 {
            self.throw_to_if_true
        } else {
            self.throw_to_if_false
        }
    }

    pub fn pop_item(&self) -> Option<Item> {
        let mut items = self.starting_items.borrow_mut();
        let item = items.pop_front();
        if item.is_some() {
            self.num_inspections.replace_with(|v| *v + 1);
        }
        item
    }

    pub fn send_item(&self, item: Item) {
        let mut items = self.starting_items.borrow_mut();

        items.push_back(item);
    }
}

pub struct Item(usize);

pub enum Operation {
    Add(Val, Val),
    Mult(Val, Val),
}

impl Operation {
    pub fn eval(&self, old: usize) -> usize {
        match self {
            Operation::Add(lhs, rhs) => lhs.eval(old) + rhs.eval(old),
            Operation::Mult(lhs, rhs) => lhs.eval(old) * rhs.eval(old),
        }
    }
}

static REGEX: OnceCell<Regex> = OnceCell::new();
impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .strip_prefix("  Operation: new = ")
            .context("Invalid operation")?;

        let re = REGEX
            .get_or_try_init(|| Regex::new(r"(.*) (\+|\*) (.*)"))
            .unwrap();
        let matches = re.captures(line).unwrap();
        let lhs = matches
            .get(1)
            .context("invalid op")
            .and_then(|v| v.as_str().parse::<Val>())?;
        let op = matches.get(2).context("invalid op")?.as_str();
        let rhs = matches
            .get(3)
            .context("invalid op")
            .and_then(|v| v.as_str().parse::<Val>())?;

        let op = match op {
            "+" => Self::Add(lhs, rhs),
            "*" => Self::Mult(lhs, rhs),
            _ => bail!("Invalid operator"),
        };
        Ok(op)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Val {
    Old,
    Literal(usize),
}

impl Val {
    pub fn eval(&self, old: usize) -> usize {
        match self {
            Val::Old => old,
            Val::Literal(v) => *v,
        }
    }
}

impl FromStr for Val {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = if s == "old" {
            Self::Old
        } else {
            s.parse::<usize>().map(Self::Literal)?
        };
        Ok(v)
    }
}

pub struct DivisibilityTest(usize);

impl FromStr for DivisibilityTest {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let div_by = s
            .strip_prefix("  Test: divisible by ")
            .context("Invalid divisibility test")?
            .parse::<usize>()?;
        Ok(Self(div_by))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let mut monkeys = data.parse::<Monkeys>().unwrap();
        (0..20).for_each(|_| monkeys.round(true));
        let monkey_business_level = monkeys.monkey_business_level();
        assert_eq!(monkey_business_level, 10605);

        let mut monkeys = data.parse::<Monkeys>().unwrap();
        (0..10000).for_each(|_| monkeys.round(false));
        let monkey_business_level = monkeys.monkey_business_level();
        assert_eq!(monkey_business_level, 2713310158);
    }
}
