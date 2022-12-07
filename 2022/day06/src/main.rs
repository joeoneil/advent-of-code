#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs;

fn main() {
    let file = fs::read_to_string("/home/joe/GitHub/advent-of-code/2022/day06/input.txt").unwrap();
    let chars: Vec<char> = file.chars().collect();
    let mut index = 0;
    loop {
        let slice = &chars[index..index + 14];
        let mut cont = false;
        for i in 0..14 {
            for j in 0..14 {
                if i.eq(&j) {
                    continue;
                }
                if slice.get(i).unwrap().eq(slice.get(j).unwrap()) {
                    cont = true;
                }
            }
        }
        if !cont {
            break;
        }
        index += 1;
    }
    println!("{}", index);
}
