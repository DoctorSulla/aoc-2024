use std::collections::HashMap;

pub fn generate_initial_rocks(file: &str) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let rocks_vec: Vec<usize> = file.trim().split(' ').map(|v| v.parse().unwrap()).collect();
    for rock in rocks_vec.into_iter() {
        if map.contains_key(&rock) {
            map.entry(rock).and_modify(|r| *r += 1);
        } else {
            map.insert(rock, 1);
        }
    }
    map
}

pub fn blink(mut rocks: HashMap<usize, usize>, iterations: usize) {
    for _ in 0..iterations {
        let mut new_map: HashMap<usize, usize> = HashMap::new();

        for (key, value) in rocks.iter() {
            if *key == 0 {
                if new_map.contains_key(&1) {
                    new_map.entry(1).and_modify(|r| *r += *value);
                } else {
                    new_map.insert(1, *value);
                }
            } else if key.to_string().len() % 2 == 0 {
                let rock_string = key.to_string();
                let (first_half, second_half) = &rock_string.split_at(rock_string.len() / 2);
                let first_half: usize = first_half.parse().unwrap();
                let second_half: usize = second_half.parse().unwrap();
                if new_map.contains_key(&first_half) {
                    new_map.entry(first_half).and_modify(|r| *r += *value);
                } else {
                    new_map.insert(first_half, *value);
                }

                if new_map.contains_key(&second_half) {
                    new_map.entry(second_half).and_modify(|r| *r += *value);
                } else {
                    new_map.insert(second_half, *value);
                }
            } else {
                let new_key = *key * 2024;
                if new_map.contains_key(&new_key) {
                    new_map.entry(new_key).and_modify(|r| *r += *value);
                } else {
                    new_map.insert(new_key, *value);
                }
            }
        }
        rocks = new_map;
    }
    let total: usize = rocks.values().sum();
    println!("The total is {}", total);
}
// for num in (0..remaining_iterations).rev() {
//     let mut rock_buffer: Vec<u64> = vec![];
//     if let Some(result) = cache.get(&rocks) {
//         rock_buffer = result.1.clone();
//         rocks = result.0.clone();
//     } else {
//         let initial_rocks = rocks.clone();
//         for rock in rocks.iter_mut() {
//             if *rock == 0 {
//                 *rock = 1;
//             } else if rock.to_string().len() % 2 == 0 {
//                 let rock_string = rock.to_string();
//                 let (first_half, second_half) = &rock_string.split_at(rock_string.len() / 2);
//                 *rock = first_half.parse().unwrap();
//                 rock_buffer.push(second_half.parse::<u64>().unwrap());
//             } else {
//                 *rock = *rock * 2024;
//             }
//         }
//         cache.insert(initial_rocks, (rocks.clone(), rock_buffer.clone()));
//     }

//     //blink(rocks.clone(), num, total);
//     *total += rock_buffer.len() as u64;
//     blink(rock_buffer, num, total, cache);
// }
