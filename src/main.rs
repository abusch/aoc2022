use color_eyre::Result;

mod day1;
mod day2;
mod day3;


fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;
    day3::run()?;

    Ok(())
}
