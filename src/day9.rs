use std::fs;
use std::ops::AddAssign;
use std::time::Instant;
use rustc_hash::FxHashSet;

const FILE_PATH: &str = "inputs/day9_input.txt";
const ROPE_LENGTH: usize = 10;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Position { x: i32, y: i32 }

#[derive(Debug)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32)
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }

}

impl Direction {
    fn from_str(str: &str) -> Self {
        use Direction::*;
        let (dir_str, count) = str.split_once(' ').unwrap();
        match dir_str {
            "U" => Up(count.parse().unwrap()),
            "D" => Down(count.parse().unwrap()),
            "L" => Left(count.parse().unwrap()),
            "R" => Right(count.parse().unwrap()),
            _ => panic!("Received an unexpected direction {str}"),
        }
    }

    fn get_offset(&self) -> Position {
        use Direction::*;
        match *self {
            Up(_) => Position { x: 0, y: 1 },
            Down(_) => Position { x: 0, y: -1 },
            Left(_) => Position { x: -1, y: 0 },
            Right(_) => Position { x: 1, y: 0 },
        }
    }

    fn get_count(&self) -> u32 {
        use Direction::*;
        match *self {
            Up(count) |
            Down(count) |
            Left(count) |
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
        
    let (p1, p2) = solve(parsed);

    println!("Elapsed: {:?}", start.elapsed());
    println!("D9P1: {p1:?}");
    println!("D9P2: {p2:?}");
}

fn follow(head: &Position, tail: &mut Position) -> bool {
    let x_offset = head.x - tail.x;
    let y_offset = head.y - tail.y;

    if x_offset.abs() > 1 || y_offset.abs() > 1 {
        *tail += Position {x: x_offset.signum(), y: y_offset.signum()};
        return true;
    }
    false
}

fn solve(directions: impl Iterator<Item = Direction>) -> (usize, usize) {
    let mut rope_positions: [Position; ROPE_LENGTH] = [Position {x: 0, y: 0}; ROPE_LENGTH];
    let tail = ROPE_LENGTH - 1;

    let (mut second_positions, mut tail_positions) = (FxHashSet::default(), FxHashSet::default());
    tail_positions.insert(rope_positions[tail]);
    second_positions.insert(rope_positions[1]);

    directions.for_each(|direction| {
        let count = direction.get_count();

        for _ in 0..count {
            rope_positions[0] += direction.get_offset();
            for i in 1..ROPE_LENGTH {
                let (heads, tails) = rope_positions.split_at_mut(i);
                let modified = follow(heads.last().unwrap(), &mut tails[0]);
                if !modified {
                    break;
                }
                if i == 1 {
                    second_positions.insert(rope_positions[1]);
                }
                if i == tail {
                    tail_positions.insert(rope_positions[tail]);
                }
            }
        }
    });

    (second_positions.len(), tail_positions.len())
}
