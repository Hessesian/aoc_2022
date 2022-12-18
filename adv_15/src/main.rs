use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
    fs::read_to_string,
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    let beacons: Vec<Beacon> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let target_y = 2_000_000;
    let (regions, objects) = solve(&beacons, target_y, None);
    let stack = reduce(regions);

    println!(
        "P1: {}",
        stack.iter().map(|(a, b)| b - a).sum::<isize>() - objects
    );

    let max = 4000000;
    // let max = 20;
    for y in 0..max {
        let (regions, _) = solve(&beacons, y, Some(max as usize));
        let stack = reduce(regions);
        if stack.len() > 1 {
            let p2 = ((stack[1].1) * 4000000) + y as isize;
            println!("P2: {p2}");
            break;
        }
    }

    Ok(())
}

fn reduce(mut regions: Vec<(isize, isize)>) -> VecDeque<(isize, isize)> {
    let mut stack: VecDeque<(isize, isize)> = VecDeque::new();
    regions.sort_by(|a, b| a.0.cmp(&b.0));
    let rng = regions.remove(0);
    stack.push_front(rng);
    for i in 0..regions.len() {
        let Some(top) = stack.pop_front() else  {break;};
        let Some(next) = regions.get(i) else {break;};

        if top.1 < next.0 {
            stack.push_back(top);
            stack.push_front(*next);
        } else if top.1 < next.1 {
            stack.push_front((top.0, next.1));
        } else {
            stack.push_front(top);
        }
    }
    stack
}

fn solve(
    beacons: &[Beacon],
    target_y: isize,
    max_x: Option<usize>,
) -> (Vec<(isize, isize)>, isize) {
    let mut regions: Vec<(isize, isize)> = vec![];
    let mut extra: HashSet<(isize, isize)> = HashSet::new();
    for beacon in beacons.iter() {
        let x = beacon.position.0;
        let from_target: isize =
            beacon.dist as isize - beacon.position.1.abs_diff(target_y) as isize;
        if from_target >= 0 {
            let width: isize = from_target;
            let mut min = x - width;
            let mut max = x + width + 1;
            if let Some(max_x) = max_x {
                min = 0.max(min.min(max_x as isize));
                max = (max_x).min(max as usize) as isize;
            }
            regions.push((min, max));
            let range = (min)..(max);
            if beacon.position.1 == target_y && range.contains(&beacon.position.0) {
                extra.insert(beacon.position);
            }
            if beacon.nearest.1 == target_y && range.contains(&beacon.nearest.0) {
                extra.insert(beacon.nearest);
            }
        }
    }
    (regions, extra.len() as isize)
}
#[derive(Debug)]
struct Beacon {
    position: (isize, isize),
    nearest: (isize, isize),
    dist: usize,
}

impl Display for Beacon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in -5..20 {
            for x in -5..30 {
                let pos = (x as isize, y as isize);
                if self.position == pos {
                    write!(f, "S")?;
                } else if self.nearest == pos {
                    write!(f, "B")?;
                } else if mht((x as isize, y as isize), self.position)
                    <= mht(self.nearest, self.position)
                {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn mht(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl FromStr for Beacon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .split('=')
            .into_iter()
            .map(|c| {
                c.as_bytes()
                    .iter()
                    .filter(|n| n.is_ascii_digit() || **n == b'-')
                    .map(|n| *n as char)
                    .collect::<String>()
            })
            .filter_map(|n| n.parse::<isize>().ok())
            .collect::<Vec<isize>>();
        Ok(Beacon {
            position: (res[0], res[1]),
            nearest: (res[2], res[3]),
            dist: mht((res[0], res[1]), (res[2], res[3])),
        })
    }
}
