fn get_rules(file: String) -> Vec<Vec<usize>> {
    let mut rules: Vec<Vec<usize>> = vec![];

    let lines = file.split("\n");
    for line in lines {
        if line.is_empty() {
        } else {
            let rule: Vec<usize> = line.split('|').map(|v| v.parse().unwrap()).collect();
            rules.push(rule);
        }
    }
    rules
}

fn get_updates(file: String) -> Vec<Vec<usize>> {
    let mut updates: Vec<Vec<usize>> = vec![];

    let lines = file.split("\n");
    for line in lines {
        if line.is_empty() {
        } else {
            let rule: Vec<usize> = line.split(',').map(|v| v.parse().unwrap()).collect();
            updates.push(rule);
        }
    }
    updates
}

pub fn calculate(rules_file: String, updates_file: String) {
    let mut middle_totals = 0;
    let mut fixed_totals = 0;
    let rules = get_rules(rules_file);
    let updates = get_updates(updates_file);

    for mut update in updates {
        let mut valid = true;
        for rule in rules.iter() {
            let pos_one = update.iter().position(|x| x == &rule[0]);
            let pos_two = update.iter().position(|x| x == &rule[1]);
            if pos_one.is_some() && pos_two.is_some() && pos_one.unwrap() > pos_two.unwrap() {
                valid = false;
            }
        }
        if valid {
            middle_totals += update[update.len() / 2];
        } else {
            while !valid {
                valid = true;
                for rule in rules.iter() {
                    let pos_one = update.iter().position(|x| x == &rule[0]);
                    let pos_two = update.iter().position(|x| x == &rule[1]);

                    if pos_one.is_some() && pos_two.is_some() && pos_one.unwrap() > pos_two.unwrap()
                    {
                        valid = false;
                        update.swap(pos_one.unwrap(), pos_two.unwrap());
                    }
                }
            }
            fixed_totals += update[update.len() / 2];
        }
    }
    println!("Total of valid instructions is {}", middle_totals);
    println!("Total of fixed instructions is {}", fixed_totals);
}
