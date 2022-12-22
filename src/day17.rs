use std::fs;
use std::fmt;
use std::time::Instant;

use rustc_hash::{FxHashSet, FxHashMap};
use ringbuffer::{ConstGenericRingBuffer, RingBufferExt, RingBufferWrite, RingBuffer};


const FILE_PATH: &str = "inputs/day17_input.txt";
const CHAMBER_HEIGHT: usize = 64;
const P1_DROP_COUNT: u64 = 2022;
const P2_DROP_COUNT: u64 = 1_000_000_000_000;

#[derive(Debug)]
enum Piece {
    Dash,
    Plus,
    L,
    I,
    Square
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn from_char(ch: char) -> Self {
        match ch {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!(),
        }
    }
}

struct Chamber {
    rocks: ConstGenericRingBuffer<u8, CHAMBER_HEIGHT>,
    top: i32,
}

impl Chamber {
    fn new() -> Self {
        Self { rocks: ConstGenericRingBuffer::new(), top: 0 }
    }

    fn insert(&mut self, bytes: PieceCoords, offset: isize) {
        let mut i: i32 = 3;
        let mut comparison_line = offset;
        while i >= 0 && comparison_line < 0 {
            self.rocks[comparison_line] |= bytes[i as usize];
            i -= 1;
            comparison_line += 1;
        }

        while i >= 0 {
            if bytes[i as usize] != 0 {
                self.top += 1;
                self.rocks.push(bytes[i as usize]);
            } else {
                break;
            }
            i -= 1;
        }
    }
}

impl fmt::Display for Chamber {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 1..=std::cmp::min(self.rocks.len(), 10) {
            writeln!(f, "{:08b}", self.rocks.get(y as isize * -1).unwrap())?;
        }
        Ok(())
    }
}

#[derive(Default, Hash, Eq, PartialEq, Debug)]
struct Point(u32, u32);

type PieceCoords = [u8; 4];

