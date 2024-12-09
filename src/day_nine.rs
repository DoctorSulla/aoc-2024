type Num = u64;

pub fn generate_filesystem(file: &str) -> Vec<Option<Num>> {
    let mut fs: Vec<Option<Num>> = vec![];
    let mut id: Num = 0;
    let mut empty_space = false;
    for char in file.chars() {
        let count: u32 = char.to_digit(10).unwrap_or(0);

        for _num in 0..count {
            if empty_space {
                fs.push(None);
            } else {
                fs.push(Some(id));
            }
        }
        if !empty_space {
            id += 1;
        }
        empty_space = !empty_space;
    }
    fs
}

pub fn defrag(mut fs: Vec<Option<Num>>) {
    let mut index = fs.len() - 1;
    loop {
        if fs[index].is_some() {
            let move_to = fs.iter().position(|x| x.is_none()).unwrap();
            if move_to > index {
                break;
            } else {
                fs.swap(index, move_to);
            }
        }
        index -= 1;
    }
    let mut checksum: Num = 0;

    for num in 1..fs.len() {
        if let Some(multiply) = fs[num] {
            let index: u64 = num.try_into().unwrap();
            checksum += index * multiply
        }
    }
    println!("The checksum is {}", checksum);
}
