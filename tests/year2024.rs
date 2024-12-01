use advent_of_code::solutions::year2024;
use advent_of_code::utils::input::Input;
#[test]
fn day01() {
    let solution = year2024::solve_day(
        1,
        (Input::Path("inputs/2024/day01-1.txt".into()), Input::Path("inputs/2024/day01-2.txt".into()))).unwrap();
    eprintln!("Part1: {}\nPart2: {}", solution.0, solution.1);
}
