use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr};
enum Dir {
    L,
    R,
    U,
    D,
}

struct Cmd {
    dir: Dir,
    step: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("bb.txt")?;
    let cmds: Vec<Cmd> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let positions = solve(&cmds, 2);

    println!("P1: {}", positions.len());
    let positions = solve(&cmds, 10);

    println!("P2: {}", positions.len());
    Ok(())
}

fn towards(from: i32, to: i32) -> i32 {
    match (from, to) {
        (f, t) if f < t => 1,
        (f, t) if f > t => -1,
        _ => 0,
    }
}
fn solve(cmds: &[Cmd], len: usize) -> HashSet<(i32, i32)> {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); len];
    for cmd in cmds.iter() {
        let step = cmd.dir.get_step();
        (0..cmd.step).for_each(|_| {
            for i in 0..rope.len() - 1 {
                let mut head = rope[i];
                let mut tail = rope[i + 1];
                if i == 0 {
                    head.0 += step.0;
                    head.1 += step.1;
                }

                if tail.0.abs_diff(head.0) > 1 || tail.1.abs_diff(head.1) > 1 {
                    tail.1 += towards(tail.1, head.1);
                    tail.0 += towards(tail.0, head.0);
                }
                rope[i] = head;
                rope[i + 1] = tail;
                if i + 1 == rope.len() - 1 {
                    positions.insert(tail);
                }
            }
        });
    }
    positions
}

impl Dir {
    fn get_step(&self) -> (i32, i32) {
        match self {
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
            Dir::U => (0, 1),
            Dir::D => (0, -1),
        }
    }
}

impl FromStr for Cmd {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((left, right)) = s.split_once(' ') else { return Err("Can't split cmd");};
        let count = right.parse::<usize>().map_err(|_| "Can't parse count")?;
        Ok(Cmd {
            dir: left.parse()?,
            step: count,
        })
    }
}

impl FromStr for Dir {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::R),
            "L" => Ok(Dir::L),
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            _ => Err("No such command"),
        }
    }
}
