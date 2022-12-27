use std::fs;
use std::cmp::{max, min};
use std::time::Instant;

use tinyvec::{ArrayVec, array_vec};
use rustc_hash::FxHashMap;

const FILE_PATH: &str = "inputs/day23_input.txt";
const DIRECTION_PRIORITIES: [Direction; 4] = [Direction::North, Direction::South, Direction::West, Direction::East];
const P1_ITERATIONS: usize = 10;
const ELF_POSITION_BOUND: usize = 200;
const OFFSET: i32 = 75;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (-1, 0),
            South => ( 1, 0),
            East  => ( 0, 1),
            West  => (0, -1),
        }
    }

    fn get_check_offsets(&self) -> [(i32, i32); 3] {
        use Direction::*; 
        match self {
            North => [(-1, -1), (-1, 0), (-1, 1)],
            South => [(1, -1), (1, 0), (1, 1)],
            East  => [(-1, 1), (0, 1), (1, 1)],
            West  => [(-1, -1), (0, -1), (1, -1)],
        }
    }
}

fn parse_elf_positions(input: &str, position_store: &mut [[bool; ELF_POSITION_BOUND]; ELF_POSITION_BOUND]) -> Vec<(i32, i32)> {
    let mut out = vec!();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    position_store[i + OFFSET as usize][j + OFFSET as usize] = true;
                    out.push((i as i32, j as i32));
                },
                _ => continue,
            };
        }
    }
    out
}
fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day23");
    let mut position_store = [[false; ELF_POSITION_BOUND]; ELF_POSITION_BOUND];
    let mut positions = parse_elf_positions(&contents, &mut position_store);
    let (p1, p2) = simulate(&mut position_store, &mut positions);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D23P1: {p1:?}");
    println!("D23P2: {p2:?}");
}

fn get_bounding_rect(elf_positions: &Vec<(i32, i32)>) -> [i32; 4] {
    let (mut left, mut right, mut top, mut bottom) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN); 
    for (i, j) in elf_positions {
        top = min(top, *i);
        bottom = max(bottom, *i);
        left = min(left, *j);
        right = max(right, *j);
    }
    [top, bottom, left, right]
}

fn simulate(position_store: &mut [[bool; ELF_POSITION_BOUND]; ELF_POSITION_BOUND], elf_positions: &mut Vec<(i32, i32)>) -> (i32, i32) {
    let mut proposed: FxHashMap<(i32, i32), ArrayVec<[usize; 4]>> = FxHashMap::default();
    let mut first_considered_direction = 0;
    let (mut p1, mut p2) = (0, 0);
    let mut iteration = 0;

    'main : loop {
        first_considered_direction = iteration % 4;
        iteration += 1;
        let mut elves_moved = false;
        proposed.clear();
        // let mut elves_who_didnt_propose = 0;
        for (elf_index, (i, j)) in elf_positions.iter().enumerate() {
            // First each elf checks every square around them and doesn't move if they are all unoccupied
            let mut all_empty = true;
            'outer: for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue
                    }

                    if position_store[((i + di) + OFFSET) as usize][((j + dj) + OFFSET) as usize] {
                        all_empty = false;
                        break 'outer;
                    }
                }
            }
            if all_empty {
                continue;
            }

            for position_index in first_considered_direction..first_considered_direction + 4 {
                let direction = DIRECTION_PRIORITIES[position_index % 4];
                if direction.get_check_offsets().iter().all(|(di, dj)| !position_store[((i + di) + OFFSET) as usize][((j + dj) + OFFSET) as usize]) {
                    let offset = direction.get_offset();
                    let proposal = (i + offset.0, j + offset.1);
                    if let Some(arr) = proposed.get_mut(&proposal) {
                        arr.push(elf_index);
                    } else {
                        proposed.insert(proposal, array_vec!([usize; 4] => elf_index));
                    }
                    break;
                }
            }
        }
        for (proposal, proposers) in proposed.iter() {
            if proposers.len() > 1 {
                continue;
            } else {
                elves_moved = true;
                let proposer = proposers.first().unwrap();
                let original = elf_positions[*proposer];
                position_store[(original.0 + OFFSET) as usize][(original.1 + OFFSET) as usize] = false;
                position_store[(proposal.0 + OFFSET) as usize][(proposal.1 + OFFSET) as usize] = true;
                elf_positions[*proposer] = *proposal;
            }
        }
        if !elves_moved {
            p2 = iteration;
            break 'main;
        }
        if iteration == P1_ITERATIONS {
            let [top, bottom, left, right] = get_bounding_rect(&elf_positions);
            let area = ((bottom - top) + 1) * ((right - left) + 1);
            p1 = area - elf_positions.len() as i32;
        }
    }

    (p1, p2 as i32)
}
