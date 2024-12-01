use std::{error::Error, io};
pub mod day01;

use crate::utils::Input;

pub fn solve_day(day: u32, input: Input) -> Result<String, Box<dyn Error>> {
    match day {
        1 => {Ok(day01::solve(input)?},
        _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))),
    }
}
