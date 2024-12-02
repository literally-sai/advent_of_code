use std::{fs::read_to_string, path::PathBuf, error::Error};

pub enum Input {
    Path(PathBuf),
    Text(String),
}

pub fn get_input(input: &Input) -> Result<String, Box<dyn Error>> {
    match input {
        Input::Path(ref path) => Ok(read_to_string(path)?),
        Input::Text(s) => Ok(s.to_string()),
    }
}
