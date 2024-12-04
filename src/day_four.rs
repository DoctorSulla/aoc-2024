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
    println!("{:?}", lines_vec);
    lines_vec
}
