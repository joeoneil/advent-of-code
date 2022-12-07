#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;
use std::path::Path;

fn main() {
    let file = fs::read_to_string(Path::new("./input.txt")).unwrap();
    let mut floor = 0;
    let chars: Vec<char> = file.chars().filter(|c| c == &'(' || c == &')').collect();
    let mut index = 0;
    for c in chars {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            break;
        }
        index += 1;
    }
    println!("{}", index);
}
