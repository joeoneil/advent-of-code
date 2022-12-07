use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let dims: Vec<(usize, usize, usize)> = file
        .split("\n")
        .filter(|a| a != &"")
        .map(|a| -> (usize, usize, usize) {
            let mut split = a.split("x");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let paper: usize = dims
        .clone()
        .into_iter()
        .map(|a| -> usize {
            // get total for all sides
            let lw = 2 * a.0 * a.1;
            let wh = 2 * a.1 * a.2;
            let lh = 2 * a.0 * a.2;
            let mut v = vec![a.0, a.1, a.2];
            v.sort();
            let smallest = v.get(0).unwrap() * v.get(1).unwrap();
            /*
            println!(
                "{}, {}, {}",
                v.get(0).unwrap(),
                v.get(1).unwrap(),
                v.get(2).unwrap()
            );
            */
            return lw + wh + lh + smallest;
        })
        .sum();

    let ribbon: usize = dims
        .clone()
        .into_iter()
        .map(|a| -> usize {
            let mut v = vec![a.0, a.1, a.2];
            v.sort();
            let perimeter = 2 * (v.get(0).unwrap() + v.get(1).unwrap());
            let volume = v.get(0).unwrap() * v.get(1).unwrap() * v.get(2).unwrap();
            perimeter + volume
        })
        .sum();
    println!("{}", paper);
    println!("{}", ribbon);
}
