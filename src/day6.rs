use std::fs;
use std::time::Instant;
use std::cmp::PartialEq;

const FILE_PATH: &str = "inputs/day6_input.txt";

trait AllUnique {
    fn all_unique(&self) -> bool;
}

impl<T: PartialEq<T>> AllUnique for Vec<T> {
    fn all_unique(&self) -> bool {
        self.iter().enumerate().all(|(i, v1)|
            self[i..].iter().filter(|&v2| *v1 == *v2).count() == 1
        )
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day6");
    let p1 = evaluate(&contents, 4);
    let p2 = evaluate(&contents, 14);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D6P1: {p1:?}");
    println!("D6P1: {p2:?}");
}

fn evaluate(input: &str, marker_length: usize) -> usize {
    let mut marker_chars = vec!['0'; marker_length];
    for (i, c) in input.chars().enumerate() {
        if i >= marker_length && marker_chars.all_unique() {
            return i;
        }
        marker_chars[i % marker_length] = c;
    }

    panic!("Should not reach this");
}
