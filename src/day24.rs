use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;
use rustc_hash::FxHashSet;

use num::integer::lcm;

const FILE_PATH: &str = "inputs/day24_input.txt";
const START: (i32, i32) = (0, 1);
const OFFSETS: [(i32, i32); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];
const P2_TRIPS: i32 = 3;
type Map = Vec<Vec<u8>>;

#[repr(u8)]
#[derive(Copy, Clone)]
enum SquareFlags {
    BlizUp  = 1,
    BlizDown = 2,
    BlizRight = 4,
    BlizLeft = 8,
    Wall = 16,
}

impl SquareFlags {
    fn from_char(c: char) -> Self {
        use SquareFlags::*;
        match c {
            '>' => BlizRight,
            '<' => BlizLeft,
            '^' => BlizUp,
            'v' => BlizDown,
            '#' => Wall,
            _ => panic!(),
        }
    }
}

fn parse_input(in_str: &str) -> Map {
    let mut out = vec!();
    for line in in_str.lines() {
        let mut row = vec!();
        for char in line.chars() {
            match char {
                '.' => row.push(0),
                _ => row.push(SquareFlags::from_char(char) as u8),
            }
        }
        out.push(row);
    }
    out
}

fn get_next_state(map: &Map) -> Map {
    let mut clone = map.clone();
    use SquareFlags::*;
    let (height, width) = (map.len(), map[0].len());
    for i in 0..height {
        for j in 0..width {
            clone[i][j] = map[i][j] & SquareFlags::Wall as u8;
        }
    }
    for (i, row) in map.iter().enumerate() {
        for (j, square) in row.iter().enumerate() {
            if BlizRight as u8 & square != 0 {
                let mut next_j = (j + 1) % (width - 1);
                if next_j == 0 { next_j = 1 };
                clone[i][next_j] |= BlizRight as u8;
            }
            if BlizDown as u8 & square != 0 {
                let mut next_i = (i + 1) % (height - 1);
                if next_i == 0 { next_i = 1 };
                clone[next_i][j] |= BlizDown as u8;
            }
            if BlizLeft as u8 & square != 0 {
                let mut next_j = ((j - 1) as i32).rem_euclid((width - 1) as i32) as usize;
                if next_j == 0 {
                    next_j = width - 2
                }
                clone[i][next_j] |= BlizLeft as u8;
            }
            if BlizUp as u8 & square != 0 {
                let mut next_i = ((i - 1) as i32).rem_euclid((height - 1) as i32) as usize;
                if next_i == 0 {
                    next_i = height - 2
                }
                clone[next_i][j] |= BlizUp as u8;
            }
        }
    }

    clone
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day24");
    let map = parse_input(&contents);
    let (p1, p2) = solve(map);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D24P1: {p1:?}");
    println!("D24P2: {p2:?}");
}

fn get_all_states(map: Map) -> Vec<Map> {
    let (height, width) = (map.len(), map[0].len());
    let iterations = lcm(height - 2, width - 2);
    let mut out = vec!(map);
    for _ in 0..iterations - 1 {
        out.push(get_next_state(out.last().unwrap()));
    }
    out
}

fn contains_flags(square: u8, flags: u8) -> bool { 
    (square & flags) != 0
}

fn manhattan_distance((r1, c1): (i32, i32), (r2, c2): (i32, i32)) -> i32 { 
    (r1 - r2).abs() + (c1 - c2).abs()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    position: (i32, i32),
    minutes: i32,
    min_distance: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.min_distance + other.minutes).cmp(&(self.min_distance + self.minutes))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(map: Map) -> (i32, i32) {
    let (height, width) = (map.len(), map[0].len());
    let all_states = get_all_states(map);
    let mut to_visit = BinaryHeap::new();
    let mut visited = FxHashSet::default();
    let bad_flags = 
        SquareFlags::BlizUp as u8 | 
        SquareFlags::BlizDown as u8 | 
        SquareFlags::BlizLeft as u8 |
        SquareFlags::BlizRight as u8 |
        SquareFlags::Wall as u8;

    let goals = [(height as i32 - 1, width as i32 - 2), START];
    let mut goal = goals[0];
    let mut p1 = None;
    let mut trips = 0;
    let start_state = State { position: START, minutes: 0, min_distance: manhattan_distance(START, goal) };
    to_visit.push(start_state);
    visited.insert(start_state);
    'main: while let Some(State { position, minutes, .. }) = to_visit.pop() {
        let (r, c) = position;
        for (dr, dc) in OFFSETS {
            let (r, c) = (r + dr, c + dc);
            if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                continue;
            }
            let next_moves = minutes + 1;
            if r == goal.0 && c == goal.1 {
                trips += 1;
                if trips < P2_TRIPS {
                    if p1.is_none() {
                        p1 = Some(next_moves);
                    }
                    goal = goals[trips as usize % 2];
                    let next_state = State {
                        position: (r, c),
                        minutes: next_moves,
                        min_distance: manhattan_distance((r,c), goal),
                    };
                    to_visit.clear();
                    visited.clear();
                    to_visit.push(next_state);
                    visited.insert(next_state);
                    continue 'main;
                } else {
                    return (p1.unwrap(), next_moves)
                }
            }
            let next_state = &all_states[next_moves as usize % all_states.len()];
            if contains_flags(next_state[r as usize][c as usize], bad_flags) {
                continue;
            }

            let new_state = State { 
                position: (r, c),
                minutes: minutes + 1, 
                min_distance: manhattan_distance((r, c), goal),
            };

            if visited.insert(new_state) {
                to_visit.push(new_state);
            }
        }
    }
    unreachable!()
}
