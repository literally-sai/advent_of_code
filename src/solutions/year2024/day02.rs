use std::error::Error;

use crate::utils::input::{Input, get_input};
use crate::utils::output::Output;

pub fn solve(input: Input) -> Result<Output, Box<dyn Error>> {
    let part_1 = part_01(&input)?;
    let part_2 = part_2(&input)?;

    Ok(Output::new(&part_1, &part_2))
}

fn part_01(input: &Input) -> Result<String, Box<dyn Error>> {
    let input = get_input(&input)?;

    let safe_levels = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            is_safe_report(&numbers)
        }).count();


    Ok(safe_levels.to_string())
}

fn is_safe_report(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let first_diff = numbers[1] - numbers[0];

    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }

    let direction = first_diff.signum();

    for pair in numbers.windows(2).skip(1) {
        let diff = pair[1] - pair[0];

        if diff == 0 {
            return false;
        }

        if diff.signum() != direction {
            return false;
        }

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    return true;
}

fn part_2(input: &Input) -> Result<String, Box<dyn Error>> {
    let input = get_input(&input)?;

    let safe_levels = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            is_safe_report_with_dampener(&numbers)
        }).count();

    Ok(safe_levels.to_string())
}

fn is_safe_report_with_dampener(numbers: &[i32]) -> bool {
    if is_safe_report(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut reduced_numbers = numbers.to_vec();
        reduced_numbers.remove(i);
        if is_safe_report(&reduced_numbers) {
            return true;
        }
    }
    false
}
