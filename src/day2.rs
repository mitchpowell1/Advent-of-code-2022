use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "inputs/day2_input.txt";

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day2");
    let parsed = contents.trim().lines();
    let p1 = evaluate(parsed.clone(), get_p1_score);
    let p2 = evaluate(parsed, get_p2_score);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D2P1: {p1:?}");
    println!("D2P2: {p2:?}");
}

fn get_p1_score(a: &str, b: &str) -> i32 {
    match b {
        "X" => 1 + match a {
            "A" => 3,
            "B" => 0,
            "C" => 6,
            _ => panic!(),
        },
        "Y" => 2 + match a {
            "A" => 6,
            "B" => 3,
            "C" => 0,
            _ => panic!(),
        },
        "Z" => 3 + match a {
            "A" => 0,
            "B" => 6,
            "C" => 3,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn get_p2_score(play: &str, outcome: &str) -> i32 {
    let response = match play {
        // Rock
        "A" => match outcome {
            // Lose
            "X" => "C",
            // Tie
            "Y" => "A",
            // Win
            "Z" => "B",
            _ => panic!(),
        },
        // Paper
        "B" => match outcome {
            // Lose
            "X" => "A",
            // Tie
            "Y" => "B",
            // Win
            "Z" => "C",
            _ => panic!(),
        },
        // Scissors
        "C" => match outcome {
            // Lose
            "X" => "B",
            // Tie
            "Y" => "C",
            // Win
            "Z" => "A",
            _ => panic!(),
        },
        _ => panic!(),
    };

    let result_score = match outcome {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    };

    let response_score = match response {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!(),
    };

    result_score + response_score

}

fn evaluate<'a>(plays: impl Iterator<Item = &'a str>, score_func: fn(&'a str, &'a str) -> i32) -> i32 {
    let mut total = 0;
    for line in plays {
        let mut split = line.split(' ');
        let play = split.next().unwrap();
        let response = split.next().unwrap();
        total += score_func(play, response);
    }
    total
}

