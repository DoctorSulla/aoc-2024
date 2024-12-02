pub fn get_lists(file: String) -> (Vec<u32>, Vec<u32>) {
    let mut list_one: Vec<u32> = vec![];
    let mut list_two: Vec<u32> = vec![];

    let lines = file.split("\n");
    for line in lines {
        if line.is_empty() {
        } else {
            let (num_one, num_two) = line.split_once("   ").expect("Couldn't split");
            let num_one = num_one.parse::<u32>().unwrap();
            let num_two = num_two.parse::<u32>().unwrap();
            list_one.push(num_one);
            list_two.push(num_two);
        }
    }
    list_one.sort();
    list_two.sort();
    (list_one, list_two)
}

pub fn get_diff((list_one, list_two): (Vec<u32>, Vec<u32>)) {
    let mut i = 0;
    let mut total: u32 = 0;
    while i < list_one.len() {
        total += list_one[i].abs_diff(list_two[i]);
        i += 1;
    }
    println!("The total difference is {:?}", total);
}

pub fn get_similarity_score((list_one, list_two): (Vec<u32>, Vec<u32>)) {
    let mut total = 0;
    for row in list_one.into_iter() {
        let count: u32 = list_two
            .iter()
            .filter(|&n| *n == row)
            .count()
            .try_into()
            .unwrap();
        total += row * count;
    }
    println!("The similarity score is {:?}", total)
}
