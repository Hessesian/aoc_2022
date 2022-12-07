use std::{fs::read_to_string, num::ParseIntError, str::FromStr};
#[derive(Debug)]
enum AppErrors {
    ParseRange(ParseIntError),
    ParseLine,
    NoInput(std::io::Error),
}

impl AppErrors {
    fn from_parse_int(err: ParseIntError) -> Self {
        AppErrors::ParseRange(err)
    }
    fn from_io(err: std::io::Error) -> Self {
        AppErrors::NoInput(err)
    }
}

struct Rng {
    left: i32,
    right: i32,
}

fn main() -> Result<(), AppErrors> {
    let input = read_to_string("input").map_err(AppErrors::from_io)?;

    let vals = input
        .lines()
        .flat_map(|l| match l.split_once(',') {
            Some((left, right)) => left
                .parse::<Rng>()
                .and_then(|v| right.parse::<Rng>().map(|c| (v, c)))
                .ok(),
            _ => None,
        })
        .fold((0, 0), |(mut contain, mut overlap), pair| {
            overlaps(&pair).then(|| overlap += 1);
            contains(&pair).then(|| contain += 1);
            (contain, overlap)
        });

    let (p1, p2) = vals;
    println!("P1: {p1}, P2: {p2}");
    Ok(())
}

impl FromStr for Rng {
    type Err = AppErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once('-') {
            Ok(Rng {
                left: left.parse::<i32>().map_err(AppErrors::from_parse_int)?,
                right: right.parse::<i32>().map_err(AppErrors::from_parse_int)?,
            })
        } else {
            Err(AppErrors::ParseLine)
        }
    }
}

impl Rng {
    fn contains(&self, other: &Rng) -> bool {
        let range = self.left..=self.right;
        range.contains(&other.left) && range.contains(&other.right)
    }

    fn overlaps(&self, other: &Rng) -> bool {
        let range = self.left..=self.right;
        range.contains(&other.left) || range.contains(&other.right)
    }
}

fn contains(pair: &(Rng, Rng)) -> bool {
    pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
}

fn overlaps(pair: &(Rng, Rng)) -> bool {
    pair.0.overlaps(&pair.1) || pair.1.overlaps(&pair.0)
}
