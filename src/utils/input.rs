use std::path::PathBuf;

pub enum Input {
    Path(PathBuf),
    Text(String),
}
