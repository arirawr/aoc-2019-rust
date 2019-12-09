use std::fs;
use std::io::{Error};

pub fn read_input(id: u8) -> Result<String, Error> {
    fs::read_to_string(format!("inputs/{}.txt", id))
}
