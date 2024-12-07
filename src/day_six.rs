use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Directions {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

pub fn get_starting_position(map: Vec<Vec<char>>) -> Option<Coordinate> {
    for (y, line) in map.iter().enumerate() {
        if let Some(x) = line.iter().position(|v| *v == '^') {
            return Some(Coordinate { x, y });
        }
    }
    None
}

pub fn get_loop_count(map: Vec<Vec<char>>) {
    let starting_pos = get_starting_position(map.clone()).unwrap();
    let possible_locations = patrol(starting_pos.clone(), map.clone(), false).unwrap();
    let mut loop_count = 0;

    for location in possible_locations {
        let mut map_clone = map.clone();
        map_clone[location.y][location.x] = '#';

        if patrol(starting_pos.clone(), map_clone.clone(), false).is_none() {
            loop_count += 1;
        }
    }

    println!("Obstacles causing infinite loops are {}", loop_count);
}

pub fn patrol(
    mut position: Coordinate,
    map: Vec<Vec<char>>,
    print: bool,
) -> Option<HashSet<Coordinate>> {
    let mut visited_with_direction: HashSet<(Directions, Coordinate)> = HashSet::new();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut current_direction = Directions::Up;
    loop {
        match current_direction {
            Directions::Up => {
                // About to leave
                if position.y == 0 {
                    visited.insert(position.clone());
                    break;
                }
                if map[position.y - 1][position.x] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    visited.insert(position.clone());
                    if !visited_with_direction.insert((current_direction.clone(), position.clone()))
                    {
                        return None;
                    }
                    position.y -= 1;
                }
            }
            Directions::Down => {
                // About to leave
                if position.y == map.len() - 1 {
                    visited.insert(position.clone());
                    break;
                }
                if map[position.y + 1][position.x] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    visited.insert(position.clone());
                    if !visited_with_direction.insert((current_direction.clone(), position.clone()))
                    {
                        return None;
                    }
                    position.y += 1;
                }
            }
            Directions::Left => {
                // About to leave
                if position.x == 0 {
                    visited.insert(position.clone());
                    break;
                }
                if map[position.y][position.x - 1] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    visited.insert(position.clone());
                    if !visited_with_direction.insert((current_direction.clone(), position.clone()))
                    {
                        return None;
                    }
                    position.x -= 1;
                }
            }
            Directions::Right => {
                if position.x == map[0].len() - 1 {
                    visited.insert(position.clone());
                    break;
                }
                if map[position.y][position.x + 1] == '#' {
                    current_direction = rotate_90(current_direction);
                } else {
                    visited.insert(position.clone());
                    if !visited_with_direction.insert((current_direction.clone(), position.clone()))
                    {
                        return None;
                    }
                    position.x += 1;
                }
            }
        }
    }
    if print {
        println!("The number of squares covered is {}", visited.len());
    }
    Some(visited)
}
