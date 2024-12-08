use std::collections::HashSet;

pub fn get_grid(file: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in file.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn get_distance(point_one: Coordinate, point_two: Coordinate) -> (i16, i16) {
    (
        (point_one.x - point_two.x).try_into().unwrap(),
        (point_one.y - point_two.y).try_into().unwrap(),
    )
}

fn is_double(diff_one: (i16, i16), diff_two: (i16, i16)) -> bool {
    // X is the same and Y is double
    if diff_one.0 == diff_two.0 && diff_one.1.abs() == diff_two.1.abs() * 2 {
        return true;
    }
    // Y is the same and X is double
    else if diff_one.1 == diff_two.1 && diff_one.0.abs() == diff_two.0.abs() * 2 {
        return true;
    }
    // Y and X are double
    else if diff_one.1.abs() == diff_two.1.abs() * 2 && diff_one.0.abs() == diff_two.0.abs() * 2 {
        return true;
    }
    false
}

pub fn get_antinodes(grid: Vec<Vec<char>>) {
    let mut antinodes: std::collections::HashSet<Coordinate> = HashSet::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            if *char != '.' {
                let current_char = *char;
                // let current_coordinate = Coordinate::new(col_index, row_index);
                let mut in_scope: Vec<Coordinate> = vec![];
                // Add row
                for num in 0..row.len() {
                    if grid[row_index][num] == current_char && num != col_index {
                        in_scope.push(Coordinate::new(num, row_index))
                    }
                }
                // Add col
                for num in 0..grid.len() {
                    if grid[num][col_index] == current_char && num != row_index {
                        in_scope.push(Coordinate::new(num, row_index))
                    }
                }
                // Go left and up unti we hit edge
                let mut x = col_index - 1;
                let mut y = row_index - 1;
                while grid.get(y).is_some() && grid.get(y).unwrap().get(x).is_some() {
                    if grid[y][x] == current_char {
                        in_scope.push(Coordinate::new(x, y));
                    }
                    x -= 1;
                    y -= 1;
                }
                // Go right and up until we hit edge
                let mut x = col_index + 1;
                let mut y = row_index - 1;
                while grid.get(y).is_some() && grid.get(y).unwrap().get(x).is_some() {
                    if grid[y][x] == current_char {
                        in_scope.push(Coordinate::new(x, y));
                    }
                    x += 1;
                    y -= 1;
                }
                // Go left and down until we hit edge
                let mut x = col_index - 1;
                let mut y = row_index + 1;
                while grid.get(y).is_some() && grid.get(y).unwrap().get(x).is_some() {
                    if grid[y][x] == current_char {
                        in_scope.push(Coordinate::new(x, y));
                    }
                    x -= 1;
                    y += 1;
                }

                // Go right and down until we hit edge
                let mut x = col_index + 1;
                let mut y = row_index + 1;
                while grid.get(y).is_some() && grid.get(y).unwrap().get(x).is_some() {
                    if grid[y][x] == current_char {
                        in_scope.push(Coordinate::new(x, y));
                    }
                    x += 1;
                    y += 1;
                }
                println!("{:?}", in_scope);
            }
        }
    }
    println!("The number of antinodes is {}", antinodes.len());
}
