use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Type {
    Rock,
    Paper,
    Scissors,
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (p1, p2) = input
        .lines()
        .map(|g| score(&from_str(g)))
        .reduce(|mut acc, val| {
            acc.0 += val.0;
            acc.1 += val.1;
            acc
        })
        .unwrap();
    println!("Score: {} x {}", p1, p2);
    let (p1, p2) = input
        .lines()
        .map(|g| score(&from_str2(g)))
        .reduce(|mut acc, val| {
            acc.0 += val.0;
            acc.1 += val.1;
            acc
        })
        .unwrap();
    println!("Score: {} x {}", p1, p2);
}

fn score(round: &(Type, Type)) -> (usize, usize) {
    let (win, lose, draw) = (6, 0, 3);
    let (p1, p2) = match round.0 {
        Type::Rock => match round.1 {
            Type::Rock => (draw, draw),
            Type::Paper => (lose, win),
            Type::Scissors => (win, lose),
        },
        Type::Paper => match round.1 {
            Type::Rock => (win, lose),
            Type::Paper => (draw, draw),
            Type::Scissors => (lose, win),
        },
        Type::Scissors => match round.1 {
            Type::Rock => (lose, win),
            Type::Paper => (win, lose),
            Type::Scissors => (draw, draw),
        },
    };

    (p1 + round.0.val(), p2 + round.1.val())
}

impl Type {
    fn val(&self) -> usize {
        match self {
            Type::Rock => 1,
            Type::Paper => 2,
            Type::Scissors => 3,
        }
    }

    fn from_p1(letter: &u8) -> Self {
        match letter {
            b'A' => Type::Rock,
            b'B' => Type::Paper,
            b'C' => Type::Scissors,
            _ => panic!("Bork"),
        }
    }

    fn from_p2(letter: &u8) -> Self {
        match letter {
            b'X' => Type::Rock,
            b'Y' => Type::Paper,
            b'Z' => Type::Scissors,
            _ => panic!("Bork"),
        }
    }

    fn from_result(&self, result: &u8) -> Self {
        match self {
            Type::Rock => match result {
                b'X' => Type::Scissors,
                b'Y' => Type::Rock,
                b'Z' => Type::Paper,
                _ => panic!("Bork"),
            },
            Type::Paper => match result {
                b'X' => Type::Rock,
                b'Y' => Type::Paper,
                b'Z' => Type::Scissors,
                _ => panic!("Bork"),
            },
            Type::Scissors => match result {
                b'X' => Type::Paper,
                b'Y' => Type::Scissors,
                b'Z' => Type::Rock,
                _ => panic!("Bork"),
            },
        }
    }
}

fn from_str(line: &str) -> (Type, Type) {
    if let Some((l, r)) = line.split_once(' ') {
        (
            Type::from_p1(&l.as_bytes()[0]),
            Type::from_p2(&r.as_bytes()[0]),
        )
    } else {
        panic!("bork");
    }
}
fn from_str2(line: &str) -> (Type, Type) {
    if let Some((l, r)) = line.split_once(' ') {
        let left = Type::from_p1(&l.as_bytes()[0]);
        (left, left.from_result(&r.as_bytes()[0]))
    } else {
        panic!("bork");
    }
}
