use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();

    let mut vals: Vec<usize> = input
        .split("\n\n")
        .map(|l| l.split('\n').flat_map(|v| v.parse::<usize>().ok()).sum())
        .collect();

    vals.sort();
    println!("Solution 1 {:?}", vals.last());
    let part_b: usize = vals.iter().rev().take(3).sum();
    println!("Solution 2 {:?}", part_b);
}
