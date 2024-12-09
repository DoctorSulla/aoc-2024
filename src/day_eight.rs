use std::collections::HashMap;
use std::collections::HashSet;

pub fn get_grid(file: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in file.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
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

pub fn collect_antennas(grid: Vec<Vec<char>>) -> HashMap<char, Vec<Coordinate>> {
    let mut antenna_map: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            if *char != '.' {
                if !antenna_map.contains_key(char) {
                    antenna_map.insert(*char, vec![Coordinate::new(col_index, row_index)]);
                } else {
                    antenna_map
                        .get_mut(char)
                        .unwrap()
                        .push(Coordinate::new(col_index, row_index));
                }
            }
        }
    }
    println!("{:?}", antenna_map);
    antenna_map
}

pub fn get_antinodes(grid: Vec<Vec<char>>) {
    let mut antinodes: std::collections::HashSet<Coordinate> = HashSet::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            if *char != '.' {}
        }
    }
    println!("The number of antinodes is {}", antinodes.len());
}
