pub fn get_lines_vec(file: String) -> Vec<Vec<char>> {
    let mut lines_vec: Vec<Vec<char>> = vec![];

    let lines = file.split("\n");
    for line in lines {
        if line.is_empty() {
        } else {
            let char_array: Vec<char> = line.chars().collect();
            lines_vec.push(char_array);
        }
    }
    lines_vec
}

pub fn check_for_xmas_two(lines_vec: Vec<Vec<char>>) {
    let mut xmas_count = 0;
    for (i, line) in lines_vec.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'A' && check_mas(i, j, &lines_vec) {
                xmas_count += 1;
            }
        }
    }
    println!("Total appearances of x-mas is {}", xmas_count);
}

pub fn check_for_xmas(lines_vec: Vec<Vec<char>>) {
    let mut xmas_count = 0;
    for (i, line) in lines_vec.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'X' {
                // Check left
                if check_left(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check right

                if check_right(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check up

                if check_up(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check down
                if check_down(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check left up diagonal

                if check_left_up_diagonal(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check left down diagonal

                if check_left_down_diagonal(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check right up diagonal
                if check_right_up_diagonal(i, j, &lines_vec) {
                    xmas_count += 1;
                }
                // Check right down diagonal
                if check_right_down_diagonal(i, j, &lines_vec) {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("Total appearances of xmas is {}", xmas_count);
}

fn check_left(row: usize, mut col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        col = col.saturating_sub(1);
    }
    buffer.as_str() == "XMAS"
}

fn check_right(row: usize, mut col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        col += 1;
    }
    buffer.as_str() == "XMAS"
}

fn check_down(mut row: usize, col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        row += 1;
    }
    buffer.as_str() == "XMAS"
}

fn check_up(mut row: usize, col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        row = row.saturating_sub(1);
    }
    buffer.as_str() == "XMAS"
}

fn check_left_up_diagonal(mut row: usize, mut col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        if row == 0 || col == 0 {
            break;
        }
        row -= 1;
        col -= 1;
    }
    buffer.as_str() == "XMAS"
}

fn check_right_up_diagonal(mut row: usize, mut col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        if row == 0 {
            break;
        }
        row -= 1;
        col += 1;
    }
    buffer.as_str() == "XMAS"
}

fn check_left_down_diagonal(mut row: usize, mut col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        if col == 0 {
            break;
        }
        row += 1;
        col -= 1;
    }
    buffer.as_str() == "XMAS"
}

fn check_right_down_diagonal(mut row: usize, mut col: usize, lines_vec: &[Vec<char>]) -> bool {
    let mut buffer = String::new();
    let rows = lines_vec.len();
    let cols = lines_vec[0].len();
    while buffer.len() < 4 && col < cols && row < rows {
        buffer.push(lines_vec[row][col]);
        row += 1;
        col += 1;
    }
    buffer.as_str() == "XMAS"
}

fn check_mas(row: usize, col: usize, lines_vec: &[Vec<char>]) -> bool {
    if row == 0 || col == 0 || row == 139 || col == 139 {
        return false;
    }

    if lines_vec[row - 1][col - 1] == 'M'
        && lines_vec[row - 1][col + 1] == 'M'
        && lines_vec[row + 1][col - 1] == 'S'
        && lines_vec[row + 1][col + 1] == 'S'
    {
        return true;
    }

    if lines_vec[row - 1][col - 1] == 'M'
        && lines_vec[row - 1][col + 1] == 'S'
        && lines_vec[row + 1][col - 1] == 'M'
        && lines_vec[row + 1][col + 1] == 'S'
    {
        return true;
    }

    if lines_vec[row - 1][col - 1] == 'S'
        && lines_vec[row - 1][col + 1] == 'M'
        && lines_vec[row + 1][col - 1] == 'S'
        && lines_vec[row + 1][col + 1] == 'M'
    {
        return true;
    }

    if lines_vec[row - 1][col - 1] == 'S'
        && lines_vec[row - 1][col + 1] == 'S'
        && lines_vec[row + 1][col - 1] == 'M'
        && lines_vec[row + 1][col + 1] == 'M'
    {
        return true;
    }
    false
}
