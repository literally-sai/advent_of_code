use std::{error::Error, io};
use crate::utils::input::*;
pub mod day01;


pub fn solve_day(day: u32, input: (Input, Input)) -> Result<(String, String), Box<dyn Error>> {
    match day {
        1 => {
            return Ok(day01::solve(input)?);
        },
        _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))),
    }
}
