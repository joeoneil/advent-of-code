#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let input =
        std::fs::read_to_string("/home/joe/GitHub/advent-of-code/2022/day01/input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut out: Vec<Vec<isize>> = vec![];
    let mut index: usize = 0;
    let mut line_index: usize = 0;
    while lines.len() > line_index {
        out.push(vec![]);
        loop {
            let current: &mut Vec<isize> = out.get_mut(index).unwrap();
            line_index += 1;
            if lines.get(line_index).is_some() && lines.get(line_index).unwrap() != &"" {
                current.push(lines.get(line_index).unwrap().parse().unwrap_or(0));
            } else {
                break;
            }
        }
        index += 1;
    }

    let mut sums: Vec<isize> = out
        .into_iter()
        .filter(|a| a.len() > 0)
        .map(move |v| v.into_iter().sum::<isize>())
        .collect::<Vec<isize>>();
    sums.sort_by(|a, b| b.cmp(a));

    println!(
        "1: {}, 2: {}, 3: {}",
        sums.get(0).unwrap(),
        sums.get(1).unwrap(),
        sums.get(2).unwrap()
    );
}
