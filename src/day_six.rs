enum Directions {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, Clone)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

fn rotate_90(direction: Directions) -> Directions {
    match direction {
        Directions::Left => Directions::Up,
        Directions::Up => Directions::Right,
        Directions::Right => Directions::Down,
        Directions::Down => Directions::Left,
    }
}

pub fn get_map(file: String) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];
    let lines = file.split("\n");
    for line in lines {
        if line.is_empty() {
        } else {
            map.push(line.chars().collect());
        }
    }
    map
}

fn check(needle: &Coordinate, haystack: &Vec<Coordinate>) -> bool {
    for coordinate in haystack {
        if needle.x == coordinate.x && needle.y == coordinate.y {
            return true;
        }
    }
    false
}

pub fn get_starting_position(map: Vec<Vec<char>>) -> Option<Coordinate> {
    for (y, line) in map.iter().enumerate() {
        if let Some(x) = line.iter().position(|v| *v == '^') {
            return Some(Coordinate { x, y });
        }
    }
    None
}

pub fn patrol(mut position: Coordinate, map: Vec<Vec<char>>) {
    let mut visited: Vec<Coordinate> = vec![];
    let mut covered_squares = 0;
    let mut current_direction = Directions::Up;
    loop {
        match current_direction {
            Directions::Up => {
                // About to leave
                if position.y == 0 {
                    covered_squares += 1;
                    break;
                }
                if map[position.y - 1][position.x] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    if !check(&position, &visited) {
                        covered_squares += 1;
                        visited.push(position.clone());
                    }
                    position.y -= 1;
                }
            }
            Directions::Down => {
                // About to leave
                if position.y == map.len() - 1 {
                    covered_squares += 1;
                    break;
                }
                if map[position.y + 1][position.x] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    if !check(&position, &visited) {
                        covered_squares += 1;
                        visited.push(position.clone());
                    }
                    position.y += 1;
                }
            }
            Directions::Left => {
                // About to leave
                if position.x == 0 {
                    covered_squares += 1;
                    break;
                }
                if map[position.y][position.x - 1] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    if !check(&position, &visited) {
                        covered_squares += 1;
                        visited.push(position.clone());
                    }
                    position.x -= 1;
                }
            }
            Directions::Right => {
                if position.x == map[0].len() - 1 {
                    covered_squares += 1;
                    break;
                }
                if map[position.y][position.x + 1] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    if !check(&position, &visited) {
                        covered_squares += 1;
                        visited.push(position.clone());
                    }
                    position.x += 1;
                }
            }
        }
    }
    println!("The number of squares covered is {}", covered_squares);
}
