use std::{collections::HashMap, error::Error};

use crate::utils::input::{Input, get_input};
pub fn solve(input: (Input, Input)) -> Result<(String, String), Box<dyn Error>> {
    let output_1 = part_01(input.0)?;
    let output_2 = part_02(input.1)?;

    Ok((output_1, output_2))
}


fn part_01(input:  Input) -> Result<String, Box<dyn Error>> {
    let input = get_input(input)?; 

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (numbers.next(), numbers.next()) {
            left.push(left_num.parse()?);
            right.push(right_num.parse()?);
        }
    }

    if left.len() != right.len() {
        return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Mismatched input lengths"
        )));
    }

    left.sort();
    right.sort();

    let distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum();

    Ok(distance.to_string())
}

fn part_02(input: Input) -> Result<String, Box<dyn Error>> {
    let input = get_input(input)?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (numbers.next(), numbers.next()) {
            left.push(left_num.parse()?);
            right.push(right_num.parse()?);
        }
    }

    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in right {
        *map.entry(num).or_insert(0) += 1; 
    }

    let sum: i32 = left.iter()
        .filter_map(|l| map.get(l).map(|r| l * r))
        .sum();

    Ok(sum.to_string())
}
