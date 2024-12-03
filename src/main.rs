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
    parser(file)
}
