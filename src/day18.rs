use std::fs;
use std::cmp::{min, max};
use std::collections::VecDeque;
use std::time::Instant;

use rustc_hash::FxHashSet;

const FILE_PATH: &str = "inputs/day18_input.txt";
const OFFSETS: [(i32, i32, i32); 6] = [
    (-1,  0, 0),
    ( 1,  0, 0),

    ( 0, -1, 0),
    ( 0,  1, 0),

    ( 0, 0, -1),
    ( 0, 0,  1),

];

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Cube(i32, i32, i32);

impl Cube {
    fn from_str(input: &str) -> Self {
        let mut split = input.split(',');
        Cube (
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        )
    }

    fn add_offset(&self, offset: (i32,i32,i32)) -> Self {
        Cube(
            self.0 + offset.0,
            self.1 + offset.1,
            self.2 + offset.2,
        )
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day18");
    let cubes: FxHashSet<Cube> = contents.lines().map(Cube::from_str).collect();
    let p1 = part_one(&cubes);
    let p2 = p1 - part_two(&cubes);

    println!("Elapsed: {:?}", start.elapsed());
    println!("D18P1: {p1:?}");
    println!("D18P2: {p2:?}");
}

fn part_one(cubes: &FxHashSet<Cube>) -> usize {
    let mut surface_area = cubes.len() * 6;

    for cube in cubes {
        for offset in OFFSETS {
            let new_cube = cube.add_offset(offset);
            if cubes.contains(&new_cube) {
                surface_area -= 1;
            }
        }
    }

    surface_area
}

fn droplet_bounds(cubes: &FxHashSet<Cube>) -> [(i32, i32); 3]{
    let (mut x_bounds, mut y_bounds, mut z_bounds) = ((i32::MAX,i32::MIN), (i32::MAX,i32::MIN), (i32::MAX,i32::MIN));
    for Cube(x,y,z) in cubes {
        x_bounds.0 = min(x_bounds.0, *x);
        x_bounds.1 = max(x_bounds.1, *x);

        y_bounds.0 = min(y_bounds.0, *y);
        y_bounds.1 = max(y_bounds.1, *y);

        z_bounds.0 = min(z_bounds.0, *z);
        z_bounds.1 = max(z_bounds.1, *z);
    }

    [x_bounds, y_bounds, z_bounds]
}

fn in_bounds(Cube(x, y, z): Cube, [x_bounds, y_bounds, z_bounds]: [(i32,i32); 3]) -> bool {
    (x >= x_bounds.0 && x <= x_bounds.1) &&
    (y >= y_bounds.0 && y <= y_bounds.1) &&
    (z >= z_bounds.0 && z <= z_bounds.1)
}

fn explore_space(space_cube: Cube, droplet_bounds: [(i32,i32); 3], cubes: &FxHashSet<Cube>) -> Option<FxHashSet<Cube>> {
    let mut visited = FxHashSet::default();
    let mut to_visit = VecDeque::new();
    visited.insert(space_cube);
    to_visit.push_back(space_cube);
    while let Some(space) = to_visit.pop_front() {
        if !in_bounds(space, droplet_bounds) {
            return None
        }

        for offset in OFFSETS {
            let next = space.add_offset(offset);
            if !visited.contains(&next) && ! cubes.contains(&next){
                to_visit.push_back(next);
                visited.insert(next);
            }
        }
    }
    Some(visited)
}

fn part_two(cubes: &FxHashSet<Cube>) -> usize {
    let mut internal_spaces: FxHashSet<Cube> = FxHashSet::default();
    let droplet_bounds = droplet_bounds(&cubes);

    for cube in cubes {
        for offset in OFFSETS {
            let new_cube = cube.add_offset(offset);
            if cubes.contains(&new_cube) || internal_spaces.contains(&new_cube) || !in_bounds(new_cube, droplet_bounds) {
                continue;
            }
            if let Some(pocket) = explore_space(new_cube, droplet_bounds, cubes) {
                pocket.into_iter().for_each(|space| { internal_spaces.insert(space); });
            }
        }
    }

   part_one(&internal_spaces)
}

