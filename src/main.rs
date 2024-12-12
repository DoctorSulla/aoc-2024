use std::collections::HashMap;

// use aoclibrary::day_eight::*;
// use aoclibrary::day_five::*;
// use aoclibrary::day_four::*;
use aoclibrary::day_nine::*;
// use aoclibrary::day_one::*;
// use aoclibrary::day_seven::*;
// use aoclibrary::day_six::*;
// use aoclibrary::day_three::*;
// use aoclibrary::day_two::*;
use aoclibrary::day_eleven::*;
use aoclibrary::day_ten::*;
use aoclibrary::read_file;

//use std::time::Instant;

fn main() {
    // let file = read_file("./puzzle_inputs/day_one.txt");
    // // D1 P1
    // get_diff(get_lists(file.clone()));
    // // D1 P2
    // get_similarity_score(get_lists(file));

    // let file = read_file("./puzzle_inputs/day_two.txt");
    // // D2 P1
    // get_safe_lines(file.clone());
    // // D2 P2
    // get_safe_line_variants(file);

    // let file = read_file("./puzzle_inputs/day_three.txt");
    // // D3 P1
    // get_valid_mul(file.clone());
    // // D3 P2
    // parser(file);

    // let file = read_file("./puzzle_inputs/day_four.txt");
    // // D4 P1
    // check_for_xmas(get_lines_vec(file.clone()));
    // // D4 P2
    // check_for_xmas_two(get_lines_vec(file));

    // let rules_file = read_file("./puzzle_inputs/day_five_rules.txt");
    // let updates_file = read_file("./puzzle_inputs/day_five_updates.txt");
    // // D5 P1
    // calculate(rules_file.clone(), updates_file.clone());

    // let file = read_file("./puzzle_inputs/day_six.txt");
    // // D6 P1
    // let map = get_map(file.clone());
    // patrol(
    //     get_starting_position(map.clone()).unwrap(),
    //     map.clone(),
    //     true,
    // );
    // // D6 P2
    // let start = Instant::now();
    // get_loop_count(map);
    // let elapsed_time = start.elapsed();
    // println!("Day six part two took {}ms", elapsed_time.as_millis());

    // let file = read_file("./puzzle_inputs/day_seven.txt");
    // // D7 P1
    // process_file(&file);
    // process_file_part_two(&file);

    // let file = read_file("./puzzle_inputs/day_eight.txt");
    // // D8 P1/P2
    // get_antinodes(get_grid(&file));

    let file = read_file("./puzzle_inputs/day_nine.txt");
    //let file = read_file("./puzzle_inputs/test_input_nine.txt");
    let file_system = generate_filesystem(&file);
    defrag(file_system.1.clone());

    defrag_part_two(file_system.1, file_system.0);

    // D10 P1
    let file = read_file("./puzzle_inputs/day_ten.txt");
    let map = generate_map(&file);
    walk_map(&map);

    let file = read_file("./puzzle_inputs/day_eleven.txt");
    // D11 P1
    let mut total: u64 = 0;
    let mut rocks = generate_initial_rocks(&file);
    let mut input_output: HashMap<u64, (u64, Option<u64>)> = HashMap::new();
    blink_x_times(25, &mut rocks, &mut total, &mut input_output);
    println!("{}", total);
}
