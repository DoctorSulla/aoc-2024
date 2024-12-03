use std::fs;

pub mod day_one;
pub mod day_three;
pub mod day_two;

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    // use super::*;
}
