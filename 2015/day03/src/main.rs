use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    // let file = "^v^v^v^v^v";
    let mut pos = (0, 0);
    let mut robo = (0, 0);
    let mut visited: HashMap<(isize, isize), isize> = HashMap::new();
    visited.insert(pos.clone(), 1);
    visited.insert(pos.clone(), 1);
    file.chars()
        .filter(|a| -> bool {
            let ret = a == &'v' || a == &'^' || a == &'<' || a == &'>';
            if !ret {
                println!("{}, {}", a, ret);
            }
            ret
        })
        .collect::<Vec<char>>()
        .chunks(2)
        .for_each(|t| -> () {
            let (a, b) = (t[0], t[1]);
            match a {
                '^' => pos = (pos.0, pos.1 + 1),
                'v' => pos = (pos.0, pos.1 - 1),
                '<' => pos = (pos.0 - 1, pos.1),
                '>' => pos = (pos.0 + 1, pos.1),
                _ => {}
            }
            match b {
                '^' => robo = (robo.0, robo.1 + 1),
                'v' => robo = (robo.0, robo.1 - 1),
                '<' => robo = (robo.0 - 1, robo.1),
                '>' => robo = (robo.0 + 1, robo.1),
                _ => {}
            }
            // println!("{}", visited.len());
            visited.entry(pos).or_insert(1);
            visited.entry(robo).or_insert(1);
            visited.entry(pos).and_modify(|a| *a += 1);
            visited.entry(robo).and_modify(|a| *a += 1);
            // println!("{}", visited.len());
        });
    println!("{}", visited.len());
}
