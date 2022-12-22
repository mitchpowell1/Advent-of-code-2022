#![feature(int_roundings)]

use std::fs;
use std::cmp::{max, min};
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use rayon::prelude::*;

const FILE_PATH: &str = "inputs/day19_input.txt";
lazy_static! {
    static ref BP_RE: Regex = Regex::new(
        concat!(r"Blueprint (?P<id>\d+): ",
                r"Each ore robot costs (?P<ore_cost>\d+) ore. ",
                r"Each clay robot costs (?P<clay_cost>\d+) ore. ",
                r"Each obsidian robot costs (?P<obs_cost_1>\d+) ore and (?P<obs_cost_2>\d+) clay. ",
                r"Each geode robot costs (?P<geode_cost_1>\d+) ore and (?P<geode_cost_2>\d+) obsidian."
        )
    ).unwrap();
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    // ore
    ore_robot_cost: i32,
    // ore
    clay_robot_cost: i32,
    // ore, clay
    obsidian_robot_cost: (i32, i32),
    // ore, obsidian
    geode_robot_cost: (i32, i32),

    max_needed_ore: i32,
    max_needed_clay: i32,
    max_needed_obsidian: i32,
}

impl Blueprint {
    fn from_str(in_str: &str) -> Self {
        let captures = BP_RE.captures(in_str).unwrap();
        let id =  in_str[captures.name("id").unwrap().range()].parse().unwrap();
        let ore_robot_cost = in_str[captures.name("ore_cost").unwrap().range()].parse().unwrap();
        let clay_robot_cost = in_str[captures.name("clay_cost").unwrap().range()].parse().unwrap();
        let obsidian_robot_cost = (
            in_str[captures.name("obs_cost_1").unwrap().range()].parse().unwrap(),
            in_str[captures.name("obs_cost_2").unwrap().range()].parse().unwrap(),
        );
        let geode_robot_cost = (
            in_str[captures.name("geode_cost_1").unwrap().range()].parse().unwrap(),
            in_str[captures.name("geode_cost_2").unwrap().range()].parse().unwrap(),
        );
        let max_needed_ore = *[ore_robot_cost, clay_robot_cost, obsidian_robot_cost.0, geode_robot_cost.0].iter().reduce(max).unwrap();
        let max_needed_clay = obsidian_robot_cost.1;
        let max_needed_obsidian = geode_robot_cost.1;
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,        
            max_needed_ore,
            max_needed_clay,
            max_needed_obsidian,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
}

impl State {
    fn new() -> Self {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        }
    }

    fn iterate(&self, ticks: i32) -> Self {
        let State { clay_bots, obsidian_bots, ore_bots, .. } = self;
        let mut next = self.clone(); 
        next.ore += ore_bots * ticks;
        next.obsidian += obsidian_bots * ticks;
        next.clay += clay_bots * ticks;

        next
    }

    fn can_sustain_geode_bot_production(&self, bp: &Blueprint) -> bool {
        let (ore, obsidian) = bp.geode_robot_cost;
        self.ore_bots >= ore && self.obsidian_bots >= obsidian
    }

    fn can_sustain_ore_production(&self, bp: &Blueprint) -> bool {
        self.ore_bots >= bp.max_needed_ore
    }

    fn can_sustain_clay_production(&self, bp: &Blueprint) -> bool {
        self.clay_bots >= bp.max_needed_clay
    }

    fn can_sustain_obsidian_production(&self, bp: &Blueprint) -> bool {
        self.obsidian_bots >= bp.max_needed_obsidian
    }

    fn ticks_until_ore_bot(&self, bp: &Blueprint) -> i32 {
        if self.ore >= bp.ore_robot_cost {
            return 1
        }
        return 1 + (bp.ore_robot_cost - self.ore).div_ceil(self.ore_bots);
    }

    fn ticks_until_clay_bot(&self, bp: &Blueprint) -> i32 {
        if self.ore >= bp.clay_robot_cost {
            return 1;
        }

        return 1 + (bp.clay_robot_cost - self.ore).div_ceil(self.ore_bots);
    }

    fn ticks_until_obsidian_bot(&self, bp: &Blueprint) -> Option<i32> {
        let (ore_cost, clay_cost) = bp.obsidian_robot_cost;
        if self.ore >= ore_cost && self.clay >= clay_cost {
            return Some(1);
        }

        if self.clay_bots == 0 {
            return None;
        }

        return Some(1 + max((ore_cost - self.ore).div_ceil(self.ore_bots), (clay_cost - self.clay).div_ceil(self.clay_bots)))
    }

