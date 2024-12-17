#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GardenState {
    Unvisited(char),
    Visited(char),
    Border,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

pub fn get_farm(file: &str) -> Vec<Vec<GardenState>> {
    let mut map: Vec<Vec<GardenState>> = vec![];
    for line in file.lines() {
        map.push(line.chars().map(|x| GardenState::Unvisited(x)).collect());
    }
    map
}

pub fn walk_farm(map: &mut Vec<Vec<GardenState>>) {
    let mut le_total = 0;
    let mut budget_total = 0;
    let mut current_char: GardenState;
    for row_index in 0..map.len() {
        for col_index in 0..map[row_index].len() {
            match map[row_index][col_index] {
                GardenState::Unvisited(v) => {
                    let mut area: usize = 1;
                    let mut perimeter: usize = 0;
                    let mut corners: usize = 0;
                    current_char = map[row_index][col_index];
                    map[row_index][col_index] = GardenState::Visited(v);
                    get_contiguous_region(
                        current_char,
                        map,
                        row_index,
                        col_index,
                        &mut area,
                        &mut perimeter,
                        &mut corners,
                    );
                    // println!(
                    //     "The area for the contiguous region {:?} is {:?},the perimeter is {}, and the number of sides is {}",
                    //     current_char, area, perimeter, corners
                    // );
                    le_total += area * perimeter;
                    budget_total += area * corners;
                }
                _ => {}
            }
        }
    }
    println!("The total is {}", le_total);
    println!("The budget total is {}", budget_total);
}

