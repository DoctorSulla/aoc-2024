use std::collections::HashMap;
use std::collections::HashSet;

pub fn get_grid(file: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in file.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    let x1: i16 = point_one.x.try_into().unwrap();
    let y1: i16 = point_one.y.try_into().unwrap();
    let x2: i16 = point_two.x.try_into().unwrap();
    let y2: i16 = point_two.y.try_into().unwrap();

    (x1 - x2, y1 - y2)
}

fn is_collinear(point_one: Coordinate, point_two: Coordinate, point_three: Coordinate) -> bool {
    let x1: i16 = point_one.x.try_into().unwrap();
    let y1: i16 = point_one.y.try_into().unwrap();
    let x2: i16 = point_two.x.try_into().unwrap();
    let y2: i16 = point_two.y.try_into().unwrap();
    let x3: i16 = point_three.x.try_into().unwrap();
    let y3: i16 = point_three.y.try_into().unwrap();
    let left = (y1 - y2) * (x1 - x3);
    let right = (y1 - y3) * (x1 - x2);
    if left == 0 && right == 0 {
        return false;
    }

    left == right
}

fn is_double(diff_one: (i16, i16), diff_two: (i16, i16)) -> bool {
    if diff_one == diff_two {
        return false;
    }
    diff_one.1 == diff_two.1 * 2 && diff_one.0 == diff_two.0 * 2
}

pub fn collect_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<Coordinate>> {
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
    antenna_map
}

fn generate_permutations(
    input: &Vec<Coordinate>,
    permutations: &mut Vec<Vec<Coordinate>>,
    length: usize,
    current: Vec<Coordinate>,
) {
    if current.len() == length - 1 {
        for coord in input {
            let mut copy = current.clone();
            copy.push(coord.clone());
            permutations.push(copy)
        }
        return ();
    }

    for coord in input {
        let mut copy = current.clone();
        copy.push(coord.clone());
        generate_permutations(input, permutations, length, copy);
    }
}

pub fn get_antinodes(grid: Vec<Vec<char>>) {
    let mut antinodes: std::collections::HashSet<Coordinate> = HashSet::new();
    let mut antinodes_p2: std::collections::HashSet<Coordinate> = HashSet::new();

    let antennas = collect_antennas(&grid);
    for key in antennas.keys() {
        let current: Vec<Coordinate> = vec![];
        let mut permutations: Vec<Vec<Coordinate>> = vec![];
        let input = antennas.get(key).unwrap();
        generate_permutations(input, &mut permutations, 2, current);

        for permutation in permutations {
            for (row_index, row) in grid.iter().enumerate() {
                for (col_index, _char) in row.iter().enumerate() {
                    let diff_one = get_distance(
                        Coordinate::new(col_index, row_index),
                        permutation[0].clone(),
                    );
                    let diff_two = get_distance(
                        Coordinate::new(col_index, row_index),
                        permutation[1].clone(),
                    );
                    if is_double(diff_one, diff_two) {
                        antinodes.insert(Coordinate::new(col_index, row_index));
                    }

                    if is_collinear(
                        permutation[0].clone(),
                        Coordinate::new(col_index, row_index),
                        permutation[1].clone(),
                    ) {
                        antinodes_p2.insert(Coordinate::new(col_index, row_index));
                    }
                }
            }
        }
    }
    println!("The number of antinodes is {}", antinodes.len());
    println!("The number of antinodes is {}", antinodes_p2.len());
}
