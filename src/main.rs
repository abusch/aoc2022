use color_eyre::Result;

mod day1;
mod day2;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;

    Ok(())
}
