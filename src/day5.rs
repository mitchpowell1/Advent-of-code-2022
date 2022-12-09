#![feature(iter_collect_into)]

use std::fs;
use std::time::Instant;


const FILE_PATH: &str = "inputs/day5_input.txt";
const VALUE_OFFSET: usize = 4;

#[derive(Debug)]
struct Instruction {
    from_stack: usize,
    to_stack: usize,
    count: usize,
}

type Stacks = Vec<Vec<char>>;

fn parse_stacks(raw_stacks: &str) -> Stacks {
    let mut stack_lines = raw_stacks.lines().rev();
    let stack_count = stack_lines.next().unwrap()
        .split_whitespace()
        .filter(|l| !l.trim().is_empty())
        .count();

    let mut stacks: Stacks = vec![vec!(); stack_count];
    for line in stack_lines {
        for (i, c) in line.chars().skip(1).step_by(VALUE_OFFSET).enumerate() {
            if c.is_alphabetic() {
                stacks[i].push(c);
            }
        }
    }
    stacks
}

fn parse_instructions(raw_instructions: &str) -> impl Iterator<Item = Instruction> + '_ + Clone{
    raw_instructions
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace().skip(1).step_by(2);
            let count = iter.next().unwrap().parse().unwrap();
            let from_stack: usize = iter.next().unwrap().parse().unwrap();
            let to_stack: usize = iter.next().unwrap().parse().unwrap();

            Instruction { from_stack: from_stack - 1, to_stack: to_stack - 1, count }
        })
}

fn part_one(instructions: impl Iterator<Item = Instruction>, stacks: &mut Stacks) {
    for Instruction { from_stack, to_stack, count } in instructions {
        for _ in 0..count {
            let v = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(v);
        }
    }
}

fn part_two(instructions: impl Iterator<Item = Instruction>, stacks: &mut Stacks) {
    let mut vals: Vec<char> = vec!();
    for Instruction { from_stack, to_stack, count } in instructions {
        vals.clear();
        let from = stacks.get_mut(from_stack).unwrap();
        from.drain(from.len() - count..from.len()).collect_into(&mut vals);
        let to = stacks.get_mut(to_stack).unwrap();
        for v in &vals {
            to.push(*v);
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day5");
    let mut contents_parts = contents.split("\n\n");
    let mut p1_stacks = parse_stacks(contents_parts.next().unwrap());
    let mut p2_stacks = p1_stacks.clone();
    let instructions = parse_instructions(contents_parts.next().unwrap());
    part_one(instructions.clone(), &mut p1_stacks);
    part_two(instructions, &mut p2_stacks);
    let p1: String = p1_stacks.iter().map(|s| s[s.len() - 1]).collect();
    let p2: String = p2_stacks.iter().map(|s| s[s.len() - 1]).collect();
    println!("Elapsed: {:?}", start.elapsed());
    println!("D5P1: {p1:?}");
    println!("D5P2: {p2:?}");
}
