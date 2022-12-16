use std::{
    cmp::Ordering, error::Error, fmt::Display, fs::read_to_string, ops::ControlFlow, str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;

    let data = input
        .split("\n\n")
        .flat_map(|s| {
            s.split_once('\n')
                .and_then(|(l, r)| l.parse::<Packet>().ok().zip(r.parse::<Packet>().ok()))
        })
        .collect::<Vec<(Packet, Packet)>>();

    let mut checksum = 0;
    for (i, (l, r)) in data.iter().enumerate() {
        if l < r {
            checksum += i + 1;
        }
        println!("{} < {} = {}", l, r, l < r);
    }

    println!("P1: {}", checksum);
    let mut part2: Vec<Packet> = data.into_iter().flat_map(|(l, r)| vec![l, r]).collect();
    let (div2, div6) = ("[[2]]".parse::<Packet>()?, "[[6]]".parse::<Packet>()?);
    part2.push(div2.clone());
    part2.push(div6.clone());

    part2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut solution: Vec<usize> = vec![];
    for (index, packet) in part2.iter().enumerate() {
        if *packet == div2 || *packet == div6 {
            solution.push(index + 1);
        }
    }
    println!("Part2: {}", solution.iter().product::<usize>());
    Ok(())
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut this = self.clone().into_iter();
        let mut other = other.clone().into_iter();

        let mut buf: Vec<PacketType> = vec![];
        let mut left = {
            |conv: Option<usize>| {
                if let Some(num) = conv {
                    buf.push(PacketType::ListEnd);
                    buf.push(PacketType::Integer(num));
                    Some(PacketType::ListStart)
                } else if let Some(num) = buf.pop() {
                    Some(num)
                } else {
                    this.next()
                }
            }
        };
        let mut buf2: Vec<PacketType> = vec![];
        let mut right = {
            |conv: Option<usize>| {
                if let Some(num) = conv {
                    buf2.push(PacketType::ListEnd);
                    buf2.push(PacketType::Integer(num));
                    Some(PacketType::ListStart)
                } else if let Some(num) = buf2.pop() {
                    Some(num)
                } else {
                    other.next()
                }
            }
        };

        let mut cleft = left(None);
        let mut cright = right(None);
        loop {
            let res = compare_packets(cleft.clone(), cright.clone());
            if let Err(flow) = res {
                match flow {
                    ControlFlow::Continue(_) => {
                        cleft = left(None);
                        cright = right(None);
                    }
                    ControlFlow::Break(which) => match which {
                        ContType::Left(package) => {
                            cleft = left(package);
                        }
                        ContType::Right(package) => {
                            cright = right(package);
                        }
                    },
                }
            } else if let Ok(order) = res {
                return order;
            }
        }
    }
}

enum ContType {
    Left(Option<usize>),
    Right(Option<usize>),
}
fn compare_packets(
    left: Option<PacketType>,
    right: Option<PacketType>,
) -> Result<Option<Ordering>, ControlFlow<ContType>> {
    match (left, right) {
        (Some(PacketType::ListStart), Some(PacketType::ListStart)) => {
            Err(ControlFlow::Continue(()))
        }
        (Some(PacketType::Integer(ln)), Some(PacketType::Integer(rn))) if ln == rn => {
            Err(ControlFlow::Continue(()))
        }
        (Some(PacketType::Integer(ln)), Some(PacketType::Integer(rn))) if ln < rn => {
            Ok(Some(std::cmp::Ordering::Less))
        }
        (Some(PacketType::Integer(ln)), Some(PacketType::Integer(rn))) if ln > rn => {
            Ok(Some(std::cmp::Ordering::Greater))
        }
        (Some(PacketType::Integer(ln)), Some(PacketType::ListStart)) => {
            Err(ControlFlow::Break(ContType::Left(Some(ln))))
        }
        (Some(PacketType::ListStart), Some(PacketType::Integer(ln))) => {
            Err(ControlFlow::Break(ContType::Right(Some(ln))))
        }
        (Some(PacketType::Integer(_)), Some(PacketType::ListEnd)) => Ok(Some(Ordering::Greater)),
        (Some(PacketType::ListEnd), Some(PacketType::Integer(_))) => Ok(Some(Ordering::Less)),
        (Some(PacketType::ListStart), Some(PacketType::ListEnd)) => Ok(Some(Ordering::Greater)),
        (Some(PacketType::ListEnd), Some(PacketType::ListStart)) => Ok(Some(Ordering::Less)),
        (None, Some(_)) => Ok(Some(Ordering::Less)),
        (Some(_), None) => Ok(Some(Ordering::Greater)),
        (None, None) => Ok(Some(Ordering::Greater)),
        _ => Err(ControlFlow::Continue(())),
    }
}

impl Display for PacketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketType::ListStart => write!(f, "["),
            PacketType::ListEnd => write!(f, "]"),
            PacketType::Integer(n) => write!(f, "{},", n),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in self.clone().into_iter() {
            write!(f, "{}", t)?;
        }
        write!(f, "")
    }
}

impl Iterator for PacketIntoIterator {
    type Item = PacketType;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.packet.value.get(self.index) else {return None;};
        let res = match current.sym_type {
            Ob => Some(PacketType::ListStart),
            Cb => Some(PacketType::ListEnd),
            Integer(num) => Some(PacketType::Integer(num)),
        };
        self.index += 1;
        res
    }
}

impl IntoIterator for Packet {
    type Item = PacketType;

    type IntoIter = PacketIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PacketIntoIterator {
            index: 0,
            packet: self,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Packet {
    value: Vec<Symbol>,
}

#[derive(Debug)]
struct PacketIntoIterator {
    packet: Packet,
    index: usize,
}

#[derive(Debug, Clone)]
enum PacketType {
    ListStart,
    ListEnd,
    Integer(usize),
}

#[derive(Debug, Clone, PartialEq)]
struct Symbol {
    sym_type: SymbolType,
    level: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolType {
    Ob,
    Cb,
    Integer(usize),
}

use SymbolType::*;
impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut level = 0;
        let mut string = String::new();
        let mut packets = vec![];
        s.as_bytes().iter().for_each(|c| match c {
            b'[' => {
                packets.push(Symbol {
                    level,
                    sym_type: Ob,
                });
                level += 1;
            }
            b']' => {
                if let Ok(num) = string.parse::<usize>() {
                    string.clear();
                    packets.push(Symbol {
                        level,
                        sym_type: Integer(num),
                    });
                }
                packets.push(Symbol {
                    level,
                    sym_type: Cb,
                });
                level -= 1;
            }
            b',' => {
                if let Ok(num) = string.parse::<usize>() {
                    string.clear();
                    packets.push(Symbol {
                        level,
                        sym_type: Integer(num),
                    });
                }
            }
            b'0'..=b'9' => {
                string.push(*c as char);
            }
            _ => {}
        });
        Ok(Packet { value: packets })
    }
}
