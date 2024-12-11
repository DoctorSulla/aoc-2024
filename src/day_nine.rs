type Num = u64;

pub fn generate_filesystem(file: &str) -> (Num, Vec<Option<Num>>) {
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
    (id - 1, fs)
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

pub fn defrag_part_two(mut fs: Vec<Option<Num>>, mut max_id: Num) {
    let mut index = fs.len() - 1;
    let mut buffer: Vec<Option<Num>> = vec![];
    let mut current_id = Some(max_id);
    while index > 0 {
        if fs[index] == current_id {
            buffer.push(fs[index].clone());
            index -= 1;
        } else if !buffer.is_empty() {
            let mut i = 0;
            let mut none_count = 0;
            while i < fs.len() {
                if i > index {
                    break;
                }
                if fs[i].is_none() {
                    none_count += 1;
                    if buffer.len() == none_count {
                        // Out with the old
                        for val in fs.iter_mut() {
                            if *val == current_id {
                                *val = None;
                            }
                        }
                        // In with the new
                        i -= none_count - 1;
                        for val in &buffer {
                            fs[i] = *val;
                            i += 1;
                        }
                        buffer.clear();
                        break;
                    }
                } else {
                    none_count = 0;
                }
                i += 1;
            }
            buffer = vec![];
            if max_id == 0 {
                break;
            }
            max_id -= 1;
            current_id = Some(max_id);
        } else {
            index -= 1;
        }
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
