use std::{
    error::Error,
    fmt::Display,
    fs::read_to_string,
    io::{stdout, Write},
    str::FromStr,
    thread,
};

#[derive(Debug)]
enum Cmd {
    Addx(isize),
    Noop,
}
#[derive(Default)]
struct Crt {
    buffer: Vec<isize>,
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..(self.buffer.len() - 1) {
            let Some(rider) = self.buffer.get(x) else {return writeln!(f);};

            let pixel = (x) % 40;
            if pixel == 0 {
                writeln!(f)?;
            }
            let pixel = pixel as isize;
            if pixel > rider - 2 && pixel < rider + 2 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        return writeln!(f);
    }
}

impl Crt {
    fn load_buffer(&mut self, data: &[Cmd]) {
        let mut x = 1;
        self.buffer.push(x);
        for cmd in data.iter() {
            match cmd {
                Cmd::Addx(num) => {
                    self.buffer.push(x);
                    x += num;
                    self.buffer.push(x);
                }
                Cmd::Noop => {
                    self.buffer.push(x);
                }
            }
        }
    }

    fn check_sum(&self) -> usize {
        let mut index: usize = 20;
        let mut sum: isize = 0;
        while let Some(val) = self.buffer.get(index - 1) {
            sum += (index as isize) * val;
            index += 40;
        }
        sum as usize
    }

    fn visualisation(&self) {
        for step in 0..(self.buffer.len() - 1) {
            Self::clear();
            for x in 0..(self.buffer.len()) {
                let x = x as isize;
                let pixel = (x) % 40;
                let line: isize = (x / 40);
                if pixel == 0 {
                    println!();
                }
                let pixel = pixel as isize;
                let Some(rider) = self.buffer.get(x as usize) else {return;};
                let Some(vr) = self.buffer.get(step) else {return;};
                let step = step as isize;
                let vr: isize = (line * 40) + vr;
                if x > step {
                    print!(" ");
                } else if x > vr - 2 && x < vr + 2 {
                    if (step == x) {
                        print!("X");
                    } else {
                        print!("O");
                    }
                } else if pixel > rider - 2 && pixel < rider + 2 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            thread::sleep_ms(150);
            stdout().flush();
        }
        println!();
    }

    fn clear() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    let data: Vec<Cmd> = input.lines().filter_map(|l| l.parse().ok()).collect();
    let mut crt = Crt::default();
    crt.load_buffer(&data);

    let sum = crt.check_sum();

    println!("p1: {sum}");
    print!("{}", crt);

    crt.visualisation();

    Ok(())
}

impl FromStr for Cmd {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0] == b'n' {
            Ok(Cmd::Noop)
        } else {
            let Some((_, right)) = s.split_once(' ') else {return Err("Broken addx");};
            Ok(Cmd::Addx(right.parse().map_err(|_| "Broken num")?))
        }
    }
}