    fn ticks_until_geode_bot(&self, bp: &Blueprint) -> Option<i32> {
        let (ore_cost, obsidian_cost) = bp.geode_robot_cost;
        if self.ore >= ore_cost && self.obsidian >= obsidian_cost {
            return Some(1);
        }

        if self.obsidian_bots == 0 {
            return None;
        }

        return Some(1 + max((ore_cost - self.ore).div_ceil(self.ore_bots), (obsidian_cost - self.obsidian).div_ceil(self.obsidian_bots)))
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day19");
    let blueprints = contents.lines().map(Blueprint::from_str).collect();
    let p1 = part_one(&blueprints);
    let p2 = part_two(&blueprints);

    println!("Elapsed: {:?}", start.elapsed());
    println!("D19P1: {p1:?}");
    println!("D19P2: {p2:?}");
}

fn max_possible_geodes(t: i32, geode_bots: i32) -> i32 {
    (0..t).fold(0, |acc, i| acc + ((t - i) * (geode_bots + i)))
}

fn get_quality_level(bp: &Blueprint, time: i32) -> i32 {
    type Cache = FxHashMap<(i32, State), i32>;
    type MaxGeodeCache = FxHashMap<i32, i32>;
    fn helper(time: i32, state: State, cache: &mut Cache, max_geode_cache: &mut MaxGeodeCache, bp: &Blueprint) -> i32 {
        if time <= 0 {
            return 0;
        }
        let cache_key = (time, state);
        if let Some(&cached) = cache.get(&cache_key) {
            return cached
        }

        let max_possible = max_possible_geodes(time, state.geode_bots);
        if let Some(&cached) = max_geode_cache.get(&time) {
            if cached > max_possible {
                cache.insert(cache_key, 0);
                return 0;
            }
        }
        
        if state.can_sustain_geode_bot_production(bp) {
            cache.insert(cache_key, max_possible);
            max_geode_cache.insert(time, max_possible);
            return max_possible;
        }

        // It is always better to build a geode robot if you can do so
        if bp.geode_robot_cost.1 <= state.ore && bp.geode_robot_cost.1 <= state.obsidian {
            let mut next_state = state.iterate(1);
            next_state.ore -= bp.geode_robot_cost.0;
            next_state.obsidian -= bp.geode_robot_cost.1;
            next_state.geode_bots += 1;
            let mut res = helper(time - 1, next_state, cache, max_geode_cache, bp);
            res += state.geode_bots;
            cache.insert(cache_key, res);
            if let Some(max_geodes) = max_geode_cache.get_mut(&time) {
                *max_geodes = max(*max_geodes, res);
            } else {
                max_geode_cache.insert(time, res);
            }
            return res;
        } 

        let mut res = 0;
        if !state.can_sustain_ore_production(bp) {
            let ticks = state.ticks_until_ore_bot(bp);
            let mut next_state = state.iterate(ticks);
            next_state.ore -= bp.ore_robot_cost;
            next_state.ore_bots += 1;
            let next_time = time - ticks;
            let generated = min(ticks, time) * state.geode_bots;
            res = max(res, generated + helper(next_time, next_state, cache, max_geode_cache, bp));
        }

        if !state.can_sustain_clay_production(bp) {
            let ticks = state.ticks_until_clay_bot(bp);
            let mut next_state = state.iterate(ticks);
            let next_time = time - ticks;
            next_state.ore -= bp.clay_robot_cost;
            next_state.clay_bots += 1;
            let generated = min(ticks, time) * state.geode_bots;
            res = max(res, generated + helper(next_time, next_state, cache, max_geode_cache, bp));
        }

        if !state.can_sustain_obsidian_production(bp) {
            if let Some(ticks) = state.ticks_until_obsidian_bot(bp) {
                let mut next_state = state.iterate(ticks);
                let (ore_cost, clay_cost) = bp.obsidian_robot_cost;
                let next_time = time - ticks;
                next_state.ore -= ore_cost;
                next_state.clay -= clay_cost;
                next_state.obsidian_bots += 1;
                let generated = min(ticks, time) * state.geode_bots;
                res = max(res, generated + helper(next_time, next_state, cache, max_geode_cache, bp));
            }
        }

        if let Some(ticks) = state.ticks_until_geode_bot(bp) {
            let mut next_state = state.iterate(ticks);
            let (ore_cost, obsidian_cost) = bp.geode_robot_cost;
            let next_time = time - ticks;
            next_state.ore -= ore_cost;
            next_state.obsidian -= obsidian_cost;
            next_state.geode_bots += 1;
            let generated = min(ticks, time) * state.geode_bots;
            res = max(res, generated + helper(next_time, next_state, cache, max_geode_cache, bp));
        }


        cache.insert(cache_key, res);
        if let Some(max_geodes) = max_geode_cache.get_mut(&time) {
            *max_geodes = max(*max_geodes, res);
        } else {
            max_geode_cache.insert(time, res);
        }
        res
    }
    let res = helper(time, State::new(), &mut FxHashMap::default(), &mut FxHashMap::default(), bp);
    return res
}

fn part_one(blueprints: &Vec<Blueprint>) -> i32 {
    blueprints.par_iter().map(|bp|bp.id * get_quality_level(bp, 24)).sum()
}

fn part_two(blueprints: &Vec<Blueprint>) -> i32 {
    blueprints[..3].par_iter().map(|bp|get_quality_level(bp, 32)).reduce(||1, |acc, val| acc * val)
}
