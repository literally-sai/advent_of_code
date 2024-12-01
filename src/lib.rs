use std::{path::Path, error::Error, io::ErrorKind};
pub mod solutions;

pub fn solve(year: u32, day: u32, input: &Path) -> Result<String, Box<dyn Error>> {
    match year {
        2024 => solutions::year2024::solve_day(day, input),
        _ => Err(Box::new(std::io::Error::new(ErrorKind::InvalidInput, "Unsupported year",))),
    }
}
