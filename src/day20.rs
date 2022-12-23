use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day20_input.txt";
const SCALE_VAL: i64 = 811589153;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day20");
    let mut p1_inputs: Vec<(usize, i64)> = contents
        .lines()
        .enumerate()
        .map(|(i,v)| (i, v.parse().unwrap()))
        .collect();

    let p1 = mix(&mut p1_inputs.clone(), 1, 1);
    let p2 = mix(&mut p1_inputs, SCALE_VAL, 10);

    println!("Elapsed: {:?}",start.elapsed());
    println!("D20P1: {p1:?}");
    println!("D20P2: {p2:?}");
}

fn mix(grove_coords: &mut Vec<(usize, i64)>, scale: i64, num_mixes: i32) -> i64 {
    let len = grove_coords.len();
    for _ in 0..num_mixes {
        for i in 0..grove_coords.len() {
            let index = grove_coords.iter().position(|v| v.0 == i).unwrap();
            let next_index = (index as i64 + &grove_coords[index].1 * scale).rem_euclid(len as i64 - 1);
            let coord = grove_coords.remove(index);
            grove_coords.insert(next_index as usize, coord);
        }
    }
    let zero_index = grove_coords.iter().position(|&(_, val)| val == 0).unwrap();
    [1000, 2000, 3000].iter().map(|&v| grove_coords[(zero_index + v as usize) % len].1 * scale).sum()
}
