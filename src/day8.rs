mod real_range;

use std::fs;
use std::time::Instant;
use std::cmp::max;

use crate::real_range::RealRange;

const FILE_PATH: &str = "inputs/day8_input.txt";

type Grid<T> = Vec<Vec<T>>;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day8");
    let parsed: Grid<u32> = contents
        .lines()
        .map(|l| {
            l.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<_>>()
        }).collect();
    let (p1, p2) = count_visible(&parsed);

    println!("Elapsed: {:?}", start.elapsed());
    println!("D8P1: {p1:?}");
    println!("D8P2: {p2:?}");
}

fn count_visible(trees: &Grid<u32>) -> (u32, u32) {
    let (width, height) = (trees[0].len(), trees.len());
    let (mut visible, mut max_scenic_score) = (0,0);

    let can_see_edge = |r_range: RealRange, c_range: RealRange, tree: u32| {
        for r in r_range {
            for c in c_range {
                if trees[r][c] >= tree {
                    return false;
                }
            }
        }
        true
    };

    let get_visibility = |r_range: RealRange, c_range: RealRange, tree: u32| {
        let mut vis = 0;
        'outer : for r in r_range {
            for c in c_range {
                vis += 1;
                if trees[r][c] >= tree {
                    break 'outer;
                }
            }
        }
        vis
    };

    for (r, _) in trees.iter().enumerate().take(width) {
        for c in 0..height {
            if r == 0 || c == 0 || r == width - 1 || c == height - 1 {
                visible += 1;
                continue;
            }
            let row_ranges = [RealRange::descending(r-1, 0), RealRange::ascending(r + 1, width)];
            let col_ranges = [RealRange::descending(c-1, 0), RealRange::ascending(c + 1, height)];

            let tree = trees[r][c];
            if row_ranges.into_iter().any(|r_range| can_see_edge(r_range, RealRange::ascending(c, c + 1), tree)) || 
                col_ranges.into_iter().any(|c_range| can_see_edge(RealRange::ascending(r, r + 1), c_range, tree)) { 
                    visible += 1;
                };

            let row_multiplicand = row_ranges.into_iter().map(|r_range| get_visibility(r_range, RealRange::ascending(c, c + 1), tree)).reduce(|a,b|a*b).unwrap();
            let col_multiplicand = col_ranges.into_iter().map(|c_range| get_visibility(RealRange::ascending(r, r + 1), c_range, tree)).reduce(|a,b|a*b).unwrap();
            max_scenic_score = max(max_scenic_score, row_multiplicand * col_multiplicand);
        }
    }

    (visible, max_scenic_score)
}
