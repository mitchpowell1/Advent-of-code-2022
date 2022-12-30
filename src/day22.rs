use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day22_input.txt";
const NUM_DIRECTIONS: u8 = 4;
const CUBE_FACE_SIZE: usize = 50;
const MAP_HEIGHT: usize = CUBE_FACE_SIZE * 4;
const MAP_WIDTH: usize = CUBE_FACE_SIZE * 3;

type Map = [[MapSpace; MAP_WIDTH]; MAP_HEIGHT];
type Cube = [Face; 6];
type Transform = dyn Fn((i32, i32)) -> (usize,(i32, i32), Orientation);
type FaceSquares = [[MapSpace; CUBE_FACE_SIZE]; CUBE_FACE_SIZE];

struct Face {
    spaces: FaceSquares,
    map_offset: (usize, usize),
    p1_transforms: [Box<Transform>; 4],
    p2_transforms: [Box<Transform>; 4],
}

impl Face {
    fn get_map_position(&self, (row, col): (usize, usize)) -> (usize, usize) {
        (row + self.map_offset.0, col + self.map_offset.1)
    }
}

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

fn parse_cube(map: Map) -> Cube {
    use Orientation::*;
    let spaces = [
        (0..50, 50..100),
        (0..50, 100..150),
        (50..100, 50..100),
        (100..150, 0..50),
        (100..150, 50..100),
        (150..200, 0..50),
    ].into_iter().map(|(row_range, col_range)| {
        let mut face_values = [[MapSpace::Open; CUBE_FACE_SIZE]; CUBE_FACE_SIZE];
        for (i, row) in row_range.enumerate() {
            for (j, col) in col_range.clone().enumerate() {
                face_values[i][j] = map[row][col];
            }
        }
        face_values
    });
    let p1_transforms: [[Box<Transform>; 4]; 6] = [
        // 0
        [
            Box::new(|(_, col)| (4, ((CUBE_FACE_SIZE as i32 -1), col), Up)),
            Box::new(|(row, _)| (1, (row, 0), Right)),
            Box::new(|(_, col)| (2, (0, col), Down)),
            Box::new(|(row, _)| (1, (row, (CUBE_FACE_SIZE as i32 -1)), Left)),
        ],
        // 1
        [
            Box::new(|(_, col)| (1, ((CUBE_FACE_SIZE as i32 -1), col), Up)),
            Box::new(|(row, _)| (0, (row, 0), Right)),
            Box::new(|(_, col)| (1, (0, col), Down)),
            Box::new(|(row, _)| (0, (row, (CUBE_FACE_SIZE as i32 -1)), Left)),
        ],
        // 2
        [
            Box::new(|(_, col)| (0, ((CUBE_FACE_SIZE as i32 -1), col), Up)),
            Box::new(|(row, _)| (2, (row, 0), Right)),
            Box::new(|(_, col)| (4, (0, col), Down)),
            Box::new(|(row, _)| (2, (row, (CUBE_FACE_SIZE as i32 -1)), Left)),
        ],
        // 3
        [
            Box::new(|(_, col)| (5, ((CUBE_FACE_SIZE as i32 -1), col), Up)),
            Box::new(|(row, _)| (4, (row, 0), Right)),
            Box::new(|(_, col)| (5, (0, col), Down)),
            Box::new(|(row, _)| (4, (row, (CUBE_FACE_SIZE as i32 -1)), Left)),
        ],
        // 4
        [
            Box::new(|(_, col)| (2, ((CUBE_FACE_SIZE as i32 -1), col), Up)),
            Box::new(|(row, _)| (3, (row, 0), Right)),
            Box::new(|(_, col)| (0, (0, col), Down)),
            Box::new(|(row, _)| (3, (row, (CUBE_FACE_SIZE as i32 -1)), Left)),
        ],
        // 5
        [
            Box::new(|(_, col)| (3, ((CUBE_FACE_SIZE as i32 -1), col), Up)),
            Box::new(|(row, _)| (5, (row, 0), Right)),
            Box::new(|(_, col)| (3, (0, col), Down)),
            Box::new(|(row, _)| (5, (row, (CUBE_FACE_SIZE as i32 -1)), Left)),
        ],
    ];
    let p2_transforms:[[Box<Transform>; 4];6] = [
        // 0
        [
            Box::new(|(_, col)| (5, (col, 0), Orientation::Right)),
            Box::new(|(row, _)| (1, (row, 0), Orientation::Right)),
            Box::new(|(_, col)| (2, (0, col), Orientation::Down)),
            Box::new(|(row, _)| (3, ((CUBE_FACE_SIZE as i32 -1) - row, 0), Orientation::Right)),
        ],
        // 1
        [
            Box::new(|(_, col)| (5, ((CUBE_FACE_SIZE as i32 -1), col), Orientation::Up)),
            Box::new(|(row, _)| (4, ((CUBE_FACE_SIZE as i32 -1) - row, (CUBE_FACE_SIZE as i32 -1)), Orientation::Left)),
            Box::new(|(_, col)| (2, (col, (CUBE_FACE_SIZE as i32 -1)), Orientation::Left)),
            Box::new(|(row, _)| (0, (row, (CUBE_FACE_SIZE as i32 -1)), Orientation::Left)),
        ],
        // 2
        [
            Box::new(|(_, col)| (0, ((CUBE_FACE_SIZE as i32 -1), col), Orientation::Up)),
            Box::new(|(row, _)| (1, ((CUBE_FACE_SIZE as i32 -1), row), Orientation::Up)),
            Box::new(|(_, col)| (4, (0, col), Orientation::Down)),
            Box::new(|(row, _)| (3, (0, row), Orientation::Down)),
        ],
        // 3
        [
            Box::new(|(_, col)| (2, (col, 0), Orientation::Right)),
            Box::new(|(row, _)| (4, (row, 0), Orientation::Right)),
            Box::new(|(_, col)| (5, (0, col), Orientation::Down)),
            Box::new(|(row, _)| (0, ((CUBE_FACE_SIZE as i32 -1) - row, 0), Orientation::Right)),
        ],
        [
        // 4
            Box::new(|(_, col)| (2, ((CUBE_FACE_SIZE as i32 -1), col), Orientation::Up)),
            Box::new(|(row, _)| (1, ((CUBE_FACE_SIZE as i32 -1) - row, (CUBE_FACE_SIZE as i32 -1)), Orientation::Left)), 
            Box::new(|(_, col)| (5, (col, (CUBE_FACE_SIZE as i32 -1)), Orientation::Left)),
            Box::new(|(row, _)| (3, (row, (CUBE_FACE_SIZE as i32 -1)), Orientation::Left)),
        ],
        // 5
        [
            Box::new(|(_, col)| (3, ((CUBE_FACE_SIZE as i32 -1), col), Orientation::Up)),
            Box::new(|(row, _)| (4, ((CUBE_FACE_SIZE as i32 -1), row), Orientation::Up)),
            Box::new(|(_, col)| (1, (0, col), Orientation::Down)),
            Box::new(|(row, _)| (0, (0, row), Orientation::Down)),
        ],
    ];
    let mut faces = spaces.into_iter()
        .zip(p1_transforms.into_iter())
        .zip(p2_transforms.into_iter())
        .map(|((spaces, p1_transforms), p2_transforms)| Face { spaces, p1_transforms, p2_transforms, map_offset: (0,0) });
    let mut cube = [
        faces.next().unwrap(),
        faces.next().unwrap(),
        faces.next().unwrap(),
        faces.next().unwrap(),
        faces.next().unwrap(),
        faces.next().unwrap(),
    ];

    cube[0].map_offset = (0, 50);
    cube[1].map_offset = (0, 100);
    cube[2].map_offset = (50, 50);
    cube[3].map_offset = (100, 00);
    cube[4].map_offset = (100, 50);
    cube[5].map_offset = (150, 0);

    cube
}

