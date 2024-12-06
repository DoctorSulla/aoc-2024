#[derive(Debug, Clone, PartialEq)]
enum Directions {
    Left,
    Right,
    Up,
    Down,
}

enum CheckResult {
    Duplicate,
    Loop,
    NoMatch,
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

fn check(
    needle: (&Directions, &Coordinate),
    haystack: &Vec<(Directions, Coordinate)>,
) -> CheckResult {
    for coordinate in haystack {
        if needle.1.x == coordinate.1.x && needle.1.y == coordinate.1.y && *needle.0 == coordinate.0
        {
            return CheckResult::Loop;
        } else if needle.1.x == coordinate.1.x && needle.1.y == coordinate.1.y {
            return CheckResult::Duplicate;
        }
    }
    return CheckResult::NoMatch;
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
    let mut loop_count = 0;
    let mut i = 0;

    while i < map.len() {
        let mut j = 0;
        while j < map[i].len() {
            if map[i][j] == '.' {
                let mut map_clone = map.clone();
                map_clone[i][j] = '#';
                if patrol(
                    get_starting_position(map_clone.clone()).unwrap(),
                    map_clone.clone(),
                    false,
                )
                .is_some()
                {
                    loop_count += 1;
                }
            }
            j += 1;
        }
        i += 1;
        println!("We are now on row {}", i);
    }
    println!("Obstacles causing infinite loops are {}", loop_count);
}

pub fn patrol(mut position: Coordinate, map: Vec<Vec<char>>, print: bool) -> Option<()> {
    let mut visited: Vec<(Directions, Coordinate)> = vec![];
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
                    match check((&current_direction, &position), &visited) {
                        CheckResult::NoMatch => {
                            covered_squares += 1;
                            visited.push((current_direction.clone(), position.clone()));
                        }
                        CheckResult::Duplicate => {}
                        CheckResult::Loop => return Some(()),
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
                    match check((&current_direction, &position), &visited) {
                        CheckResult::NoMatch => {
                            covered_squares += 1;
                            visited.push((current_direction.clone(), position.clone()));
                        }
                        CheckResult::Duplicate => {}
                        CheckResult::Loop => return Some(()),
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
                    match check((&current_direction, &position), &visited) {
                        CheckResult::NoMatch => {
                            covered_squares += 1;
                            visited.push((current_direction.clone(), position.clone()));
                        }
                        CheckResult::Duplicate => {}
                        CheckResult::Loop => return Some(()),
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
                    match check((&current_direction, &position), &visited) {
                        CheckResult::NoMatch => {
                            covered_squares += 1;
                            visited.push((current_direction.clone(), position.clone()));
                        }
                        CheckResult::Duplicate => {}
                        CheckResult::Loop => return Some(()),
                    }
                    position.x += 1;
                }
            }
        }
    }
    if print {
        println!("The number of squares covered is {}", covered_squares);
    }
    None
}
