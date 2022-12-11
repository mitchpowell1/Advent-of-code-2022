#![feature(slice_concat_trait)]

use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day10_input.txt";
const PIXEL_WIDTH: usize = 40;
const CYCLE_COUNT: usize = 240;

#[derive(Debug)]
enum Instruction { Noop, Addx(i32) }

impl Instruction {
    fn from_str(in_str: &str) -> Self {
        match in_str {
            "noop" => Instruction::Noop,
            _ => {
                Instruction::Addx(in_str.split_whitespace().nth(1).unwrap().parse().unwrap())
            }
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day10");
    let parsed = contents.trim().lines().map(Instruction::from_str);
    let (p1, p2) = process_instructions(parsed);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D10P1: {p1:?}");
    println!("D10P2: {p2}");
}

fn process_instructions(instructions: impl Iterator<Item = Instruction>) -> (i32, String) {
    let mut cycle = 0;
    let mut x_reg: i32 = 1;
    let mut signal_sum = 0;
    let mut pixel_output = [' '; CYCLE_COUNT];

    let mut consume_cycles = |count: i32, add: i32| {
        for _ in 0..count {
            cycle += 1;
            let mod_cycle = (cycle - 1) % PIXEL_WIDTH as i32;
            if mod_cycle == x_reg || mod_cycle == x_reg + 1 || mod_cycle == x_reg - 1 {
                pixel_output[cycle as usize - 1] = '#'
            }
            if (cycle - 20) % 40 == 0 {
                signal_sum += cycle * x_reg;
            }
        }
        x_reg += add;
    };

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                consume_cycles(1, 0);
            },
            Instruction::Addx(v) => { 
                consume_cycles(2, v);
            }
        }
    }


    let mut str = String::new();
    for (i, c) in pixel_output.iter().enumerate() {
        if  i % PIXEL_WIDTH == 0 {
            str.push('\n');
        }
        str.push(*c);
    }

    (signal_sum, str)
}
