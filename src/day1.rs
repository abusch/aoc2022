use color_eyre::Result;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day1_1.txt")?;

    let mut calories = Vec::new();
    let mut current_calories = 0;

    for line in data.lines() {
        if line.trim().is_empty() {
            calories.push(current_calories);
            current_calories = 0;
            continue;
        }
        let calories = line.trim().parse::<u64>()?;
        current_calories += calories;
    }
    calories.push(current_calories);

    let max = calories.iter().max().expect("empty vector?");
    println!("Elf with most calories carries {max} calories");

    calories.sort();
    let top_3_sum: u64 = calories.into_iter().rev().take(3).sum();
    println!("Top 3 elves carrying the most calories carry {top_3_sum} calories in total");

    Ok(())
}
