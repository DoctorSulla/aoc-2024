use std::collections::HashMap;

pub fn generate_initial_rocks(file: &str) -> Vec<u64> {
    file.trim().split(' ').map(|v| v.parse().unwrap()).collect()
}

pub fn blink(mut rocks: Vec<u64>, remaining_iterations: usize, total: &mut u64) {
    let mut rock_buffer: Vec<u64> = vec![];
    for num in 0..remaining_iterations {
        println!("{}", num);
        for rock in rocks.iter_mut() {
            if *rock == 0 {
                *rock = 1;
            } else if rock.to_string().len() % 2 == 0 {
                let rock_string = rock.to_string();
                let (first_half, second_half) = &rock_string.split_at(rock_string.len() / 2);
                *rock = first_half.parse().unwrap();
                rock_buffer.push(second_half.parse::<u64>().unwrap());
            } else {
                *rock = *rock * 2024;
            }
        }

        blink(rocks.clone(), num, total);
        blink(rock_buffer.clone(), num, total);
    }
    *total += rock_buffer.len() as u64;
    println!("{}", total);
}

// pub fn how_many_does_it_generate(mut rock: u64, iterations: usize) -> u64 {
//     let mut total = 0;
//     for _num in 0..iterations {
//         // 0 Check
//         if rock == 0 {
//             rock = 1;
//         } else if rock.to_string().len() % 2 == 0 {
//             let rock_string = rock.to_string();
//             let (first_half, _second_half) = &rock_string.split_at(rock_string.len() / 2);
//             rock = first_half.parse().unwrap();
//             total += 1;
//         } else {
//             rock = rock * 2024;
//         }
//     }
//     total
// }
