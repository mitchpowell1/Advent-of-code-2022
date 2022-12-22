use std::fs;

const FILE_PATH: &str = "inputs/day20_input_test.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day20");
    let parsed: Vec<i32> = contents.lines().map(|v| v.parse().unwrap() ).collect();
    let p1 = part_one(&parsed);
    println!("D20P1: {p1:?}");
}

fn part_one(grove_coords: &Vec<i32>) -> i32 {
    let mut cloned_coords = grove_coords.clone();

    //println!("{cloned_coords:?}");
    for &coord in grove_coords.iter() {
        let current_index = cloned_coords.iter().position(|&v| v == coord).unwrap() as i32;
        let mut next_index = if coord < 0 { current_index + (coord - 1) } else { current_index + coord };
        let len = grove_coords.len() as i32;
        next_index = ((next_index % len) + len) % len;

        // if next_index < 0 {
        //     next_index += grove_coords.len() as i32;
        // }
        //
        // next_index = next_index % (cloned_coords.len() as i32);

        cloned_coords.insert(next_index as usize, coord);
        if next_index >= current_index {
            cloned_coords.remove(current_index as usize);
        } else {
            cloned_coords.remove((current_index + 1) as usize);
        }
        println!("{cloned_coords:?}");
    }

    let zero_index = cloned_coords.iter().position(|&v| v == 0).unwrap(); 
    [1000, 2000, 3000].into_iter().map(|i| cloned_coords[(zero_index + i) % grove_coords.len()]).sum()
}
