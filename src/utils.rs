use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

pub fn print_solution(year: u32, day: u32, part: u8,solution: fn(&str) -> i32) {
    let input = read_input(year, day).unwrap();
    let solution = solution(&input);
    println!("\nThe output for {year} day {day} part {part} is -> {solution}\n");
}

fn read_input(year: u32, day: u32) -> io::Result<String> {
    let filename = format!("inputs/year{}/day{:02}", year, day);
    let path = Path::new(&filename);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
