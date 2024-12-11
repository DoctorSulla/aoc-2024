pub fn generate_map(file: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = vec![];
    for line in file.lines() {
        let row: Vec<u32> = line.chars().map(|v| v.to_digit(10).unwrap()).collect();
    }
    map
}

// Walk the whole mountain range
pub fn walk_map(map: &[Vec<u32>]) {
    let mut trailheads = 0;
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == 0 {
                // Start
                look_for_next(col_index, row_index, 0, map, &mut trailheads);
            }
        }
    }
    println!("The total number of trailheads is {}", trailheads);
}

fn look_for_next(
    col_index: usize,
    row_index: usize,
    current_height: u32,
    map: &[Vec<u32>],
    trailheads: &mut u32,
) {
    if current_height == 9 {
        *trailheads += 1;
    }
    // Left
    if map.get(row_index).is_some() && map[row_index].get(col_index.saturating_sub(1)).is_some() {
        if map[row_index][col_index] == current_height + 1 {
            look_for_next(
                col_index - 1,
                row_index,
                current_height + 1,
                map,
                trailheads,
            );
        }
    }
    // Right
    if map.get(row_index).is_some() && map[row_index].get(col_index + 1).is_some() {
        if map[row_index][col_index] == current_height + 1 {
            look_for_next(
                col_index + 1,
                row_index,
                current_height + 1,
                map,
                trailheads,
            );
        }
    }
    // Up
    if map.get(row_index.saturating_sub(1)).is_some() && map[row_index].get(col_index).is_some() {
        if map[row_index][col_index] == current_height + 1 {
            look_for_next(
                col_index,
                row_index - 1,
                current_height + 1,
                map,
                trailheads,
            );
        }
    }

    // Down
    if map.get(row_index + 1).is_some() && map[row_index].get(col_index).is_some() {
        if map[row_index][col_index] == current_height + 1 {
            look_for_next(
                col_index,
                row_index + 1,
                current_height + 1,
                map,
                trailheads,
            );
        }
    }
}
