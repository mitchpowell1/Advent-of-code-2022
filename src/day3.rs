use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day3_input.txt";
static LC_ASCII_OFFSET: u8 = 96;
static UC_ASCII_OFFSET: u8 = 64;
static ALPHABET_WIDTH: u8 = 26;
const WINDOW_SIZE: usize = 3;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day3");
    let parsed = contents.trim().lines();
    let p1 = part_one(parsed.clone());
    let p2 = part_two(parsed);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D3P1: {p1:?}");
    println!("D3P2: {p2:?}");
}

fn find_common_byte(iterators: &[impl Iterator<Item = u8> + Clone]) -> Option<u8> {
    let first = iterators[0].clone();
    'outer: for v1 in first {
        for it in iterators[1..].iter() {
            if !it.clone().any(|v2| v2 == v1) {
                continue 'outer;
            }
        }
        return Some(v1);
    }
    None
}

#[inline(always)]
fn get_priority(char_code: u8) -> i32 {
    if char_code < b'a' {
        return (char_code - UC_ASCII_OFFSET + ALPHABET_WIDTH) as i32;
    }
    (char_code - LC_ASCII_OFFSET) as i32
}

fn part_one<'a>(rucksacks: impl Iterator<Item = &'a str>) -> i32 {
    rucksacks.fold(0, |acc, sack| {
        let compartment_size = sack.len() / 2;
        let common = find_common_byte(&[sack[..compartment_size].bytes(), sack[compartment_size..].bytes()]).unwrap();
        acc + get_priority(common)
    })
}

fn part_two<'a>(rucksacks: impl Iterator<Item = &'a str>) -> i32 {
    let mut rucksack_window: [&str; WINDOW_SIZE] = [""; WINDOW_SIZE];
    let mut total = 0;
    for (i, sack) in rucksacks.enumerate() {
        rucksack_window[i % WINDOW_SIZE] = sack;

        if (i + 1) % WINDOW_SIZE == 0 {
            total += get_priority(find_common_byte(&rucksack_window.map(|s|s.bytes())).unwrap());
        }
    }
    total
}
