use aoclibrary::day_one::*;
use aoclibrary::day_two::*;
use aoclibrary::read_file;

fn main() {
    let file = read_file("./puzzle_inputs/day_one.txt");
    // D1 P1
    get_diff(get_lists(file));

    let file = read_file("./puzzle_inputs/day_one.txt");
    // D1 P2
    get_similarity_score(get_lists(file));

    let file = read_file("./puzzle_inputs/day_two.txt");
    // D2 P1
    get_safe_lines(file);
}
