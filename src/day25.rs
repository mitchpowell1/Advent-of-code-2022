use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day25_input.txt";

fn get_value(in_str: &str) -> i64 { 
    let mut out = 0;
    for c in in_str.chars() {
        out *= 5;
        match c {
            '-' => out -= 1,
            '=' => out -= 2,
            _ => out += c.to_digit(10).unwrap() as i64,
        }
    }
    out
}

fn convert_to_snafu(number: i64) -> String {
    if number == 0 {
        return String::new();
    }
    match number % 5 {
        3 => convert_to_snafu(1 + number / 5) + "=",
        4 => convert_to_snafu(1 + number / 5) + "-",
        _ => convert_to_snafu(number / 5) + &(number % 5).to_string(),
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day25");
    let sum: i64 = contents.as_str().trim().lines().map(get_value).sum();
    let p1 = convert_to_snafu(sum);
    println!("Elapsed: {:?}", start.elapsed());
    println!("P1: {p1:?}");
}
