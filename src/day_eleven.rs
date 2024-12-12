use std::collections::HashMap;

pub fn generate_initial_rocks(file: &str) -> Vec<u64> {
    file.trim().split(' ').map(|v| v.parse().unwrap()).collect()
}

fn blink(
    rocks: &mut Vec<u64>,
    input_output: &mut HashMap<u64, (u64, Option<u64>)>,
    remaining_blinks: usize,
    total: &mut u64,
) {
    let mut rock_buffer: Vec<u64> = vec![];
    for rock in rocks.iter_mut() {
        if let Some(val) = input_output.get(rock) {
            *rock = val.0;
            if let Some(new) = val.1 {
                rock_buffer.push(new);
            }
        } else {
            // 0 Check
            if *rock == 0 {
                input_output.insert(*rock, (1, None));
                *rock = 1;
            } else if rock.to_string().len() % 2 == 0 {
                let rock_string = rock.to_string();
                let (first_half, second_half) = &rock_string.split_at(rock_string.len() / 2);
                input_output.insert(
                    *rock,
                    (
                        first_half.parse().unwrap(),
                        Some(second_half.parse().unwrap()),
                    ),
                );
                *rock = first_half.parse().unwrap();
                rock_buffer.push(second_half.parse::<u64>().unwrap());
            } else {
                input_output.insert(*rock, (*rock * 2024, None));
                *rock = *rock * 2024;
            }
        }
    }
    if !rock_buffer.is_empty() {
        blink_x_times(remaining_blinks - 1, &mut rock_buffer, total, input_output);
    }
}

pub fn blink_x_times(
    x: usize,
    rocks: &mut Vec<u64>,
    total: &mut u64,
    input_output: &mut HashMap<u64, (u64, Option<u64>)>,
) {
    for num in 0..x {
        // println!(
        //     "We are now on blink {} and the length is {}",
        //     num,
        //     rocks.len()
        // );
        blink(rocks, input_output, x - num, total);
    }
    *total += rocks.len() as u64;
    println!("This iteration added {}", rocks.len());
    println!("Total is now {}", total);
}
