use std::fs;
use std::time::Instant;
use std::cmp::{min, max};

const FILE_PATH: &str = "inputs/day15_input.txt";
const SCAN_LINE: i32 = 2000000;
const DISTRESS_UPPER_BOUND: i32 = 4_000_000;
const DISTRESS_LOWER_BOUND: i32 = 0;

#[derive(Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn get_distance(&self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

struct Sensor {
    beacon: Point,
    location: Point,
    beacon_distance: i32,
}

impl Sensor {
    fn new(location: Point, beacon: Point) -> Self {
        Sensor { location, beacon, beacon_distance: location.get_distance(beacon) }
    }
}

#[derive(Debug)]
struct ScanRange {
    min: i32,
    max: i32,
}
impl ScanRange {
    fn new(min: i32, max: i32) -> Self {
        ScanRange { min, max }
    }

    fn intersects(&self, other: &ScanRange) -> bool {
        return
            self.min >= other.min && self.min <= other.max || 
            self.max >= other.min && self.max <= other.max ||
            other.max >= self.min && other.max <= self.max ||
            other.min >= self.min && other.min <= self.max
    }
    fn contains(&self, val: i32) -> bool {
        self.min <= val && self.max >= val
    }

}

#[derive(Debug)]
struct Scanned {
    ranges: Vec::<ScanRange>
}

impl Scanned {
    fn new() -> Self {
        Scanned { ranges: vec!() }
    }
    fn merge_ranges(&mut self) {
        let mut i = 0;
        let mut j = 1;
        loop {
            if j >= self.ranges.len() {
                return
            }
            if self.ranges[i].intersects(&self.ranges[j]) {
                self.ranges[i].min = min(self.ranges[j].min, self.ranges[i].min);
                self.ranges[i].max = max(self.ranges[j].max, self.ranges[i].max);
                self.ranges.remove(j);
                j = i + 1;
            } else {
                j += 1;
                if j == self.ranges.len() {
                    i += 1;
                    j = i + 1;
                }
            }
        }
    }
    fn insert(&mut self, rng1: ScanRange) {
        for rng2 in self.ranges.iter_mut() {
            if rng2.intersects(&rng1) {
                rng2.min = min(rng1.min, rng2.min);
                rng2.max = max(rng1.max, rng2.max);
                self.merge_ranges();
                return;
            }
        }
        self.ranges.push(rng1);
    }

    fn contains(&self, val: i32) -> bool {
        self.ranges.iter().any(|rng| rng.contains(val))
    }

    fn remove(&mut self, v: i32) {
        for i in 0..self.ranges.len() {
            let rng = &mut self.ranges[i];
            if rng.min == v && rng.max == v {
                self.ranges.remove(i);
                self.merge_ranges();
                return; 
            }
            if rng.min == v {
                rng.min = v + 1;
                self.merge_ranges();
                return;
            }
            if rng.max == v {
                rng.max = v - 1;
                self.merge_ranges();
                return;
            }
            if rng.min < v && rng.max > v {
                let rng2 = ScanRange::new(v + 1, rng.max);
                rng.max = v - 1;
                self.ranges.push(rng2);
            }
        }
    }

    fn size(&self) -> usize {
        let mut out = 0;
        for rng in &self.ranges {
            out += ((rng.max - rng.min) + 1) as usize;
        }
        out
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors = vec!();
    for line in input.lines() {
        let (sensor_str, beacon_str) = line.split_once(": ").unwrap();
        let (_, x_str) = sensor_str.split_once("x=").unwrap();
        let (sensor_x, sensor_y) = x_str.split_once(", y=").unwrap();
        let (_, x_str) = beacon_str.split_once("x=").unwrap();
        let (beacon_x, beacon_y) = x_str.split_once(", y=").unwrap();

        sensors.push(Sensor::new(
            Point(sensor_x.parse().unwrap(), sensor_y.parse().unwrap()),
            Point(beacon_x.parse().unwrap(), beacon_y.parse().unwrap()),
        ));
    }
    sensors
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day15");
    let sensors = parse_input(&contents);
    let mut scanned = Scanned::new();
    part_one(&sensors, SCAN_LINE, false, &mut scanned);
    let p1 = scanned.size();
    let p2 = part_two(&sensors, &mut scanned);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D15P1: {p1:?}");
    println!("D15P2: {p2:?}");
}

fn part_one(sensors: &Vec<Sensor>, line: i32, set_bounds: bool, scanned: &mut Scanned) {
    for sensor in sensors {
        let Point(x, _) = sensor.location;
        let line_distance = sensor.location.get_distance(Point(x, line));
        let intersection = sensor.beacon_distance - line_distance;
        if intersection < 0 {
            continue
        }
        if set_bounds {
            let min_bound = max(x - intersection, DISTRESS_LOWER_BOUND);
            let max_bound = min(x + intersection, DISTRESS_UPPER_BOUND);
            scanned.insert(ScanRange::new(min_bound, max_bound));
        } else {
            scanned.insert(ScanRange::new(x-intersection, x + intersection));
        }
    }
    
    if !set_bounds {
        for Sensor{ beacon, .. } in sensors {
            if beacon.1 == line {
                scanned.remove(beacon.0);
            }
        }
    }
}

fn part_two(sensors: &Vec<Sensor>, scanned: &mut Scanned) -> u128 {
    for line in 0..DISTRESS_UPPER_BOUND {
        scanned.ranges.clear();
        part_one(sensors, line, true, scanned);
        if scanned.size() == DISTRESS_UPPER_BOUND as usize {
            for x in 0..DISTRESS_UPPER_BOUND {
                if !scanned.contains(x) {
                    return ((x as u128 * 4_000_000 as u128) + line as u128) as u128;
                }
            }
        }
    }
    0
}
