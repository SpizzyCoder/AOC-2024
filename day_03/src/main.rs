use std::fs;

mod part1;
mod part2;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    part1::solve(&input);
    part2::solve(&input);
}
