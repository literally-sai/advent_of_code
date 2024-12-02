use std::{error::Error, io::ErrorKind};
pub mod solutions;
pub mod utils;

use crate::utils::input::Input;
use crate::utils::output::Output;
pub fn solve(year: u32, day: u32, input: Input) -> Result<Output, Box<dyn Error>> {
    match year {
        2024 => solutions::year2024::solve_day(day, input),
        _ => Err(Box::new(std::io::Error::new(ErrorKind::InvalidInput, "Unsupported year",))),
    }
}
