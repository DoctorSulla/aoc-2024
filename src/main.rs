use aoclibrary::day_eight;
use aoclibrary::day_eight::get_antinodes;
use aoclibrary::day_eight::get_grid;
use aoclibrary::day_five::*;
use aoclibrary::day_four::*;
use aoclibrary::day_one::*;
use aoclibrary::day_seven::*;
use aoclibrary::day_six::*;
use aoclibrary::day_three::*;
use aoclibrary::day_two::*;
use aoclibrary::read_file;

use std::time::Instant;

fn main() {
    let file = read_file("./puzzle_inputs/day_one.txt");
    // D1 P1
    get_diff(get_lists(file.clone()));
    // D1 P2
    get_similarity_score(get_lists(file));

    let file = read_file("./puzzle_inputs/day_two.txt");
    // D2 P1
    get_safe_lines(file.clone());
    // D2 P2
    get_safe_line_variants(file);

    let file = read_file("./puzzle_inputs/day_three.txt");
    // D3 P1
    get_valid_mul(file.clone());
    // D3 P2
    parser(file);

    let file = read_file("./puzzle_inputs/day_four.txt");
    // D4 P1
    check_for_xmas(get_lines_vec(file.clone()));
    // D4 P2
    check_for_xmas_two(get_lines_vec(file));

    let rules_file = read_file("./puzzle_inputs/day_five_rules.txt");
    let updates_file = read_file("./puzzle_inputs/day_five_updates.txt");
    // D5 P1
    calculate(rules_file.clone(), updates_file.clone());

    let file = read_file("./puzzle_inputs/day_six.txt");
    // D6 P1
    let map = get_map(file.clone());
    patrol(
        get_starting_position(map.clone()).unwrap(),
        map.clone(),
        true,
    );
    // D6 P2
    let start = Instant::now();
    get_loop_count(map);
    let elapsed_time = start.elapsed();
    println!("Day six part two took {}ms", elapsed_time.as_millis());

    let file = read_file("./puzzle_inputs/day_seven.txt");
    // D7 P1
    process_file(&file);
    process_file_part_two(&file);

    let file = read_file("./puzzle_inputs/day_eight.txt");
    // D8 P1
    get_antinodes(get_grid(&file));
    // D8 P2
}
