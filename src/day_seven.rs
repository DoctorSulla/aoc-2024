use radix_fmt::*;

pub fn process_file(file: &str) {
    let mut total_solvable = 0;
    let lines = file.split("\n");
    for line in lines {
        if !line.is_empty() {
            let parts: Vec<&str> = line.split(':').collect();

            let target = parts[0].parse::<u64>().unwrap();
            let inputs: Vec<u64> = parts[1]
                .trim()
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect();

            let combos = generate_combos(inputs.len() - 1);
            for combo in combos {
                let mut total = 0;
                for (i, number) in inputs.iter().enumerate() {
                    if i == 0 {
                        total = *number;
                    } else if combo[i - 1] == '0' {
                        total += *number;
                    } else if combo[i - 1] == '1' {
                        total *= *number;
                    }
                }
                if total == target {
                    total_solvable += total;
                    break;
                }
            }
        }
    }
    println!("The total is {}", total_solvable);
}

pub fn process_file_part_two(file: &str) {
    let mut total_solvable = 0;
    let lines = file.split("\n");
    for line in lines {
        if !line.is_empty() {
            let parts: Vec<&str> = line.split(':').collect();

            let target = parts[0].parse::<u64>().unwrap();
            let inputs: Vec<u64> = parts[1]
                .trim()
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect();

            let combos = generate_combos_part_two(inputs.len() - 1);
            for combo in combos {
                let mut total = 0;
                for (i, number) in inputs.iter().enumerate() {
                    if i == 0 {
                        total = *number;
                    } else if combo[i - 1] == '0' {
                        total += *number;
                    } else if combo[i - 1] == '1' {
                        total *= *number;
                    } else if combo[i - 1] == '2' {
                        total = format!("{}{}", total, number).parse().unwrap();
                    }
                }
                if total == target {
                    total_solvable += total;
                    break;
                }
            }
        }
    }
    println!("The total including concat operator is {}", total_solvable);
}

fn generate_combos(len: usize) -> Vec<Vec<char>> {
    let mut combos: Vec<Vec<char>> = vec![];
    let total = 2_u64.pow(len.try_into().unwrap());
    for num in 0..total {
        let mut bin_rep = format!("{num:b}");
        while bin_rep.len() < len {
            bin_rep = format!("0{}", bin_rep);
        }
        combos.push(bin_rep.chars().collect());
    }
    combos
}

fn generate_combos_part_two(len: usize) -> Vec<Vec<char>> {
    let mut combos: Vec<Vec<char>> = vec![];
    let total = 3_u64.pow(len.try_into().unwrap());
    for num in 0..total {
        let mut base3_rep = radix_3(num).to_string();
        while base3_rep.len() < len {
            base3_rep = format!("0{}", base3_rep);
        }
        combos.push(base3_rep.chars().collect());
    }
    combos
}
