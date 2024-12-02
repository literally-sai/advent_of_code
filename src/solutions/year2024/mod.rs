use std::{error::Error, io};
use crate::utils::{input::Input, output::Output};
pub mod day01;
pub mod day02;

pub fn solve_day(day: u32, input: Input) -> Result<Output, Box<dyn Error>> {
    match day {
        1 => {
            return Ok(day01::solve(input)?);
        },
        2 => {
            return Ok(day02::solve(input)?);
        }
        _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))),
    }
}
