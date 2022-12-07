use std::{collections::VecDeque, fs::read_to_string, str::FromStr, time::Instant};

#[derive(Debug, Default, Clone)]
struct Column {
    crates: VecDeque<char>,
}

#[derive(Debug, Default, Clone)]
struct CraneCommand {
    number: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Ship {
    cargo: Vec<Column>,
    scratch: VecDeque<char>,
}
impl Ship {
    fn execute(&mut self, c: &CraneCommand, is_9001: bool) {
        let from = &mut self.cargo[c.from].crates;
        let drain = from.drain(0..c.number);
        if is_9001 {
            self.scratch.extend(drain);
        } else {
            self.scratch.extend(drain.rev());
        }
        let dest = &mut self.cargo[c.to].crates;
        self.scratch.append(dest);
        dest.append(&mut self.scratch);
    }

    fn top_crates(&self) -> String {
        self.cargo.iter().flat_map(|c| c.crates.front()).collect()
    }
}

#[derive(Debug)]
enum Error {
    ShipParse,
    Input,
}

fn main() -> Result<(), Error> {
    let input = read_to_string("okDo.txt/bigboy.txt").map_err(|_| Error::Input)?;
    let now = Instant::now();
    let ship = input.parse::<Ship>()?;
    let commands: Vec<CraneCommand> = input
        .lines()
        .skip_while(|l| l.contains("\n\n"))
        .flat_map(|l| l.parse::<CraneCommand>().ok())
        .collect();

    let mut ship_1 = ship.clone();
    commands.iter().for_each(|c| ship_1.execute(c, false));
    println!(
        "Part 1: {} in {:?}",
        ship_1.top_crates(),
        Instant::now() - now
    );

    let mut ship_2 = ship;
    commands.iter().for_each(|c| ship_2.execute(c, true));
    println!(
        "Part 2: {} in {:?}",
        ship_2.top_crates(),
        Instant::now() - now
    );
    Ok(())
}

impl FromStr for Ship {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns: Vec<Column> = vec![Column::default(); 200];
        s.lines()
            .take_while(|l| l.chars().any(|c| !c.is_numeric()))
            .for_each(|l| {
                l.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if (i + 3) % 4 == 0 { Some(c) } else { None })
                    .enumerate()
                    .for_each(|(i, c)| {
                        if let 'A'..='Z' = c {
                            columns[i].crates.push_back(c)
                        }
                    });
            });
        Ok(Ship {
            cargo: columns,
            scratch: VecDeque::new(),
        })
    }
}

impl FromStr for CraneCommand {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<usize> = s
            .split_ascii_whitespace()
            .enumerate()
            .flat_map(|(i, c)| {
                if i % 2 != 0 {
                    c.parse::<usize>().ok()
                } else {
                    None
                }
            })
            .collect();
        if vals.len() != 3 {
            Err(Error::ShipParse)
        } else {
            Ok(CraneCommand {
                number: vals[0],
                from: vals[1] - 1,
                to: vals[2] - 1,
            })
        }
    }
}
