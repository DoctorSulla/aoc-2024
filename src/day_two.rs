pub fn get_safe_lines(file: String) {
    let mut safe_lines = 0;
    let mut unsafe_lines = 0;

    let lines = file.split("\n");
    for line in lines {
        if line == "" {
        } else {
            let values: Vec<&str> = line.split(" ").collect();
            let values: Vec<i32> = values.into_iter().map(|v| v.parse().unwrap()).collect();
            if is_line_safe(values) {
                safe_lines += 1;
            } else {
                unsafe_lines += 1
            };
        }
    }
    println!(
        "The number of safe lines is {} and unsafe lines is {}",
        safe_lines, unsafe_lines
    );
}

enum Direction {
    Ascending,
    Descending,
}

fn valid_diff(val_one: i32, val_two: i32) -> bool {
    let diff = val_one.abs_diff(val_two);
    if diff < 1 || diff > 3 {
        return false;
    }
    true
}

fn is_line_safe(line: Vec<i32>) -> bool {
    let mut direction = Direction::Ascending;
    // Check all increasing or decreasing
    let mut i = 0;
    while i < line.len() {
        if i == 0 {
        } else if i == 1 {
            if line[i] > line[i - 1] {
                direction = Direction::Ascending;
                if !valid_diff(line[i], line[i - 1]) {
                    return false;
                }
            } else if line[i] < line[i - 1] {
                direction = Direction::Descending;
                if !valid_diff(line[i], line[i - 1]) {
                    return false;
                }
            } else {
                return false;
            };
        } else {
            match direction {
                Direction::Ascending => {
                    let diff = line[i] - line[i - 1];
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                }
                Direction::Descending => {
                    let diff = line[i - 1] - line[i];
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                }
            }
        }
        i += 1;
    }
    true
}
