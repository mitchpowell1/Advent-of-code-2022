use std::fs;
use std::time::Instant;
use std::ops::RangeInclusive;
use std::cmp::PartialOrd;

const FILE_PATH: &str = "inputs/day4_input.txt";

trait ContainsOther<T=Self> {
    fn contains_other(&self, other: &T) -> bool;
}

trait OverlapsOther<T=Self> {
    fn overlaps_other(&self, other: &T) -> bool;
}

impl<T: PartialOrd> ContainsOther for RangeInclusive<T> {
    fn contains_other(&self, other: &RangeInclusive<T>) -> bool{
        other.contains(self.start()) && other.contains(self.end())
    }
}

impl<T: PartialOrd> OverlapsOther for RangeInclusive<T> {
    fn overlaps_other(&self, other: &RangeInclusive<T>) -> bool {
        other.contains(self.start()) ||
        other.contains(self.end()) ||
        self.contains(other.start()) ||
        self.contains(other.end())
    }
}


fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day4");
    let parsed = contents
        .lines()
        .map(|line| {
            let mut ranges = line.split(&['-', ',']).map(|d| d.parse().unwrap());
            let b1: u8 = ranges.next().unwrap();
            let e1: u8 = ranges.next().unwrap();

            let b2: u8 = ranges.next().unwrap();
            let e2: u8 = ranges.next().unwrap();

            (b1..=e1, b2..=e2)
        });

    let (p1, p2) = evaluate(parsed);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D4P1: {p1:?}");
    println!("D4P2: {p2:?}");
}

fn evaluate(ranges: impl Iterator<Item = (RangeInclusive<u8>, RangeInclusive<u8>)>) -> (i32, i32) {
    let mut containment_total = 0;
    let mut overlap_total = 0;
    for (r1, r2) in ranges {
        if r1.contains_other(&r2) || r2.contains_other(&r1) {
            containment_total += 1;
        }
        if r1.overlaps_other(&r2) {
            overlap_total += 1;
        }
    }
    (containment_total, overlap_total)
}