impl Piece {
    fn get_bytes(&self) -> PieceCoords {
        use Piece::*;
        match self {
            // ###
            Dash => [
                0b_0000000_0,
                0b_0000000_0,
                0b_0000000_0,
                0b_0011110_0,
            ],

            //  # 
            // ###
            //  #
            Plus => [
                0b_0000000_0,
                0b_0001000_0,
                0b_0011100_0,
                0b_0001000_0,
            ],

            //   #
            //   #
            // ###
            L => [
                0b_0000000_0,
                0b_0000100_0,
                0b_0000100_0,
                0b_0011100_0,
            ],

            // #
            // #
            // #
            // #
            I => [
                0b_0010000_0,
                0b_0010000_0,
                0b_0010000_0,
                0b_0010000_0,
            ],

            // ##
            // ##
            Square => [
                0b_0000000_0,
                0b_0000000_0,
                0b_0011000_0,
                0b_0011000_0,
            ],
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day17");
    let jets = contents.trim().chars().map(Jet::from_char).enumerate().cycle();
    let (p1, p2) = part_one(jets);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D17P1: {p1:?}");
    println!("D17P1: {p2:?}");
}

fn check_push(bytes: PieceCoords, jet: Jet, chamber: &Chamber, offset: isize) -> bool {
    let mut out: PieceCoords = [0; 4];
    match jet {
        Jet::Left => {
            for (i, &byte) in bytes.iter().rev().enumerate() {
                if byte & 0b10000000 != 0{
                    return false;
                }
                out[i] = byte << 1;
            }
        },
        Jet::Right => {
            for (i, &byte) in bytes.iter().rev().enumerate() {
                if byte & 0b00000010 != 0 {
                    return false;
                }
                out[i] = byte >> 1;
            }
        },
    }

    if offset < 0 {
        let mut comparison_line = offset; 
        let mut i = 0;
        while i < 4 && comparison_line < 0 {
            if let Some(comp) = chamber.rocks.get(comparison_line) {
                if out[i as usize] & comp != 0 {
                    return false
                }
            } else {
                break
            }
            i += 1;
            comparison_line += 1;
        }
    }

    true
}

fn check_drop(bytes: PieceCoords, chamber: &Chamber, offset: isize) -> bool {
    // We are still hovering above the last roof
    if offset > 0 {
        return true
    }

    let mut i: i32 = 3;
    let mut comparison_line = (offset - 1) as isize;

    while i >= 0 && comparison_line < 0 {
        if let Some(comp) = chamber.rocks.get(comparison_line) {
            if bytes[i as usize] & comp != 0 {
                return false
            }
        } else {
            return false
        }
        i -= 1;
        comparison_line += 1;
    }

    // We have not pushed something this far into the buffer
    true
}

fn get_chamber_arr(chamber: &Chamber) -> [u8; CHAMBER_HEIGHT] {
    let mut out = [0; CHAMBER_HEIGHT];
    for i in 1..=CHAMBER_HEIGHT {
        out[i - 1] = *chamber.rocks.get(-1 * i as isize).unwrap();
    }

    out
}

fn part_one(mut jets: impl Iterator<Item = (usize, Jet)>) -> (i32, u64) {
    let mut dropped_count = 0;
    let mut chamber = Chamber::new();
    let mut cache:FxHashSet<(usize, usize, [u8; CHAMBER_HEIGHT])> = FxHashSet::default();
    let mut pieces = [
        Piece::Dash,
        Piece::Plus, 
        Piece::L, 
        Piece::I, 
        Piece::Square
    ].iter().enumerate().cycle();

    let mut offset_height = 0;
    let mut p1 = 0;
    let mut p2: u64 = 0;
    let (mut cycle_index, mut cycle_length) = (0, 0);
    let mut cycle_i = 0;
    let mut partial_cycle_heights = FxHashMap::default();

    'outer: loop {
        let (piece_index, piece) = pieces.next().unwrap();
        let mut new_piece = true;

        let (mut piece_bytes, mut offset) = (piece.get_bytes(), 3);

        loop {
            let (jet_index, jet) = jets.next().unwrap();
            if new_piece && dropped_count != 0 {
                let chamber_arr = get_chamber_arr(&chamber);
                if cache.contains(&(piece_index, jet_index, chamber_arr)) {
                    if cycle_index == 0 {
                        // How many pieces are dropped before the start of the first cycle
                        cycle_index = dropped_count;
                        offset_height = chamber.top;
                        cache.clear();
                        cache.insert((piece_index, jet_index, chamber_arr));
                    } else {
                        if cycle_length == 0 {
                            cycle_length = dropped_count - cycle_index;
                            let cycle_height = chamber.top - offset_height;
                            let cycle_gap = P2_DROP_COUNT - cycle_index as u64;
                            let full_cycles = cycle_gap / cycle_length;
                            let partial_cycle = cycle_gap % cycle_length;

                            p2 += offset_height as u64;
                            p2 += cycle_height as u64 * full_cycles;
                            p2 += partial_cycle_heights.get(&(partial_cycle)).unwrap();

                            if p1 != 0 { break 'outer };
                        }
                    }
                } else {
                    cache.insert((piece_index, jet_index, chamber_arr));
                }
            }
            new_piece = false;

            if check_push(piece_bytes, jet, &chamber, offset) {
                for byte in piece_bytes.iter_mut() {
                    match jet {
                        Jet::Right => *byte >>= 1,
                        Jet::Left => *byte <<= 1,
                    }
                }
            }

            if check_drop(piece_bytes, &chamber, offset) {
                offset -= 1;
            } else {
                chamber.insert(piece_bytes, offset);
                if offset_height != 0 {
                    cycle_i += 1;
                    partial_cycle_heights.insert(cycle_i, (chamber.top - offset_height) as u64);
                }
                break;
            }
        }
        dropped_count += 1;
        if dropped_count == P1_DROP_COUNT { 
            p1 = chamber.top;
            if p2 != 0 {
                break 'outer;
            }
        }
    }

    (p1, p2)
}
