use std::path::Path;
pub mod day01;

pub fn solve_day(day: u32, input: &Path) -> Option<String> {
    match day {
        1 => Some(day01::solve(input)),
        _ => None,
    }
}
