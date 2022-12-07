#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs;

#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let file = fs::read_to_string("/home/joe/GitHub/advent-of-code/2022/day02/input.txt").unwrap();
    let lines: Vec<&str> = file.split("\n").collect();
    let moves: Vec<(RPS, RPS)> = lines
        .into_iter()
        .filter(|s| !s.eq(&""))
        .map(|s| parse2(s))
        .collect();
    let score: isize = moves.into_iter().map(|m| score(m)).sum();
    println!("{}", score);
}

fn parse(input: &str) -> (RPS, RPS) {
    let split: Vec<&str> = input.split(" ").collect();
    let (a, b) = (split.get(0).unwrap(), split.get(1).unwrap());
    let r1 = match *a {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => unreachable!(),
    };
    let r2 = match *b {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => unreachable!(),
    };
    return (r1, r2);
}

fn parse2(input: &str) -> (RPS, RPS) {
    let split: Vec<&str> = input.split(" ").collect();
    let (a, b) = (split.get(0).unwrap(), split.get(1).unwrap());
    let r1 = match *a {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => unreachable!(),
    };
    let r2 = match *b {
        "X" => lose(&r1),
        "Y" => tie(&r1),
        "Z" => win(&r1),
        _ => unreachable!(),
    };
    (r1, r2)
}

fn score(input: (RPS, RPS)) -> isize {
    let move_score = match &input.1 {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    let win = match &input {
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 0,
        (RPS::Scissors, RPS::Scissors) => 3,
    };

    move_score + win
}

fn win(input: &RPS) -> RPS {
    match input {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    }
}

fn lose(input: &RPS) -> RPS {
    match input {
        RPS::Rock => RPS::Scissors,
        RPS::Paper => RPS::Rock,
        RPS::Scissors => RPS::Paper,
    }
}

fn tie(input: &RPS) -> RPS {
    input.clone()
}
