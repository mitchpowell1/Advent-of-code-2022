use std::fs;
use std::thread;
use std::sync::Arc;
use std::time::Instant;
use std::collections::VecDeque;
use rustc_hash::FxHashSet;

const FILE_PATH: &str = "inputs/day12_input.txt";
const OFFSETS: [(i32,i32); 4] = [(1, 0), (-1, 0), (0, 1), (0,-1)];

type Grid<T> = Vec<Vec<T>>;

type Point = (usize, usize);

fn parse_input(in_str: &str) -> (Grid<char>, Point, Point) {
    let mut grid = vec!();
    let (mut start, mut end) = ((0,0), (0,0));

    for line in in_str.lines() {
        let mut row = vec!();
        for (c, char)in line.chars().enumerate() {
            row.push(char);
            if char == 'S' {
                start = (grid.len(), c);
            }
            if char == 'E' {
                end = (grid.len(), c);
            }
        }
        grid.push(row);
    }
    (grid, start, end)
}

fn main() {
    let t = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day12");
    let (grid, start, end) = parse_input(&contents);
    let arc1 = Arc::new(grid);
    let arc2 = arc1.clone();

    let p2_handle = thread::spawn(move || calculate_min_path(&arc1, &end, 'a', false));

    let p1 = calculate_min_path(&arc2, &start, 'E', true);
    let p2 = p2_handle.join().expect("Panic occurred during p2");

    println!("Elapsed: {:?}", t.elapsed());
    println!("D12P1: {p1:?}");
    println!("D12P2: {p2:?}");
}

fn get_elevation(ch: char) -> u8 {
    match ch {
        'S' => b'a',
        'E' => b'z',
        _ => ch as u8
    }
}

fn calculate_min_path(grid: &Grid<char>, start: &Point, end_char: char, ascending: bool) -> u16 {
    let (width, height) = (grid[0].len(), grid.len());
    let mut visited: FxHashSet<Point> = FxHashSet::default();
    let mut to_visit: VecDeque<(Point, u16)> = VecDeque::new();
    to_visit.push_front((*start, 0));

    while let Some((current, steps)) = to_visit.pop_front() {
        visited.insert(current);
        let current_height = get_elevation(grid[current.0][current.1]);
        for offset in OFFSETS {
            let r1 = offset.0 + current.0 as i32;
            let c1 = offset.1 + current.1 as i32;
            if (r1 < 0 || r1 as usize >= height) || (c1 < 0 || c1 as usize >= width) {
                continue
            }
            let (r1, c1) = (r1 as usize, c1 as usize);
            let next = (r1, c1);
            let next_height = get_elevation(grid[r1][c1]);
            let heights_match = if ascending { next_height <= current_height + 1 } else { current_height <= next_height + 1 };
            if !heights_match || visited.contains(&next) {
                continue;
            }
            if grid[r1][c1] == end_char {
                return steps + 1;
            }
            to_visit.push_back((next, steps + 1));
            visited.insert(next);
        }
    }
    panic!("Not Found!")
}
