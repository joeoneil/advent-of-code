#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let trees: Vec<Vec<usize>> = file
        .split("\n")
        .filter(|a| a != &"")
        .map(|a| -> Vec<usize> { a.chars().map(|c| c.to_string().parse().unwrap()).collect() })
        .collect();

    let rows = trees.len();
    let cols = trees.get(0).unwrap().len();

    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            let height = trees.get(r).unwrap().get(c).unwrap();
            let left: Vec<usize> = trees.get(r).unwrap()[0..c]
                .into_iter()
                .filter(|a| *a >= height)
                .map(|a| *a)
                .collect();

            let right: Vec<usize> = trees.get(r).unwrap()[c + 1..]
                .into_iter()
                .filter(|a| *a >= height)
                .map(|a| *a)
                .collect();

            let top: Vec<usize> = trees[0..r]
                .into_iter()
                .map(|a| a.get(c).unwrap())
                .filter(|a| *a >= height)
                .map(|a| *a)
                .collect();

            let bottom: Vec<usize> = trees[r + 1..]
                .into_iter()
                .map(|a| a.get(c).unwrap())
                .filter(|a| *a >= height)
                .map(|a| *a)
                .collect();

            /*
            println!(
                "{}, {} L:{} R:{} T:{} B:{}",
                r,
                c,
                left.len(),
                right.len(),
                top.len(),
                bottom.len()
            );
            */
            if left.len() == 0 || right.len() == 0 || top.len() == 0 || bottom.len() == 0 {
                count += 1;
                // println!("{}, {} visible", r, c);
            }
        }
    }

    println!("{}", count);

    // Part 2
    let mut greatest = 0;
    for r in 0..rows {
        for c in 0..cols {
            let height = trees.get(r).unwrap().get(c).unwrap();

            let mut extra = 0;
            let mut left: usize = trees.get(r).unwrap()[0..c]
                .into_iter()
                .rev()
                .map_while(|a| -> Option<()> {
                    if a < height {
                        Some(())
                    } else {
                        extra = 1;
                        None
                    }
                })
                .count();
            left += extra;

            extra = 0;
            let mut right: usize = trees.get(r).unwrap()[c + 1..]
                .into_iter()
                .map_while(|a| -> Option<()> {
                    if a < height {
                        Some(())
                    } else {
                        extra = 1;
                        None
                    }
                })
                .count();
            right += extra;

            extra = 0;
            let mut top: usize = trees[0..r]
                .into_iter()
                .rev()
                .map(|a| a.get(c).unwrap())
                .map_while(|a| -> Option<()> {
                    if a < height {
                        Some(())
                    } else {
                        extra = 1;
                        None
                    }
                })
                .count();
            top += extra;

            extra = 0;
            let mut bottom: usize = trees[r + 1..]
                .into_iter()
                .map(|a| a.get(c).unwrap())
                .map_while(|a| -> Option<()> {
                    if a < height {
                        Some(())
                    } else {
                        extra = 1;
                        None
                    }
                })
                .count();
            bottom += extra;

            /*
            println!(
                "{}, {} L:{} R:{} T:{} B:{}",
                r,
                c,
                left.len(),
                right.len(),
                top.len(),
                bottom.len()
            );
            */
            /*
            println!(
                "{}: {}, {}, [{}, {}, {}, {}]",
                trees.get(r).unwrap().get(c).unwrap(),
                r,
                c,
                left,
                right,
                top,
                bottom
            );
            */
            if greatest <= left * right * top * bottom {
                greatest = left * right * top * bottom;
            }
        }
    }

    println!("{}", greatest);
}
