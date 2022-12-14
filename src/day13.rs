use std::fs;

const FILE_PATH: &str = "inputs/day13_input_test.txt";

enum Value {
    List(Vec<Value>),
    Number(i32),
}

// fn parse_line(line: &str) -> Value {
//     let packet_stack = vec!();
//     let mut i = 0
//     for i in 0..line.len() {
//         match &line[i..i+1]
//         if &line[i..i+1]
//     }
//     match &line[0..1] {
//         "[" => {
//             let mut nested_depth = 0;
//             let mut values = vec!();
//             for i in 1..line.len() {
//                 if &line[i..i+1] == "]" {
//                     nested_depth -= 1;
//                     if nested_depth == 0 {
//                         return Value::List(parse_line(&line[1..i]));
//                     }
//                 }
//             }
//             Value::Number(0)
//         },
//         _ => Value::Number(line.parse().unwrap())
//     }
// }

// fn parse_input(input: &str) {
//     let out = input
//         .split_whitespace()
//         .map(parse_line);
// }

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day13");
    let parsed = contents.trim().split("\n\n");
    // let p1 =  part_one(parsed);
    // println!("D13P1: {p1:?}");
}

// fn compare_pairs(left: &str, right: &str) -> bool {
//     println!("Left: {left:?}");
//     println!("Right: {right:?}");
//     let (mut left_chars, mut right_chars) = (left.chars(), right.chars());
//     let (mut left_depth, mut right_depth) = (0,0);
//     false
// }
//
// fn part_one<'a>(packet_pairs: impl Iterator<Item = &'a str>) -> i32 {
//     for pair in packet_pairs {
//         let (p1, p2) = pair.split_once("\n").unwrap();
//         compare_pairs(p1, p2);
//     }
//     0
// }
