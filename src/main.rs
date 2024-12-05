use aoclibrary::day_five::*;
use aoclibrary::day_four::*;
use aoclibrary::day_one::*;
use aoclibrary::day_three::*;
use aoclibrary::day_two::*;
use aoclibrary::read_file;

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
}
