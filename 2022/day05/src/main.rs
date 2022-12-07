#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;

struct Command {
    count: usize,
    start: usize,
    end: usize,
}

fn main() {
    // Read into file
    let chom = fs::read_to_string("/home/joe/GitHub/advent-of-code/day05/input.txt").unwrap();

    // Split and filter empty lines
    let lines = chom
        .split("\n")
        .into_iter()
        .filter(|a| a != &"")
        .collect::<Vec<&str>>();

    // First 8 lines are the crates
    let crate_lines = lines.clone().into_iter().take(8);
    let crates: Vec<Vec<String>> = crate_lines
        .into_iter()
        .map(|line| -> Vec<String> {
            line.chars()
                .into_iter()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|a| -> Vec<char> { a.into_iter().cloned().collect() })
                .collect::<Vec<Vec<char>>>()
                .into_iter()
                .map(|a| -> String { a.into_iter().collect() })
                .collect::<Vec<String>>()
        })
        .collect();

    // Split crate lines into 'stacks' (they are actually the rows, not columns)
    let stacks: Vec<Vec<char>> = crates
        .into_iter()
        .map(|a| -> Vec<char> {
            a.into_iter()
                .map(|b| -> char { b.chars().collect::<Vec<char>>().get(1).unwrap().clone() })
                .collect()
        })
        .collect();

    // Construct letter stacks using eldrich bullshit (nested loop)
    let mut letters: Vec<Vec<char>> = vec![];
    let mut count = 0;
    loop {
        letters.push(vec![]);
        count += 1;
        if count > 8 {
            break;
        }
    }

    let mut row = 0;
    loop {
        let mut col = 0;
        if row > 7 {
            break;
        }
        loop {
            if col > 8 {
                break;
            }
            let row = stacks.get(row).unwrap();
            // println!("{}, {}", col, row.len());
            let elem = row.get(col).unwrap();
            if elem != &' ' {
                letters.get_mut(col).unwrap().push(elem.clone())
            }
            col += 1;
        }
        row += 1;
    }

    // Reverse letters because bozo
    letters = letters
        .into_iter()
        .map(|mut a| -> Vec<char> {
            a.reverse();
            a
        })
        .collect();

    println!("{:?}", letters);

    // Construct commands from command lines
    let command_lines: Vec<Command> = lines
        .into_iter()
        .skip_while(|a| !a.starts_with("move"))
        .filter(|a| a != &"")
        .map(|a| -> Command {
            let tokens = a.split(" ").collect::<Vec<&str>>();
            let count: usize = tokens.get(1).unwrap().parse().unwrap();
            let start: usize = tokens.get(3).unwrap().parse().unwrap();
            let end: usize = tokens.get(5).unwrap().parse().unwrap();
            Command {
                count,
                start: start - 1, // change from 1 index to 0 index
                end: end - 1,
            }
        })
        .collect();

    // Execute shit
    for command in command_lines {
        /*
        println!(
            "moving {} from {} to {}",
            command.count,
            command.start + 1,
            command.end + 1
        );
        */
        let mut i = 0;
        // println!("{:?}", letters.get(command.start).unwrap());
        // println!("{:?}", letters.get(command.end).unwrap());
        let mut intermediate: Vec<char> = vec![]; // PART 2
        loop {
            // println!("{}", i);
            if i >= command.count {
                break;
            }
            let start_stack = letters.get_mut(command.start).unwrap();
            let c = start_stack.pop().unwrap();
            // println!("popped {}", c);
            // letters.get_mut(command.end).unwrap().push(c);
            intermediate.push(c); // PART 2
            i += 1;
        }
        // BEGIN PART 2
        intermediate.reverse();
        letters
            .get_mut(command.end)
            .unwrap()
            .extend(intermediate.into_iter());
        // END PART 2

        // println!("{:?}", letters.get(command.start).unwrap());
        // println!("{:?}", letters.get(command.end).unwrap());
    }

    // Fucking finally
    println!("final");
    for stack in letters {
        println!("{:?}", stack);
    }
}
