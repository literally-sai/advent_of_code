use std::path::Path;
use advent_of_code::solutions::year2024;

#[test]
fn day01() {
    let m = year2024::solve_day(1, Path::new("./inputs/2024/day01.txt")).unwrap();

    eprintln!("{m}");
}
