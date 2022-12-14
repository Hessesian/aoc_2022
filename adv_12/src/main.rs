use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::read_to_string,
};
const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;

    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut starts: Vec<(usize, usize)> = vec![];
    let mut gr = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.trim_end()
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    b'S' => {
                        start = (x, y);
                        Node {
                            position: (x, y),
                            neighbours: HashSet::new(),
                            value: 0,
                        }
                    }
                    b'E' => {
                        end = (x, y);
                        Node {
                            position: (x, y),
                            neighbours: HashSet::new(),
                            value: b'z' - b'a',
                        }
                    }
                    b'a'..=b'z' => {
                        if *c == b'a' {
                            starts.push((x, y));
                        }

                        Node {
                            position: (x, y),
                            neighbours: HashSet::new(),
                            value: c - b'a',
                        }
                    }
                    _ => panic!("bork"),
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    println!("{:?}, {:?}", start, end);

    calculate_edges(start, &mut gr);

    let dist = djikstra(start, &gr, end);
    println!("P1: {}", dist);
    let mut distances: Vec<usize> = vec![];
    for low in starts {
        let dist = djikstra(low, &gr, end);
        distances.push(dist);
    }

    println!("P2: {}", distances.iter().min().unwrap());
    Ok(())
}

fn djikstra(start: (usize, usize), gr: &Vec<Vec<Node>>, end: (usize, usize)) -> usize {
    let mut buffer: VecDeque<(usize, usize)> = VecDeque::new();
    buffer.push_front(start);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let max = usize::MAX;
    let mut dist = vec![vec![max; gr.len()]; gr[0].len()];
    let mut max_dist = usize::MAX;
    while !buffer.is_empty() {
        let Some(current_pos) = buffer.pop_front() else  {break;};
        if current_pos.1 == start.1 && current_pos.0 == start.0 {
            dist[current_pos.0][current_pos.1] = 0;
        }
        let current_dist = dist[current_pos.0][current_pos.1];
        if current_pos == end {
            max_dist = current_dist;
        }
        if current_dist > max_dist || visited.contains(&current_pos) {
            continue;
        }
        let current = &gr[current_pos.1][current_pos.0];
        for neighs in current.neighbours.iter() {
            let node = &gr[neighs.1][neighs.0];
            let dstnc = dist[node.position.0][node.position.1];
            if dstnc > current_dist {
                dist[neighs.0][neighs.1] = current_dist + 1;
                buffer.push_back(*neighs);
                visited.remove(neighs);
            }
        }
        visited.insert(current_pos);
    }
    dist[end.0][end.1]
}

fn calculate_edges(start: (usize, usize), gr: &mut [Vec<Node>]) {
    let mut buffer: VecDeque<(usize, usize)> = VecDeque::new();
    buffer.push_front(start);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !buffer.is_empty() {
        let Some(current_pos) = buffer.pop_front() else  {break;};
        for (x, y) in MOVES.iter() {
            let current = &gr[current_pos.1][current_pos.0];
            let (newx, newy) = (
                current.position.0 as isize + x,
                current.position.1 as isize + y,
            );
            let mut positions = vec![];
            {
                if let Some(node) = gr.get(newy as usize).and_then(|l| l.get(newx as usize)) {
                    if current.value + 1 >= node.value {
                        positions.push(node.position);
                    }
                    if !visited.contains(&node.position) {
                        buffer.push_front(node.position);
                    }
                }
            }
            let current = &mut gr[current_pos.1][current_pos.0];
            for pos in positions {
                if !current.neighbours.contains(&pos) {
                    current.neighbours.insert(pos);
                }
            }
            visited.insert(current.position);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    position: (usize, usize),
    neighbours: HashSet<(usize, usize)>,
    value: u8,
}
