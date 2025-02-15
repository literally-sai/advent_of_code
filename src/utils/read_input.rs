use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

pub fn read_input(year: u32, day: u32) -> io::Result<String> {
    let filename = format!("inputs/year{}/day{:02}", year, day);
    let path = Path::new(&filename);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}