fn parse_directions(in_str: &str) -> Vec<Direction> {
    let mut out = vec!();
    let mut iter = in_str.chars().peekable();
    while let Some(c) = iter.next() {
        if c.is_ascii_digit() {
            let mut val = c.to_digit(10).unwrap();
            while iter.peek().is_some() && iter.peek().unwrap().is_ascii_digit() {
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
    let cube = parse_cube(map);
    let directions = parse_directions(raw_directions);
    let p1 = walk(&cube, &directions, true);
    let p2 = walk(&cube, &directions, false);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D22P1: {p1:?}");
    println!("D22P2: {p2:?}");
}

fn walk(cube: &Cube, directions: &[Direction], use_p1_mappings: bool) -> i32 {
    let mut face_index = 0;
    let mut position = (0,0);
    let mut orientation = Orientation::Right;
    let add_offset = |(row, col): (usize, usize), (dr, dc): (i32, i32)| (row as i32 + dr, col as i32 + dc);

    let get_transform_index = |orientation: Orientation| match orientation {
        Orientation::Up => 0,
        Orientation::Right => 1,
        Orientation::Down => 2,
        Orientation::Left => 3,
    };

    for direction in directions {
        match direction {
            Direction::Walk(v) => {
                for _ in 0..*v {
                    let (mut next_row, mut next_col) = add_offset(position, orientation.offset());
                    let mut next_face = face_index;
                    let mut next_orientation = orientation;

                    if !(0..CUBE_FACE_SIZE as i32).contains(&next_row) || !(0..CUBE_FACE_SIZE as i32).contains(&next_col) {
                        let transform_index = get_transform_index(orientation);
                        if use_p1_mappings {
                            (next_face, (next_row, next_col), next_orientation) = cube[face_index].p1_transforms[transform_index]((next_row, next_col));
                        } else {
                            (next_face, (next_row, next_col), next_orientation) = cube[face_index].p2_transforms[transform_index]((next_row, next_col));
                        }
                    }

                    if cube[next_face].spaces[next_row as usize][next_col as usize] == MapSpace::Wall {
                        break;
                    }
                    position = (next_row as usize, next_col as usize);
                    face_index = next_face;
                    orientation = next_orientation;
                }
            },
            Direction::Turn(dir) => orientation = orientation.turn(*dir),
        }
    }
    let (r, c) = cube[face_index].get_map_position(position);
    (1000 * (r + 1) + 4 * (c + 1)) as i32 + orientation as i32
}
