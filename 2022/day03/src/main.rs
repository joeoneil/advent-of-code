#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;

fn main() {
    let test = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let thing: Vec<isize> = test.chars().into_iter().map(|c| parse(c)).collect();
    // println!("{:?}", thing);

    let file = fs::read_to_string("/home/joe/GitHub/advent/day03/input.txt").unwrap();

    let bags: Vec<(Vec<isize>, Vec<isize>)> = file
        .split("\n")
        .map(|s| -> (Vec<isize>, Vec<isize>) {
            let as_string = s.to_string();
            let chars = as_string.chars();
            let first = chars.clone().take(chars.clone().count() / 2);
            let second = chars
                .clone()
                .skip(chars.clone().count() / 2)
                .take(chars.clone().count() / 2);
            return (
                first.into_iter().map(|c| parse(c)).collect::<Vec<isize>>(),
                second.into_iter().map(|c| parse(c)).collect::<Vec<isize>>(),
            );
        })
        .filter(|b| b.0.get(0).is_some())
        .collect();
    let mut sum = 0;
    let mut index = 0;
    for bag in bags.clone() {
        let (a, b) = bag.clone();
        let mut item_found = false;
        for item in a.clone() {
            if item_found {
                break;
            }
            for item2 in b.clone() {
                if item == item2 {
                    sum += item;
                    item_found = true;
                    break;
                }
            }
        }
        index += 1;
    }

    let mut badge_sum = 0;
    let chunks: Vec<Vec<(Vec<isize>, Vec<isize>)>> = bags.chunks(3).map(|a| a.to_vec()).collect();
    println!("{}", bags.len());
    for chunk in chunks {
        assert_eq!(chunk.len(), 3);
        let (a, b, c) = (
            chunk.get(0).unwrap(),
            chunk.get(1).unwrap(),
            chunk.get(2).unwrap(),
        );
        let mut ac = a.clone().0;
        ac.append(&mut a.clone().1);
        let mut bc = b.clone().0;
        bc.append(&mut b.clone().1);
        let mut cc = c.clone().0;
        cc.append(&mut c.clone().1);

        let mut common: Vec<isize> = vec![];
        for elem_a in ac {
            for elem_b in bc.clone() {
                if elem_a == elem_b {
                    common.push(elem_a);
                }
            }
        }
        let mut badge: isize = 0;
        for elem_ab in common {
            if badge != 0 {
                break;
            }
            for elem_c in cc.clone() {
                if elem_ab == elem_c {
                    badge = elem_ab;
                    break;
                }
            }
        }
        badge_sum += badge;
    }
    println!("{}", sum);
    println!("{}", badge_sum);
}

fn parse(input: char) -> isize {
    let num = input as usize;
    return if num >= 'A' as usize && num <= 'Z' as usize {
        (num - 64 + 26) as isize
    } else {
        (num - 96) as isize
    };
}
