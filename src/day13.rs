use std::{cmp::Ordering, iter::once};

use eyre::Result;
use nom::{
    branch::alt, character::complete::char, combinator::map, multi::separated_list0,
    sequence::delimited, Finish, IResult,
};
use once_cell::sync::Lazy;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day13.txt")?;

    let packet_pairs = parse_packets(&data)?;
    let sum = count_ordered_packet_pairs(&packet_pairs);

    println!("Part 1: sum of indices = {sum}");

    let mut packets = packet_pairs
        .into_iter()
        .flat_map(|(p1, p2)| once(p1).chain(once(p2)))
        .chain(once(DELIM1.clone()))
        .chain(once(DELIM2.clone()))
        .collect::<Vec<_>>();

    packets.sort();
    let pos1 = packets.iter().position(|p| p == &*DELIM1).unwrap();
    let pos2 = packets.iter().position(|p| p == &*DELIM2).unwrap();
    let key = (pos1 + 1) * (pos2 + 1);

    println!("Part 2: decoder key = {key}");

    Ok(())
}

static DELIM1: Lazy<Value> = Lazy::new(|| Value::List(vec![Value::list_from(2)]));
static DELIM2: Lazy<Value> = Lazy::new(|| Value::List(vec![Value::list_from(6)]));

fn count_ordered_packet_pairs(packet_pairs: &[(Value, Value)]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| (p1 <= p2).then_some(i + 1))
        .sum::<usize>()
}

fn parse_packets(data: &str) -> Result<Vec<(Value, Value)>> {
    let lines = data.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let p1 = parse_packet(lines[0])?;
            let p2 = parse_packet(lines[1])?;
            Ok((p1, p2))
        })
        .collect()
}

fn parse_int(s: &str) -> IResult<&str, Value> {
    map(nom::character::complete::u32, Value::Integer)(s)
}

fn parse_list(s: &str) -> IResult<&str, Value> {
    map(
        delimited(
            char('['),
            separated_list0(char(','), parse_value),
            char(']'),
        ),
        Value::List,
    )(s)
}

fn parse_value(s: &str) -> IResult<&str, Value> {
    alt((parse_int, parse_list))(s)
}

fn parse_packet(s: &str) -> Result<Value> {
    let (_rest, res) = parse_value(s)
        .finish()
        .map_err(|_| eyre::eyre!("Parse error"))?;
    Ok(res)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    List(Vec<Value>),
}

impl Value {
    pub fn list_from(v: u32) -> Self {
        Self::List(vec![Self::Integer(v)])
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::Integer(v), Value::List(_)) => Self::list_from(*v).cmp(other),
            (Value::List(_), Value::Integer(v)) => self.cmp(&Self::list_from(*v)),
            (Value::List(l1), Value::List(l2)) => l1.cmp(l2),
        };
        Some(res)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        let p1 = parse_packet("[1,1,3,1,1]").unwrap();
        let p2 = parse_packet("[1,1,5,1,1]").unwrap();
        assert!(p1 <= p2);

        let p1 = parse_packet("[[1],[2,3,4]]").unwrap();
        let p2 = parse_packet("[[1],4]").unwrap();
        assert!(p1 <= p2);

        let p1 = parse_packet("[9]").unwrap();
        let p2 = parse_packet("[[8,7,6]]").unwrap();
        assert!(p1 > p2);

        let p1 = parse_packet("[[4,4],4,4]").unwrap();
        let p2 = parse_packet("[[4,4],4,4,4]").unwrap();
        assert!(p1 <= p2);

        let p1 = parse_packet("[7,7,7,7]").unwrap();
        let p2 = parse_packet("[7,7,7]").unwrap();
        assert!(p1 > p2);

        let p1 = parse_packet("[]").unwrap();
        let p2 = parse_packet("[3]").unwrap();
        assert!(p1 <= p2);

        let p1 = parse_packet("[[[]]]").unwrap();
        let p2 = parse_packet("[[]]").unwrap();
        assert!(p1 > p2);

        let p1 = parse_packet("[5,6,7]").unwrap();
        let p2 = parse_packet("[5,6,0]").unwrap();
        assert!(p1 > p2);

        let p1 = parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let p2 = parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert!(p1 > p2);
    }

    #[test]
    fn test_part1() {
        let data = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let packet_pairs = parse_packets(data).unwrap();
        let sum = count_ordered_packet_pairs(&packet_pairs);
        assert_eq!(13, sum);
    }
}
