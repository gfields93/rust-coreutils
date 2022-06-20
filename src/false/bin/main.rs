use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct False;

impl Display for False {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for False { }

fn main() -> Result<(), Box<dyn Error>> {
    Err(Box::new(False))
}