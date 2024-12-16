#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GardenStatus {
    Unvisited(char),
    Visited(char),
}

pub fn get_farm(file: &str) -> Vec<Vec<GardenStatus>> {
    let mut map: Vec<Vec<GardenStatus>> = vec![];
    for line in file.lines() {
        map.push(line.chars().map(|x| GardenStatus::Unvisited(x)).collect());
    }
    map
}

pub fn walk_farm(map: &mut Vec<Vec<GardenStatus>>) {
    let mut le_total = 0;
    let mut current_char: GardenStatus;
    for row_index in 0..map.len() {
        for col_index in 0..map[row_index].len() {
            match map[row_index][col_index] {
                GardenStatus::Unvisited(v) => {
                    let mut area: usize = 1;
                    let mut perimeter: usize = 0;
                    current_char = map[row_index][col_index];
                    map[row_index][col_index] = GardenStatus::Visited(v);
                    get_contiguous_region(
                        current_char,
                        map,
                        row_index,
                        col_index,
                        &mut area,
                        &mut perimeter,
                    );
                    println!(
                        // "The area for the contiguous region {:?} is {:?} and the perimeter is {}",
                        current_char,
                        area, perimeter
                    );
                    le_total += area * perimeter;
                }
                GardenStatus::Visited(_) => {}
            }
        }
    }
    println!("The total is {}", le_total);
}

fn get_contiguous_region(
    current_char: GardenStatus,
    map: &mut Vec<Vec<GardenStatus>>,
    row_index: usize,
    col_index: usize,
    area: &mut usize,
    perimeter: &mut usize,
) {
    match current_char {
        GardenStatus::Unvisited(v) => {
            let mut perimeter_addition = 4;

            if row_index != 0
                && (map[row_index - 1][col_index] == current_char
                    || map[row_index - 1][col_index] == GardenStatus::Visited(v))
            {
                perimeter_addition -= 1;
            }
            // Check down
            if row_index != map.len() - 1
                && (map[row_index + 1][col_index] == current_char
                    || map[row_index + 1][col_index] == GardenStatus::Visited(v))
            {
                perimeter_addition -= 1;
            }
            // Check left
            if col_index != 0
                && (map[row_index][col_index - 1] == current_char
                    || map[row_index][col_index - 1] == GardenStatus::Visited(v))
            {
                perimeter_addition -= 1;
            }
            // Check right
            if col_index != map[0].len() - 1
                && (map[row_index][col_index + 1] == current_char
                    || map[row_index][col_index + 1] == GardenStatus::Visited(v))
            {
                perimeter_addition -= 1;
            }
            *perimeter += perimeter_addition;

            // Check up
            if row_index != 0 && map[row_index - 1][col_index] == current_char {
                *area += 1;
                map[row_index - 1][col_index] = GardenStatus::Visited(v);
                get_contiguous_region(current_char, map, row_index - 1, col_index, area, perimeter);
            }
            // Check down
            if row_index != map.len() - 1 && map[row_index + 1][col_index] == current_char {
                *area += 1;
                map[row_index + 1][col_index] = GardenStatus::Visited(v);
                get_contiguous_region(current_char, map, row_index + 1, col_index, area, perimeter);
            }
            // Check left
            if col_index != 0 && map[row_index][col_index - 1] == current_char {
                *area += 1;
                map[row_index][col_index - 1] = GardenStatus::Visited(v);
                get_contiguous_region(current_char, map, row_index, col_index - 1, area, perimeter);
            }
            // Check right
            if col_index != map[0].len() - 1 && map[row_index][col_index + 1] == current_char {
                *area += 1;
                map[row_index][col_index + 1] = GardenStatus::Visited(v);
                get_contiguous_region(current_char, map, row_index, col_index + 1, area, perimeter);
            }
        }
        GardenStatus::Visited(_) => {}
    }
}
