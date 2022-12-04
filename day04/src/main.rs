#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;

fn main() {
    let contains: usize = fs::read_to_string("/home/joe/GitHub/advent/day04/input.txt")
        .unwrap()
        .split("\n")
        .into_iter()
        .filter(|a| a != &"")
        .map(|s| -> (isize, isize, isize, isize) {
            let s1 = s.replace(",", "-");
            let mut s2 = s1.split('-').map(|a| a.parse::<isize>().unwrap());
            (
                s2.next().unwrap(),
                s2.next().unwrap(),
                s2.next().unwrap(),
                s2.next().unwrap(),
            )
        })
        //.filter(|a| -> bool { a.0 >= a.2 && a.1 <= a.3 || a.2 >= a.0 && a.3 <= a.1 }) // Part 1
        .filter(|a| -> bool { a.2 <= a.0 && a.0 <= a.3 || a.0 <= a.2 && a.2 <= a.1 }) // Part 2
        .count();
    println!("{}", contains)
}
