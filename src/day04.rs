use std::ops::RangeInclusive;

use eyre::{ContextCompat, Result};

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day04.txt")?;

    let range_pairs = data.lines().map(parse_ranges).collect::<Result<Vec<_>>>()?;

    let count1 = range_pairs
        .iter()
        .filter(|(range1, range2)| {
            (range1.contains(range2.start()) && range1.contains(range2.end()))
                || (range2.contains(range1.start()) && range2.contains(range1.end()))
        })
        .count();
    println!("Part 1: {count1}");

    let count2 = range_pairs
        .iter()
        .filter(|(range1, range2)| {
            (range2.contains(range1.start())) || (range1.contains(range2.start()))
        })
        .count();
    println!("Part 2: {count2}");

    Ok(())
}

fn parse_ranges(s: &str) -> Result<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    let (r1, r2) = s.split_once(',').context("Bad input")?;
    let range1 = as_range(r1)?;
    let range2 = as_range(r2)?;

    Ok((range1, range2))
}

fn as_range(s: &str) -> Result<RangeInclusive<u64>> {
    let (begin, end) = s.split_once('-').context("Bad input")?;
    let low = begin.parse::<u64>()?;
    let high = end.parse::<u64>()?;

    Ok(RangeInclusive::new(low, high))
}
