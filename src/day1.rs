use std::num::ParseIntError;

use eyre::Result;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day1.txt")?;

    let (mut calories, _) =
        data.lines()
            .into_iter()
            .try_fold((Vec::new(), 0), |(mut vec, acc), line| {
                if line.trim().is_empty() {
                    vec.push(acc);
                    Ok::<_, ParseIntError>((vec, 0))
                } else {
                    let cal = line.trim().parse::<u64>()?;
                    Ok((vec, acc + cal))
                }
            })?;

    let max = calories.iter().max().expect("empty vector?");
    println!("Elf with most calories carries {max} calories");

    calories.sort();
    let top_3_sum: u64 = calories.into_iter().rev().take(3).sum();
    println!("Top 3 elves carrying the most calories carry {top_3_sum} calories in total");

    Ok(())
}
