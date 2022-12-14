use std::fs;
use std::cmp::{min, max};
use std::time::Instant;

const FILE_PATH: &str = "inputs/day14_input.txt";
const SAND_ORIGIN: (u32, u32) = (500, 0);
const CAVE_WIDTH: usize = 500;
const CAVE_HEIGHT: usize = 200;
const X_OFFSET: usize = 250;


#[derive(Debug)]
struct Cave {
    bottom: u32,
    occupied: [bool; CAVE_HEIGHT * CAVE_WIDTH],
    drop_cache: Vec<(u32,u32)>,
}

fn get_flat_index(x: u32, y: u32) -> usize{
    y as usize * CAVE_WIDTH + (x as usize - X_OFFSET) % CAVE_WIDTH
}

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Cave {
    let mut out_cave = Cave { 
        bottom: 0,
        occupied: [false; CAVE_HEIGHT * CAVE_WIDTH],
        drop_cache: vec!(),
    };
    for line in lines {
        let mut points = line.split(" -> ").map(|p|{
            let (x, y) = p.split_once(',').unwrap();
            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        });
        let mut last = points.next().unwrap();
        out_cave.bottom = max(out_cave.bottom, last.1);
        while let Some(p) = points.next() {
            let (x1, y1) = last;
            let (x2, y2) = p;

            out_cave.bottom = max(out_cave.bottom, y2);

            for y in min(y1,y2)..=max(y1,y2) {
                out_cave.occupied[get_flat_index(x1, y)] = true;
            }

            for x in min(x1, x2)..=max(x1,x2) {
                out_cave.occupied[get_flat_index(x, y1)] = true;
            }

            last = p;
        }
    }

    out_cave
}

impl Cave {
    fn is_occupied(&self, x: u32, y: u32) -> bool{
        self.occupied[get_flat_index(x, y)]
    }

    fn drop(&mut self, x:u32, y:u32) -> Result<(u32, u32), ()> {
        let (mut x1, mut y1) = (x, y);
        while y1 < self.bottom {
            if !self.is_occupied(x1, y1 + 1) {
                y1 += 1;
                self.drop_cache.push((x1,y1));
                continue;
            } 
            if !self.is_occupied(x1 - 1, y1 + 1) {
                x1 -= 1;
                y1 += 1;
                self.drop_cache.push((x1,y1));
                continue;
            }
            if !self.is_occupied(x1 + 1, y1 + 1) {
                x1 += 1;
                y1 += 1;
                self.drop_cache.push((x1,y1));
                continue;
            }
            self.occupied[get_flat_index(x1, y1)] = true;
            self.drop_cache.pop();
            return Ok((x1,y1));
        }
        Err(())
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day14");
    let mut cave = parse_input(contents.lines());

    let p1 = part_one(&mut cave);
    let p2 = part_two(&mut cave) + p1;
    println!("Elapsed: {:?}", start.elapsed());
    println!("D14P1: {p1:?}");
    println!("D14P2: {p2:?}");
}

fn part_one(cave: &mut Cave) -> u32 {
    let mut settled_count = 0;
    let  (mut x, mut y) = SAND_ORIGIN;
    while let Ok(_) = cave.drop(x, y) {
        (x,y) = if let Some(cache_drop) = cave.drop_cache.pop() { 
            cache_drop
        } else { 
            SAND_ORIGIN 
        };
        settled_count += 1;
    }
    settled_count
}

fn part_two(cave: &mut Cave) -> u32 {
    let mut settled_count = 0;
    let last_row_index = CAVE_WIDTH * ((cave.bottom + 2) as usize) - 1;
    cave.occupied[last_row_index..last_row_index + CAVE_WIDTH].iter_mut().for_each(|x| *x = true);
    cave.bottom = cave.bottom + 2;
    let res = cave.drop_cache.pop();
    let (mut x, mut y) = if let Some(drop) = res { drop } else { SAND_ORIGIN };
    while let Ok(settled) = cave.drop(x, y) {
        settled_count += 1;
        (x,y) = if let Some(cache_drop) = cave.drop_cache.pop() { 
            cache_drop 
        } else { 
            SAND_ORIGIN 
        };
        if settled == SAND_ORIGIN {
            break;
        }
    }
    settled_count
}
