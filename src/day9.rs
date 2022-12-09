use std::fs;
use std::time::Instant;
use rustc_hash::FxHashSet;

const FILE_PATH: &str = "inputs/day9_input.txt";
type Position = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32)
}

impl Direction {
    fn from_str(str: &str) -> Self {
        use Direction::*;
        let mut split = str.split_whitespace();
        let dir_str = split.next().unwrap();
        let count: u32 = split.next().unwrap().parse().unwrap();
        match dir_str {
            "U" => Up(count),
            "D" => Down(count),
            "L" => Left(count),
            "R" => Right(count),
            _ => panic!("Received an unexpected direction {str}"),
        }
    }

    fn get_offset(&self) -> Position {
        use Direction::*;
        match *self {
            Up(_) => (0, 1),
            Down(_) => (0, -1),
            Left(_) => (-1, 0),
            Right(_) => (1, 0),
        }
    }

    fn get_count(&self) -> u32 {
        use Direction::*;
        match *self {
            Up(count) => count,
            Down(count) => count,
            Left(count) => count,
            Right(count) => count,
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day9");
    let parsed = contents
        .lines()
        .map(Direction::from_str);
    let p1 = solve(parsed.clone(), 2);
    let p2 = solve(parsed, 10);

    println!("Elapsed: {:?}", start.elapsed());
    println!("D9P1: {p1:?}");
    println!("D9P2: {p2:?}");
}

fn get_new_tail_position(head: Position, tail: Position) -> Position {
    let x_offset = head.0 - tail.0;
    let y_offset = head.1 - tail.1;

    // Tail does not move if it is within one row and column of x
    if x_offset.abs() <= 1 && y_offset.abs() <= 1 {
        tail
    } else {
        (tail.0 + x_offset.signum(), tail.1 + y_offset.signum())
    }
}

fn solve(directions: impl Iterator<Item = Direction>, rope_length: usize) -> usize {
    let mut rope_positions: Vec<Position> = vec![(0,0); rope_length];
    let mut next_positions = rope_positions.clone();
    let mut tail_positions: FxHashSet<Position> = FxHashSet::default();
    tail_positions.insert(*rope_positions.last().unwrap());

    for direction in directions {
        let (dx, dy) = direction.get_offset();
        let count = direction.get_count();

        for _ in 0..count {
            next_positions[0].0 += dx;
            next_positions[0].1 += dy;
            let mut changed = 0;
            for (i, knot) in rope_positions[1..].iter().enumerate() {
                let next = get_new_tail_position(next_positions[i], *knot);
                if next == *knot {
                    break;
                }
                changed += 1;
                next_positions[i + 1] = next;
            }

            for (i, knot) in next_positions[0..=changed].iter().enumerate() {
                rope_positions[i] = *knot;
            }
            if changed == rope_positions.len() - 1 {
                tail_positions.insert(*rope_positions.last().unwrap());
            }
        }
    }

    tail_positions.len()
}
