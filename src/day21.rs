use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use regex::Regex;

const FILE_PATH: &str = "inputs/day21_input.txt";
const ROOT_NAME: &str = "root";
const HUMAN_NAME: &str = "humn";
lazy_static! {
    static ref MONKEY_REGEX: Regex = Regex::new(r"(?P<name>[a-z]+): ((?P<equation>(?P<lhs>[a-z]+) (?P<operator>\+|\-|\*|/) (?P<rhs>[a-z]+))|(?P<number>\d+))").unwrap();
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Operator {
    fn from_str(in_str: &str) -> Self {
        match in_str {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "/" => Operator::Divide,
            "*" => Operator::Multiply,
            _ => unreachable!(),
        }
    }

    fn evaluate(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Divide => lhs / rhs,
            Operator::Multiply => lhs * rhs,
        }
    }

    fn inverse(&self) -> Self {
        use Operator::*;
        match self {
            Add => Subtract,
            Subtract => Add,
            Divide => Multiply,
            Multiply => Divide,
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Operator::Add => write!(f, "+")?,
            Operator::Subtract => write!(f, "-")?,
            Operator::Multiply => write!(f, "*")?,
            Operator::Divide => write!(f, "/")?,
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Number(i64),
    Equation(&'a str, Operator, &'a str),
}

impl<'a> Monkey<'a> {
    fn from_str(in_str: &'a str) -> Self {
        let captures = MONKEY_REGEX.captures(in_str).unwrap();
        if let Some(num) = captures.name("number") {
            return Monkey::Number(in_str[num.range()].parse().unwrap());
        }

        let lhs = &in_str[captures.name("lhs").unwrap().range()];
        let rhs = &in_str[captures.name("rhs").unwrap().range()];
        let operator = Operator::from_str(&in_str[captures.name("operator").unwrap().range()]);
        Monkey::Equation(lhs, operator, rhs)
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day21");
    let mut monkeys = FxHashMap::default();
    for line in contents.lines() {
        let (monkey_name, _) = line.split_once(':').unwrap();
        let monkey = Monkey::from_str(line);
        monkeys.insert(monkey_name, monkey);
    }
    let p1 = part_one(&monkeys);
    let p2 = part_two(&monkeys);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D21P1: {p1:?}");
    println!("D21P2: {p2:?}");
}

fn get_evaluations<'a>(monkeys: &FxHashMap<&'a str, Monkey>, evaluations_to_skip: &[&'a str]) -> FxHashMap<&'a str, i64> {
    let mut evaluated: FxHashMap<&str, i64> = monkeys
        .iter()
        .filter_map(|(&name, monkey)| {
            if evaluations_to_skip.contains(&name) {
                return None;
            }
            match monkey {
                Monkey::Number(num) => Some((name, *num)),
                _ => None,
            }
        })
        .collect();
    
    loop {
        let mut evaluations = 0;
        for (name, monkey) in monkeys {
            if evaluations_to_skip.contains(name) || evaluated.contains_key(name) {
                continue;
            }

            if let Monkey::Equation(lhs, op, rhs) = monkey {
                if !(evaluated.contains_key(lhs) && evaluated.contains_key(rhs)) {
                    continue
                }
                let left = evaluated.get(lhs).unwrap();
                let right = evaluated.get(rhs).unwrap();
                evaluated.insert(name, op.evaluate(*left, *right));
                evaluations += 1;
            }
        }
        if evaluations == 0 {
            break;
        }
    }

    evaluated
}

fn part_one(monkeys: &FxHashMap<&str, Monkey>) -> i64 {
    let evaluated = get_evaluations(monkeys, &[]);
    *evaluated.get(ROOT_NAME).unwrap()
}

fn part_two(monkeys: &FxHashMap<&str, Monkey>) -> i64 {
    let evaluated = get_evaluations(monkeys, &[ROOT_NAME, HUMAN_NAME]);

    let (lhs, rhs) = if let Monkey::Equation(lhs, _, rhs) = monkeys.get(ROOT_NAME).unwrap() { (lhs, rhs) } else { unreachable!() };
    let working_value = if let Some(value) = evaluated.get(lhs) { *value } else { *evaluated.get(rhs).unwrap() } as i64 ;
    let start = if evaluated.contains_key(lhs) { rhs } else { lhs }; 
    fn helper(node: &str, working_value: i64, monkeys: &FxHashMap<&str, Monkey>, evaluated: &FxHashMap<&str, i64>) -> i64 {
        if node == HUMAN_NAME {
            return working_value;
        }
        let (lhs, op, rhs) = if let Monkey::Equation(lhs, op, rhs) = monkeys.get(node).unwrap() { (*lhs, *op, *rhs) } else { unreachable!() };
        if let Some(val) = evaluated.get(lhs) {
            if op == Operator::Subtract {
                return helper(rhs, -1 * (working_value - *val), monkeys, evaluated);
            }
            if op == Operator::Divide {
                return helper(rhs, working_value / val, monkeys, evaluated);
            }
            return helper(rhs, op.inverse().evaluate(working_value, *val), monkeys, evaluated);
        }
        let val = evaluated.get(rhs).unwrap();
        return  helper(lhs, op.inverse().evaluate(working_value, *val), monkeys, evaluated);
    }

    helper(start, working_value, monkeys, &evaluated)
}
