pub fn process_file(file: &String) {
    let mut total = 0;
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
            if check_possible(target, inputs) {
                total += target;
            }
        }
    }
    println!("The total is {}", total);
}

fn check_possible(target: u64, inputs: Vec<u64>) -> bool {
    true
}

fn generate_combos(inputs: Vec<char>, len: usize) {}
