use std::{collections::VecDeque, error::Error, fs::read_to_string, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    let monks: Vec<Monkey> = input.split("\n\n").filter_map(|s| s.parse().ok()).collect();

    let mut part1 = monks.clone();
    let part1 = solve(&mut part1, 20, None);
    println!("P1 {part1}");

    let divisor = monks
        .iter()
        .map(|m| m.test.divisible)
        .reduce(|acc, x| acc * x);
    let mut part2 = monks;
    let part2 = solve(&mut part2, 10_000, divisor);
    println!("P2: {part2}");
    Ok(())
}

fn solve(monkes: &mut [Monkey], rounds: usize, divisor: Option<usize>) -> usize {
    for _ in 0..rounds {
        for i in 0..monkes.len() {
            while let Some((item, to)) = monkes[i].inspect(divisor) {
                monkes[to].items.push_back(item);
            }
        }
    }
    let mut business: Vec<usize> = monkes.iter().map(|m| m.inspected).collect();
    business.sort();
    business.iter().rev().take(2).product()
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspected: usize,
}

#[derive(Debug, Clone)]
struct Test {
    divisible: usize,
    positive: usize,
    negative: usize,
}

#[derive(Debug, Clone)]
struct Operation {
    op_type: OpType,
    value: usize,
}

#[derive(Debug, Clone)]
enum OpType {
    Plus,
    Mult,
    SelfMult,
}

#[derive(Debug, Clone)]
struct Item {
    worry: usize,
}
impl Monkey {
    fn inspect(&mut self, divisor: Option<usize>) -> Option<(Item, usize)> {
        let Some(mut current) = self.items.pop_front() else { return None;};
        match self.operation.op_type {
            OpType::Plus => current.worry += self.operation.value,
            OpType::Mult => current.worry *= self.operation.value,
            OpType::SelfMult => current.worry *= current.worry,
        };
        self.inspected += 1;

        if let Some(divisor) = divisor {
            current.worry %= divisor;
        } else {
            current.worry /= 3;
        }
        if current.worry % self.test.divisible == 0 {
            return Some((current, self.test.positive));
        }

        Some((current, self.test.negative))
    }
}

fn parse_err() -> &'static str {
    "Monkey parsing failed"
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut next = { || lines.next().ok_or_else(parse_err) };

        let id = parse_id(next()?)?;
        let items = parse_items(next()?)?;
        let operation = parse_operation(next()?)?;
        let divisible = number_at(next()?, "by ")?;
        let positive = number_at(next()?, "monkey ")?;
        let negative = number_at(next()?, "monkey ")?;

        Ok(Monkey {
            id,
            items,
            operation,
            test: Test {
                divisible,
                positive,
                negative,
            },
            inspected: 0,
        })
    }
}

fn parse_id(s: &str) -> Result<usize, &'static str> {
    s.split_once(':')
        .and_then(|(l, _)| l.rsplit_once(' '))
        .and_then(|(_, r)| r.parse::<usize>().ok())
        .ok_or_else(parse_err)
}

fn number_at(s: &str, at: &str) -> Result<usize, &'static str> {
    s.split_once(at)
        .and_then(|(_, r)| r.parse::<usize>().ok())
        .ok_or_else(parse_err)
}
fn parse_operation(s: &str) -> Result<Operation, &'static str> {
    s.split_once("old ")
        .and_then(|(_, r)| r.split_once(' '))
        .and_then(|(l, r)| {
            let Some(val) = r.parse::<usize>().ok() else {
                return Some((OpType::SelfMult, 0));
            };
            let ops = match l {
                "*" => Some(OpType::Mult),
                "+" => Some(OpType::Plus),
                _ => None,
            };
            ops.zip(Some(val))
        })
        .map(|(op_type, value)| Operation { op_type, value })
        .ok_or_else(parse_err)
}

fn parse_items(s: &str) -> Result<VecDeque<Item>, &'static str> {
    s.split_once(':')
        .map(|(_, r)| {
            r.split(',')
                .filter_map(|n| n.trim().parse::<usize>().ok())
                .map(|worry| Item { worry })
                .collect::<VecDeque<Item>>()
        })
        .ok_or_else(parse_err)
}
