use std::fs;

const FILE_PATH: &str = "inputs/day25_input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day25");
    let parsed = contents.trim();
    println!("{parsed}");
}
