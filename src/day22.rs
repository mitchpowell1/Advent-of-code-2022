use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day22_input.txt";
const NUM_DIRECTIONS: u8 = 4;
const CUBE_FACE_SIZE: usize = 50;
const MAP_HEIGHT: usize = CUBE_FACE_SIZE * 4;
const MAP_WIDTH: usize = CUBE_FACE_SIZE * 3;

type Map = [[MapSpace; MAP_WIDTH]; MAP_HEIGHT];

#[derive(Debug, PartialEq, Copy, Clone)]
enum MapSpace {
    Wall,
    Open,
    Unavailable,
}

impl MapSpace {
    fn from_char(c: char) -> Self {
        match c {
            '.' => MapSpace::Open,
            '#' => MapSpace::Wall,
            _ => MapSpace::Unavailable,
        }
    }
}

impl std::fmt::Display for MapSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            MapSpace::Wall => write!(f, "#")?,
            MapSpace::Open => write!(f, ".")?,
            MapSpace::Unavailable => write!(f, " ")?,
        };
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Orientation {
    fn turn(&self, direction: Turn) -> Self {
        use Orientation::*;
        let current = *self as i32;
        let next = (current + direction as i32).rem_euclid(NUM_DIRECTIONS as i32);
        match next {
            _ if next == Right as i32 => Right,
            _ if next == Down as i32 => Down,
            _ if next == Left as i32 => Left,
            _ if next == Up as i32 => Up,
            _ => unreachable!(),
        }
    }

    fn offset(&self) -> (i32, i32) {
        use Orientation::*;
        match self {
            Right => (0, 1),
            Left => (0, -1),
            Up => (-1, 0),
            Down => (1, 0),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Walk(u32),
    Turn(Turn),
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Right = 1,
    Left = -1,
}

impl Turn {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => unreachable!(),
        }
    }
}

fn parse_map(in_str: &str) -> Map {
    let mut map = [[MapSpace::Unavailable; MAP_WIDTH]; MAP_HEIGHT];
    for (i, line) in in_str.lines().enumerate() {
        line.chars().enumerate().for_each(|(j, c)| map[i][j] = MapSpace::from_char(c));
    }
    map
}

fn parse_directions(in_str: &str) -> Vec<Direction> {
    let mut out = vec!();
    let mut iter = in_str.chars().peekable();
    while let Some(c) = iter.next() {
        if c.is_digit(10) {
            let mut val = c.to_digit(10).unwrap();
            while iter.peek().is_some() && iter.peek().unwrap().is_digit(10) {
                val *= 10;
                val += iter.next().unwrap().to_digit(10).unwrap();
            }
            out.push(Direction::Walk(val));
        } else {
            out.push(Direction::Turn(Turn::from_char(c)));
        }
    }
    out
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day22");
    let (raw_map, raw_directions) = contents.trim_end().split_once("\n\n").unwrap();
    let map = parse_map(raw_map);
    let directions = parse_directions(raw_directions);
    let p1 = part_one(&map, &directions);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D22P1: {p1:?}");
}

fn part_one(map: &Map, directions: &[Direction]) -> i32 {
    let mut position = (0 as usize, map[0].iter().position(|m| *m == MapSpace::Open).unwrap());
    let mut orientation = Orientation::Right;
    let add_offset = |(row, col): (usize, usize), (dr, dc): (i32, i32)| {
        ((row as i32 + dr).rem_euclid(MAP_HEIGHT as i32) as usize, (col as i32 + dc).rem_euclid(MAP_WIDTH as i32) as usize)
    };

    for direction in directions {
        match direction {
            Direction::Walk(v) => {
                for _ in 0..*v {
                    let (mut next_row, mut next_col) = add_offset(position, orientation.offset());
                    while map[next_row][next_col] == MapSpace::Unavailable {
                        (next_row, next_col) = add_offset((next_row, next_col), orientation.offset());
                    }
                    if map[next_row][next_col] == MapSpace::Wall {
                        break;
                    }
                    position = (next_row, next_col);
                }
            },
            Direction::Turn(dir) => orientation = orientation.turn(*dir),
        }
    }
    (1000 * (position.0 + 1) + 4 * (position.1 + 1)) as i32 + orientation as i32
}

fn part_two(map: &Map, directions: &[Direction]) -> i32 {
    let mut position = (0 as usize, map[0].iter().position(|m| *m == MapSpace::Open).unwrap());
    let mut orientation = Orientation::Right;
    let add_offset = |(row, col): (usize, usize), (dr, dc): (i32, i32)| {
        let next_row = (row as i32 + dr).rem_euclid(MAP_HEIGHT as i32) as usize;
        let next_col = (col as i32 + dc).rem_euclid(MAP_WIDTH as i32) as usize;
        (next_row, next_col, Orientation::Right)
    };

    for direction in directions {
        match direction {
            Direction::Walk(v) => {
                for _ in 0..*v {
                    let (mut next_row, mut next_col, mut next_orientation) = add_offset(position, orientation.offset());
                    if map[next_row][next_col] == MapSpace::Wall {
                        break;
                    }
                    position = (next_row, next_col);
                    orientation = next_orientation;
                }
            },
            Direction::Turn(dir) => orientation = orientation.turn(*dir),
        }
    }
    (1000 * (position.0 + 1) + 4 * (position.1 + 1)) as i32 + orientation as i32
}
