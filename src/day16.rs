#![feature(iter_collect_into)]

use std::fs;
use std::time::Instant;
use std::cmp::max;
use std::collections::VecDeque;

use bitmaps::Bitmap;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use regex::Regex;
use lazy_static::lazy_static;

const FILE_PATH: &str = "inputs/day16_input.txt";
const START_VALVE: &str = "AA";
lazy_static! {
    static ref VALVE_REGEX: Regex = Regex::new(r"Valve (?P<valve_name>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<paths>([A-Z]{2}(, )?)+)")
        .unwrap();
}

struct Valves<'a> {
    flow_rates: FxHashMap<&'a str, i32>,
    hops: FxHashMap<(&'a str, &'a str), i32>,
}

impl<'a> Valves<'a> {
    fn new(valves: impl Iterator<Item = (&'a str, i32, Vec<&'a str>)>) -> Self {
        let mut flow_rates = FxHashMap::default(); 
        let mut adjacencies = FxHashMap::default(); 
        let mut hops = FxHashMap::default();

        valves.for_each(|(n, f, adj)| {
            flow_rates.insert(n, f);
            adjacencies.insert(n, adj);
        });


        for &v1 in adjacencies.keys() {
            'inner : for &v2 in adjacencies.keys() {
                if hops.contains_key(&(v1, v2)) {
                    continue;
                }
                let mut queue = VecDeque::new();
                let mut visited = FxHashSet::default();
                queue.push_back((0, v1));
                visited.insert(v1);
                while let Some((n, valve)) = queue.pop_front() {
                    for &v in adjacencies.get(valve).unwrap() {
                        if visited.contains(v) {
                            continue;
                        }
                        hops.insert((v1, v), n + 1);
                        hops.insert((v, v1), n + 1);
                        if v == v2 {
                            continue 'inner;
                        }
                        queue.push_back((n + 1, v));
                        visited.insert(v);
                    }
                }
            }
        }



        Valves { 
            flow_rates,
            hops,
        }
    }

    fn search(&self, v1: &'a str, v2: &'a str) -> i32 {
        if self.hops.contains_key(&(v1, v2)) {
            return self.hops[&(v1, v2)]
        }
        panic!()
    }
}

fn parse_line<'a>(in_str: &'a str) -> (&'a str, i32, Vec<&'a str>) {
    let captures = VALVE_REGEX.captures(in_str).unwrap();
    let name = &in_str[captures.name("valve_name").unwrap().range()];
    let flow_rate = in_str[captures.name("flow_rate").unwrap().range()].parse().unwrap();
    let paths = in_str[captures.name("paths").unwrap().range()].split(", ").collect();

    (name, flow_rate, paths)
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day16");
    let mut valves = Valves::new(contents.lines().map(parse_line)); 
    let relevant_valves: Vec<&str> = valves.flow_rates
        .iter()
        .filter_map(|(&n, &f)| 
            if f > 0 { Some(n) } else { None }
        )
        .collect();

    let p1 = part_one(&mut valves, &relevant_valves);
    let p2 = part_two(&mut valves, &relevant_valves);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D16P1: {p1:?}");
    println!("D16P2: {p2:?}");
}

fn part_one<'a>(valves: &mut Valves<'a>, relevant_valves: &Vec<&'a str>) -> i32 {
    compute(valves, relevant_valves, 30, Bitmap::new())
}

fn part_two<'a> (valves: &mut Valves<'a>, relevant_valves: &Vec<&'a str>) -> i32 {
    (1..usize::pow(2, (relevant_valves.len()) as u32) / 2).into_par_iter()
        .map(|mask| {
            if mask.count_ones() < 7 || mask.count_ones() > 8 {
                 return 0
            }
            let mut mask = Bitmap::from_value(mask as u16);
            let r1 = compute(valves, relevant_valves, 26, mask);
            mask.invert();
            let r2 = compute(valves, relevant_valves, 26, mask);
            r1 + r2
        })
        .reduce(|| i32::MIN, max)
        //.reduce(max).unwrap()
}

fn compute<'a>(valves: &Valves<'a>, relevant_valves: &Vec<&'a str>, starting_time: i32, starting_state: Bitmap<16>) -> i32 {
    type Memo<'a> = FxHashMap<(i32, &'a str, Bitmap<16>), i32>;
    let mut memo: Memo = FxHashMap::default();

    fn helper<'a>(time_remaining: i32, current_valve: &'a str, all_valves: &Valves<'a>, relevant_valves: &[&'a str], mut states: Bitmap<16>, memo: &mut Memo<'a>) -> i32 {
        let memo_key = (time_remaining, current_valve, states.clone());
        if memo.contains_key(&memo_key) {
            return *memo.get(&memo_key).unwrap();
        }
        let mask = Bitmap::mask(relevant_valves.len());
        if time_remaining <= 0 || (states & mask) == mask {
            // All valves are open, there is nothing more we can do
            memo.insert(memo_key, 0);
            return 0;
        }

        let current_position = relevant_valves.iter().position(|&v| v == current_valve);
        let should_consider_current = current_position.is_some() && !states.get(current_position.unwrap());

        let time_delta_with_flipping =  if should_consider_current {
            let current_position = current_position.unwrap();
            states.set(current_position, true);
            let current_flow_delta = (time_remaining - 1) * all_valves.flow_rates.get(current_valve).unwrap();

            let max_moving_on = (0..relevant_valves.len())
                .filter(|&i| !states.get(i))
                .map(|i| {
                    let n = relevant_valves[i];
                    let t_delta = 1 + all_valves.search(current_valve, n);
                    helper(time_remaining - t_delta, n, all_valves, relevant_valves, states.clone(), memo)
                })
                .reduce(max)
                .unwrap_or_else(||0);

            states.set(current_position, false);

            current_flow_delta + max_moving_on
        } else {
            0
        };

        let time_delta_without_flipping = {
            (0..relevant_valves.len())
                .filter(|&i| !states.get(i) && relevant_valves[i] != current_valve)
                .map(|i| {
                    let n = relevant_valves[i];
                    let t_delta = all_valves.search(current_valve, n);
                    helper(time_remaining - t_delta, n, all_valves, relevant_valves, states.clone(), memo)
                })
                .reduce(max)
                .unwrap_or_else(||0)
        };

        let m = max(time_delta_with_flipping, time_delta_without_flipping);
        memo.insert(memo_key, m);
        m
    }

    helper(starting_time, START_VALVE, valves, &relevant_valves, starting_state, &mut memo)
}
