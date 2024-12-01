use std::path::Path;
pub mod solutions;

pub fn solve(year: u32, day: u32, input: &Path) -> Option<String> {
    match year {
        2024 => solutions::year2024::solve_day(day, input),
        _ => None,
    }
}
