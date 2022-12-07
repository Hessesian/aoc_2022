use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    let normalized = input.lines().map(|l| {
        let l = l.trim().chars();
        l.map(normalize).collect::<Vec<u8>>()
    });

    let res: Vec<Vec<u8>> = normalized
        .clone()
        .map(|l| {
            let (left, right) = (&l[0..l.len() / 2], &l[l.len() / 2..l.len()]);
            let mut matching: Vec<u8> = vec![];
            left.iter().for_each(|c| {
                if right.contains(c) && !matching.contains(c) {
                    matching.push(*c);
                }
            });
            matching
        })
        .collect();
    let part1 = sum_res(res);

    println!("Part1: {}", part1);
    let mut p2_res: Vec<Vec<u8>> = vec![];
    let mut res = normalized.clone();
    loop {
        let mut matching: Vec<u8> = vec![];
        let Some(first) = res.next() else {break};
        let Some(second) = res.next() else {break};
        let Some(third) = res.next() else {break};
        for a in first.iter() {
            if second.contains(a) && third.contains(a) && !matching.contains(a) {
                matching.push(*a);
            }
        }
        p2_res.push(matching);
    }
    let part1 = sum_res(p2_res);
    println!("Part2 {}", part1);
}

fn sum_res(res: Vec<Vec<u8>>) -> usize {
    let part1 = res
        .iter()
        .map(|l| l.iter().fold(0, |acc, x| acc + *x as usize))
        .sum::<usize>();
    part1
}

fn normalize(c: char) -> u8 {
    let val: u8 = if c.is_ascii_lowercase() {
        b'a' - 1
    } else {
        b'A' - 27
    };
    c as u8 - val
}

#[test]
fn ascii_vals() {
    let u8 = 8;
    let b = 9 & u8;
    assert_eq!(b, 8);
}

#[test]
fn anon() {
    let input = read_to_string("../input.txt").expect("Failed to read input");

    let total: u32 = input
        .lines()
        .map(|line| {
            let a: HashSet<_> = line[..line.len() / 2].chars().into_iter().collect();
            let b: HashSet<_> = line[line.len() / 2..].chars().into_iter().collect();
            let x = a.intersection(&b).next().unwrap();
            match &x {
                ref val if ('a'..='z').contains(val) => (val as u32) - ('a' as u32) + 1,
                _ => (*x as u32) - ('A' as u32) + 27,
            }
        })
        .sum();
    println!("{:?}", total);
}
