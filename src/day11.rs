use std::fs;
use std::collections::VecDeque;
use std::time::Instant;

use num::integer::lcm;

const FILE_PATH: &str = "inputs/day11_input.txt";
const P1_CYCLE_COUNT: usize = 20;
const P2_CYCLE_COUNT: usize = 10_000;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    inspection_count: usize,
    divisor: u64,
    true_index: usize,
    false_index: usize,
    operand: Option<u64>,
    operator: Operator,
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Mult,
}

fn get_operation(param_str: &str) -> (Operator, Option<u64>) {
    let mut parameters = param_str.split_whitespace().skip(1);

    let operator = match parameters.next().unwrap() {
        "*" => Operator::Mult,
        "+" => Operator::Add,
        _ => panic!()
    };

    let raw_operand = parameters.next().unwrap();
    let operand = match raw_operand{
        "old" => None,
        _ => Some(raw_operand.parse().unwrap()),
    };

    (operator, operand)
}

fn get_test<'a>(mut lines: impl Iterator<Item = &'a str>) -> (u64, usize, usize) {
    let divisor: u64 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let true_monkey: usize = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let false_monkey: usize = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();

    (divisor, true_monkey, false_monkey)
}
impl Monkey {
    fn operation(&self, v: u64) -> u64 {
        let operand = if self.operand.is_none() { v } else { self.operand.unwrap() };
        match self.operator {
            Operator::Add => v + operand,
            Operator::Mult => v * operand
        }
    }

    fn cycle(&mut self, worry_divisor: u64) -> Option<(u64, usize)> {
        if let Some(v) = self.items.pop_front() {
            self.inspection_count += 1;
            let v = self.operation(v) / worry_divisor;
            let idx = if v % self.divisor == 0 { self.true_index } else { self.false_index };
            return Some((v, idx))
        };
        None
    }

    fn from_str(raw_monkey: &str) -> Self {
        let mut lines = raw_monkey.lines().skip(1);

        let (_, items) = lines.next().unwrap().split_once(':').unwrap();
        let items: VecDeque<u64> = items.split(',').map(|i| i.trim().parse().unwrap()).collect();

        let (_, raw_op) = lines.next().unwrap().split_once('=').unwrap();
        let (operator, operand) = get_operation(raw_op);

        let (divisor, true_index, false_index) = get_test(lines);

        Monkey { items, operator, operand, divisor, true_index, false_index, inspection_count: 0 }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day11");
    let mut monkeys: Vec<Monkey> = contents.trim().split("\n\n").map(Monkey::from_str).collect();
    let mut monkeys2: Vec<Monkey> = monkeys.iter().map(Monkey::clone).collect();
    let worry_mod = monkeys.iter().map(|m| m.divisor).reduce(lcm).unwrap();
    let p1 = compute(&mut monkeys, 3, P1_CYCLE_COUNT, worry_mod);
    let p2 = compute(&mut monkeys2, 1, P2_CYCLE_COUNT, worry_mod);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D11P1: {p1:?}");
    println!("D11P2: {p2:?}");
}

fn compute(monkeys: &mut [Monkey], divisor: u64, cycles: usize, worry_mod: u64) -> usize {
    for _ in 0..cycles {
        for i in 0..monkeys.len() {
            while let Some((value, index)) = monkeys[i].cycle(divisor) {
                monkeys[index].items.push_back(value % worry_mod);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));
    return monkeys[0].inspection_count * monkeys[1].inspection_count;
}
