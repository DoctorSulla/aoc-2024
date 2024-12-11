use std::collections::HashSet;

pub fn generate_map(file: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = vec![];
    for line in file.lines() {
        let row: Vec<u32> = line.chars().map(|v| v.to_digit(10).unwrap()).collect();
        map.push(row);
    }
    map
}

// Walk the whole mountain range
pub fn walk_map(map: &[Vec<u32>]) {
    let mut total_nine_coords: u32 = 0;

    let mut distinct_paths: u32 = 0;
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == 0 {
                // Start
                let mut nine_coords: HashSet<(usize, usize)> = HashSet::new();
                look_for_next(
                    col_index,
                    row_index,
                    0,
                    map,
                    &mut nine_coords,
                    &mut distinct_paths,
                );

                total_nine_coords += nine_coords.len() as u32;
            }
        }
    }
    println!("The total number of reachable 9s is {}", total_nine_coords);
    println!("The total number of distinct paths is {}", distinct_paths);
}

fn look_for_next(
    col_index: usize,
    row_index: usize,
    current_height: u32,
    map: &[Vec<u32>],
    nine_coords: &mut HashSet<(usize, usize)>,
    distinct_paths: &mut u32,
) {
    let height = map.len();
    let width = map[0].len();

    if current_height == 9 {
        nine_coords.insert((col_index, row_index));
        *distinct_paths += 1;
        return ();
    }
    // Left
    if col_index != 0 {
        if map[row_index][col_index - 1] == current_height + 1 {
            look_for_next(
                col_index - 1,
                row_index,
                current_height + 1,
                map,
                nine_coords,
                distinct_paths,
            );
        }
    }
    // Right
    if col_index < width - 1 {
        if map[row_index][col_index + 1] == current_height + 1 {
            look_for_next(
                col_index + 1,
                row_index,
                current_height + 1,
                map,
                nine_coords,
                distinct_paths,
            );
        }
    }
    // Up
    if row_index != 0 {
        if map[row_index - 1][col_index] == current_height + 1 {
            look_for_next(
                col_index,
                row_index - 1,
                current_height + 1,
                map,
                nine_coords,
                distinct_paths,
            );
        }
    }

    // Down
    if row_index < height - 1 {
        if map[row_index + 1][col_index] == current_height + 1 {
            look_for_next(
                col_index,
                row_index + 1,
                current_height + 1,
                map,
                nine_coords,
                distinct_paths,
            );
        }
    }
}
