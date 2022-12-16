use std::fs;
use std::fmt::{Debug, Formatter, Result};
use std::time::Instant;
use std::cmp::Ordering;

const FILE_PATH: &str = "inputs/day13_input.txt";

#[derive(Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

impl<'a> Debug for Packet {
    fn fmt(&self, formatter: &mut Formatter<'_> ) -> Result {
        match self {
            Packet::List(ls) => {
                formatter.write_str("[")?;
                for i in 0..ls.len() {
                    formatter.write_fmt(format_args!("{:?}", ls.get(i).unwrap()))?;
                    if i != ls.len() - 1 {
                        formatter.write_str(", ").unwrap();
                    }
                }
                formatter.write_str("]")?;
            },
            Packet::Number(n) => {
                formatter.write_fmt(format_args!("{:?}", n))?;
            }

        }
        Ok(())
    }
}

fn parse_line(line: &str) -> Packet {
    // Assume that the first character is an open brace and start by parsing the second character
    let mut i = 1;
    let mut packet_vec = vec!();
    // Assume that the last character is a closing brace and don't parse it
    while i < line.len() - 1 {
        match &line[i..=i] {
            "[" => {
                let mut nesting_level = 1;
                let mut j = i + 1;
                while nesting_level > 0 {
                    match &line[j..=j] {
                        "[" => {
                            nesting_level += 1;
                            j += 1;
                        },
                        "]" => {
                            nesting_level -= 1;
                            j += 1;
                        },
                        _ => {
                            j += 1
                        }
                    }
                }
                packet_vec.push(parse_line(&line[i..j]));
                i = j + 1;
            },
            _ => {
                let mut j = i;
                for c in line[i..line.len()].chars() {
                    j += 1;
                    if !c.is_numeric() {
                        break;
                    }
                }
                packet_vec.push(Packet::Number(line[i..j - 1].parse().unwrap()));
                i = j;
            }
        }
    }
    Packet::List(packet_vec)
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day13");
    let mut parsed: Vec<Packet> = contents.trim().lines().filter(|&l| !l.is_empty()).map(parse_line).collect();
    let p1 = part_one(&parsed[..]);
    parsed.push(Packet::List(vec!(Packet::Number(2))));
    parsed.push(Packet::List(vec!(Packet::Number(6))));
    let p2 = part_two(&mut parsed[..]);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D13P1: {p1:?}");
    println!("D13P2: {p2:?}");
}

fn compare_packets(left: &Packet, right: &Packet) -> Ordering {
    let left = if let Packet::List(ls1) = left { ls1 } else { unreachable!() };
    let right = if let Packet::List(ls2) = right { ls2 } else { unreachable!() };
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        let left = left.get(i).unwrap();
        let right = right.get(j).unwrap();
        if let Packet::List(_) = left{
            if let Packet::List(_) = right {
                let res = compare_packets(left, right);
                match res {
                    Ordering::Equal => {
                        i += 1;
                        j += 1;
                    },
                    _ => return res,
                }
            } else {
                let r_val = if let Packet::Number(p) = right { p } else { unreachable!() };
                let res = compare_packets(left, &Packet::List(vec!(Packet::Number(*r_val))));
                match res {
                    Ordering::Equal => {
                        i += 1;
                        j += 1;
                    },
                    _ => return res,
                }
            }
        } else {
            if let Packet::List(_) = right {
                let l_val = if let Packet::Number(p) = left { p } else { unreachable!() };
                let res = compare_packets(&Packet::List(vec!(Packet::Number(*l_val))), right);
                match res {
                    Ordering::Equal => {
                        i += 1;
                        j += 1;
                    },
                    _ => return res,
                }
            } else {
                let l_val = if let Packet::Number(l) = left { l } else { unreachable!() };
                let r_val = if let Packet::Number(r) = right { r } else { unreachable!() };
                // println!("Comparing {l_val:?} and {r_val:?}");
                let res = l_val.cmp(r_val);
                match res {
                    Ordering::Equal => {
                        i += 1;
                        j += 1;
                    },
                    _ => return res,
                }
            }
        }
    }
    if i >= left.len() && j >= right.len() {
        return Ordering::Equal;
    }
    if i >= left.len() {
        return Ordering::Less;
    }
    return Ordering::Greater

}

fn part_two(packet_pairs: &mut [Packet]) -> i32 {
    packet_pairs.sort_by(compare_packets);
    let (mut i1, mut i2) = (0, 0);
    for (i, p) in packet_pairs.iter().enumerate() {
        if let Packet::List(ls) = p {
            if ls.len() == 1 {
                if let Packet::Number(val) = ls.get(0).unwrap() {
                    if *val == 6 {
                        i1 = i + 1;
                    } else if *val == 2 {
                        i2 = i + 1;
                    }
                }
            }
        }
    }
    (i1 * i2) as i32
}

fn part_one(packet_pairs: &[Packet]) -> i32 {
    packet_pairs
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| (i, compare_packets(&chunk[0], &chunk[1])))
        .filter(|(_, o)| *o == Ordering::Less)
        .map(|(i, _)| (i + 1) as i32)
        .sum()
}
