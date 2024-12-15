fn get_map(file: &str) -> Vec<Vec<Option<char>>> {
    let mut map: Vec<Vec<Option<char>>> = vec![];
    for line in file.lines() {
        map.push(line.chars().map(|x| Some(x)).collect());
    }
    map
}

fn walk_map(map: &mut Vec<Vec<Option<char>>>) {
    let mut current_char = map[0][0];

    for (row_index, row) in map.iter_mut().enumerate() {
        for (col_index, col) in row.iter_mut().enumerate() {}
    }
}
