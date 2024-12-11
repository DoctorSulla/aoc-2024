fn generate_map(file: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = vec![];
    for line in file.lines() {
        let row: Vec<u32> = line.chars().map(|v| v.to_digit(10).unwrap()).collect();
    }
    map
}
