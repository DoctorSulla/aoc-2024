use regex::Captures;
use regex::Regex;

pub fn get_valid_mul(file: String) {
    let mut total: u64 = 0;
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let captures: Vec<Captures> = pattern.captures_iter(&file).collect();
    for capture in captures {
        let (_full_match, capture_groups) = capture.extract::<2>();
        total +=
            capture_groups[0].parse::<u64>().unwrap() * capture_groups[1].parse::<u64>().unwrap();
    }
    println!("The total of the valid multiplications is {}", total);
}

pub fn parser(file: String) {
    let mut total: u64 = 0;
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut multiply = true;
    let mut buffer = String::new();

    for char in file.chars() {
        buffer.push(char);
        if buffer.as_str().contains("do()") {
            multiply = true;
            buffer = "".to_string();
        } else if buffer.as_str().contains("don't()") {
            multiply = false;
            buffer = "".to_string();
        } else if pattern.is_match(buffer.as_str()) {
            if multiply {
                let (_full_match, capture_groups) =
                    pattern.captures(buffer.as_str()).unwrap().extract::<2>();
                total += capture_groups[0].parse::<u64>().unwrap()
                    * capture_groups[1].parse::<u64>().unwrap();
            }
            buffer = "".to_string();
        }
    }
    println!("The total of the valid multiplications is {}", total);
}
