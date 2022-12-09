use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day1_input.txt";

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let parsed_input = contents
        .trim()
        .split("\n\n")
        .map(|l|
             l.split('\n')
             .map(|f| f.parse().unwrap())
        );
    let carries = get_sorted_sums(parsed_input);

    println!("Elapsed: {:?}", start.elapsed());
    println!("D1P1: {:?}", carries[0]);
    println!("D1P2: {:?}", &carries[0..3].iter().sum::<i32>());
}

fn get_sorted_sums(input: impl Iterator<Item = impl Iterator<Item = i32>>) -> Vec<i32> {
    let mut elf_carries = input.map(|e| e.sum()).collect::<Vec<i32>>();
    elf_carries.sort_unstable_by(|a,b| b.cmp(a));
    elf_carries
}
