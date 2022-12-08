#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

#[derive(Clone)]
enum Entry {
    Dir(Dir),
    File(usize),
}

impl Entry {
    fn get_size(&self) -> usize {
        match &self {
            Entry::Dir(d) => d.get_size(),
            Entry::File(f) => *f,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Dir(d) => write!(f, "{}", d),
            Entry::File(file) => write!(f, "{}", file),
        }
    }
}

#[derive(Clone)]
struct Dir {
    entries: HashMap<String, Entry>,
    depth: usize,
}

impl Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.entries.clone() {
            match entry.1 {
                Entry::Dir(dir) => {
                    write!(
                        f,
                        "d [{:10}] {}{}\n",
                        dir.get_size(),
                        (0..self.depth).map(|_| "    ").collect::<String>(),
                        entry.0
                    )
                    .unwrap(); // fuck it we ball
                    write!(f, "{}", dir).unwrap();
                }
                Entry::File(file) => write!(
                    f,
                    "- [{:10}] {}{}]\n",
                    file,
                    (0..self.depth).map(|_| "\t").collect::<String>(),
                    entry.0
                )
                .unwrap(),
            }
        }
        Ok(())
    }
}

impl Dir {
    fn new() -> Self {
        Dir {
            entries: HashMap::new(),
            depth: 0,
        }
    }

    fn new_with_depth(depth: usize) -> Self {
        Dir {
            entries: HashMap::new(),
            depth,
        }
    }

    fn is_leaf(self: &Self) -> bool {
        !(self
            .entries
            .clone()
            .into_iter()
            .filter(|a| match a.1 {
                Entry::Dir(_) => true,
                Entry::File(_) => false,
            })
            .count()
            > 0)
    }

    fn add(self: &mut Self, e: Entry, name: &str) -> () {
        self.entries.insert(name.to_string(), e);
    }

    fn get_mut(self: &mut Self, name: &str) -> Option<&mut Entry> {
        self.entries.get_mut(&name.to_string())
    }

    fn get(self: &Self, name: &str) -> Option<&Entry> {
        self.entries.get(&name.to_string())
    }

    fn get_dir_mut(self: &mut Self, name: &str) -> Option<&mut Dir> {
        if let Some(Entry::Dir(d)) = self.entries.get_mut(&name.to_string()) {
            Some(d)
        } else {
            None
        }
    }

    fn get_or_add_dir(self: &mut Self, name: &str) -> &mut Dir {
        match self
            .entries
            .entry(name.to_string())
            .or_insert(Entry::Dir(Dir::new_with_depth(self.depth + 1)))
        {
            Entry::Dir(d) => d,
            Entry::File(f) => unreachable!(),
        }
    }

    fn get_size(self: &Self) -> usize {
        self.entries
            .clone()
            .into_iter()
            .map(|a| a.1.get_size())
            .sum()
    }

    fn flatten(self: &Self) -> Vec<Dir> {
        let mut children = self
            .entries
            .clone()
            .into_iter()
            .map(|a| a.1)
            .filter(|a| -> bool {
                match a {
                    Entry::Dir(_) => true,
                    Entry::File(_) => false,
                }
            })
            .flat_map(|a| -> Vec<Dir> {
                match a {
                    Entry::Dir(d) => d.flatten(),
                    Entry::File(_) => unreachable!(),
                }
            })
            .collect::<Vec<Dir>>();
        children.push(self.clone());
        children
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = file.split("\n").filter(|a| a != &"").collect();

    let mut tld: Dir = Dir::new();

    let mut stack: Vec<&str> = vec![];
    {
        // extra block so that current_dir drops out of scope later
        let mut current_dir: &mut Dir = &mut tld;
        for line in lines {
            let tokens: Vec<&str> = line.split(" ").collect();

            // parse commands
            if tokens.get(0).unwrap().eq(&"$") {
                // line is command
                if tokens.get(1).unwrap().eq(&"ls") {
                    continue;
                } // ls can be ignored
                let arg = tokens.get(2).unwrap(); // cmd is cd, 2nd index is dir
                current_dir = match *arg {
                    "/" => &mut tld,
                    ".." => {
                        stack.pop();
                        let mut local_stack = stack.clone();
                        let mut next_dir: &mut Dir = &mut tld;
                        loop {
                            // println!("{:?}, {:?}", next_dir.get_entries(), local_stack);
                            next_dir = match local_stack.get(0) {
                                Some(s) => next_dir.get_dir_mut(s).expect(
                                    format!("{}, {:?}. {:?}", "Thing ate shit", local_stack, stack)
                                        .as_str(),
                                ),
                                None => break next_dir,
                            };
                            local_stack.remove(0);
                        }
                    }
                    _ => {
                        stack.push(tokens.get(2).unwrap());
                        current_dir.get_or_add_dir(tokens.get(2).unwrap())
                    }
                };
                continue;
            }

            // parse entries
            if tokens.get(0).unwrap().eq(&"dir") {
                // Entry is directory
                let name = tokens.get(1).unwrap();
                current_dir.add(Entry::Dir(Dir::new_with_depth(stack.len())), name);
            } else {
                // Entry is a file
                let size = tokens.get(0).unwrap().parse::<usize>().unwrap();
                let name = tokens.get(1).unwrap();

                current_dir.add(Entry::File(size), name);
            }
        }
    }

    println!("{}", tld);

    println!("{}", tld.flatten().len());

    println!("{}", tld.is_leaf());

    println!("{}", Dir::new().is_leaf());

    let total_size: usize = tld
        .flatten()
        .into_iter()
        .filter(|a| a.get_size() <= 100000)
        .map(|a| a.get_size())
        .sum();

    let space_needed = 30000000 - (70000000 - tld.get_size());
    let mut dirs = tld.flatten();
    dirs.sort_by(|a, b| a.get_size().cmp(&b.get_size()));

    dirs = dirs
        .into_iter()
        .filter(|a| a.get_size() >= space_needed)
        .collect();

    println!("{}", dirs.get(0).unwrap().get_size());

    let chom = tld.entries.clone();
}
