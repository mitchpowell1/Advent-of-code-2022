use std::fs;

const FILE_PATH: &str = "inputs/day24_input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day24");
    let parsed = contents.trim();
    println!("{parsed}");
}
