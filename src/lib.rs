use std::fs;

pub mod day_eight;
pub mod day_eleven;
pub mod day_five;
pub mod day_four;
pub mod day_nine;
pub mod day_one;
pub mod day_seven;
pub mod day_six;
pub mod day_ten;
pub mod day_three;
pub mod day_twelve;
pub mod day_two;

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    // use super::*;
}