fn get_contiguous_region(
    current_char: GardenState,
    map: &mut Vec<Vec<GardenState>>,
    row_index: usize,
    col_index: usize,
    area: &mut usize,
    perimeter: &mut usize,
    corners: &mut usize,
) {
    match current_char {
        GardenState::Unvisited(v) => {
            // Inside corner if two adjacent sides are the same and diagonal is not
            // Outside corner if two adjacent sides are not the same
            let mut north = GardenState::Border;
            let mut south = GardenState::Border;
            let mut east = GardenState::Border;
            let mut west = GardenState::Border;
            let mut northwest = GardenState::Border;
            let mut southwest = GardenState::Border;
            let mut northeast = GardenState::Border;
            let mut southeast = GardenState::Border;

            // Bound checks
            if row_index != 0 {
                north = map[row_index - 1][col_index];
            }
            if row_index != 0 && map[row_index].get(col_index + 1).is_some() {
                northeast = map[row_index - 1][col_index + 1];
            }
            if row_index != 0 && col_index != 0 {
                northwest = map[row_index - 1][col_index - 1];
            }
            if map.get(row_index + 1).is_some() {
                south = map[row_index + 1][col_index];
            }
            if map.get(row_index + 1).is_some()
                && map.get(row_index + 1).unwrap().get(col_index + 1).is_some()
            {
                southeast = map[row_index + 1][col_index + 1];
            }
            if col_index != 0 && map.get(row_index + 1).is_some() {
                southwest = map[row_index + 1][col_index - 1];
            }
            if col_index != 0 {
                west = map[row_index][col_index - 1];
            }
            if map[row_index].get(col_index + 1).is_some() {
                east = map[row_index][col_index + 1];
            }

            if (north == GardenState::Unvisited(v) || north == GardenState::Visited(v))
                && (west == GardenState::Unvisited(v) || west == GardenState::Visited(v))
                && northwest != GardenState::Unvisited(v)
                && northwest != GardenState::Visited(v)
            {
                *corners += 1;
            }

            if (north == GardenState::Unvisited(v) || north == GardenState::Visited(v))
                && (east == GardenState::Unvisited(v) || east == GardenState::Visited(v))
                && northeast != GardenState::Unvisited(v)
                && northeast != GardenState::Visited(v)
            {
                *corners += 1;
            }

            if (south == GardenState::Unvisited(v) || south == GardenState::Visited(v))
                && (west == GardenState::Unvisited(v) || west == GardenState::Visited(v))
                && southwest != GardenState::Unvisited(v)
                && southwest != GardenState::Visited(v)
            {
                *corners += 1;
            }

            if (south == GardenState::Unvisited(v) || south == GardenState::Visited(v))
                && (east == GardenState::Unvisited(v) || east == GardenState::Visited(v))
                && southeast != GardenState::Unvisited(v)
                && southeast != GardenState::Visited(v)
            {
                *corners += 1;
            }

            if south != GardenState::Visited(v)
                && south != GardenState::Unvisited(v)
                && east != GardenState::Visited(v)
                && east != GardenState::Unvisited(v)
            {
                *corners += 1;
            }

            if south != GardenState::Visited(v)
                && south != GardenState::Unvisited(v)
                && west != GardenState::Visited(v)
                && west != GardenState::Unvisited(v)
            {
                *corners += 1;
            }

            if north != GardenState::Visited(v)
                && north != GardenState::Unvisited(v)
                && east != GardenState::Visited(v)
                && east != GardenState::Unvisited(v)
            {
                *corners += 1;
            }

            if north != GardenState::Visited(v)
                && north != GardenState::Unvisited(v)
                && west != GardenState::Visited(v)
                && west != GardenState::Unvisited(v)
            {
                *corners += 1;
            }

            // Perimiter - Start with 4 and remove 1 for each adjacent plot of the same type
            let mut perimeter_addition = 4;

            if row_index != 0
                && (map[row_index - 1][col_index] == current_char
                    || map[row_index - 1][col_index] == GardenState::Visited(v))
            {
                perimeter_addition -= 1;
            }
            // Check down
            if row_index != map.len() - 1
                && (map[row_index + 1][col_index] == current_char
                    || map[row_index + 1][col_index] == GardenState::Visited(v))
            {
                perimeter_addition -= 1;
            }
            // Check left
            if col_index != 0
                && (map[row_index][col_index - 1] == current_char
                    || map[row_index][col_index - 1] == GardenState::Visited(v))
            {
                perimeter_addition -= 1;
            }
            // Check right
            if col_index != map[0].len() - 1
                && (map[row_index][col_index + 1] == current_char
                    || map[row_index][col_index + 1] == GardenState::Visited(v))
            {
                perimeter_addition -= 1;
            }
            *perimeter += perimeter_addition;

            // Area + Sides

            // Check up
            if row_index != 0 && map[row_index - 1][col_index] == current_char {
                *area += 1;
                map[row_index - 1][col_index] = GardenState::Visited(v);
                get_contiguous_region(
                    current_char,
                    map,
                    row_index - 1,
                    col_index,
                    area,
                    perimeter,
                    corners,
                );
            }
            // Check down
            if row_index != map.len() - 1 && map[row_index + 1][col_index] == current_char {
                *area += 1;
                map[row_index + 1][col_index] = GardenState::Visited(v);
                get_contiguous_region(
                    current_char,
                    map,
                    row_index + 1,
                    col_index,
                    area,
                    perimeter,
                    corners,
                );
            }
            // Check left
            if col_index != 0 && map[row_index][col_index - 1] == current_char {
                *area += 1;
                map[row_index][col_index - 1] = GardenState::Visited(v);
                get_contiguous_region(
                    current_char,
                    map,
                    row_index,
                    col_index - 1,
                    area,
                    perimeter,
                    corners,
                );
            }
            // Check right
            if col_index != map[0].len() - 1 && map[row_index][col_index + 1] == current_char {
                *area += 1;
                map[row_index][col_index + 1] = GardenState::Visited(v);
                get_contiguous_region(
                    current_char,
                    map,
                    row_index,
                    col_index + 1,
                    area,
                    perimeter,
                    corners,
                );
            }
        }
        _ => {}
    }
}
