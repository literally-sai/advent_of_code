use std::{error::Error, fs::read_to_string, path::Path};

use crate::utils::Input;

pub fn solve(input: &Input) -> Result<String, Box<dyn Error>> {
    
    let content: String;

    match input {
        Input::Path(ref path) => {
            content = read_to_string(path)?;
        }
        Input::Text(s) => {
            content = s.clone();
        }
    }

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in content.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (numbers.next(), numbers.next()) {
            left.push(left_num.parse()?);
            right.push(right_num.parse()?);
        }
    }

    left.sort();
    right.sort();

    let mut distance: i32 = 0;

    for i in 0..right.len() {
       distance += i32::abs(right[i] - left[i]);

    }

    Ok(distance.to_string()) 
}
