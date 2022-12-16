use std::{
    collections::HashSet,
    error::Error,
    fmt::{write, Display},
    fs::read_to_string,
    str::FromStr,
};

type AppErr = Box<dyn Error>;

fn main() -> Result<(), AppErr> {
    let input = read_to_string("sample")?;
    let (mut width, mut height) = (0, 0);
    let walls: HashSet<Point> = {
        let mut walls = HashSet::new();

        let wall_points: Vec<Wall> = input.lines().filter_map(|l| l.parse().ok()).collect();
        for wall in wall_points {
            wall.path.iter().reduce(|last, new| {
                width = new.x.max(width);
                height = new.y.max(height);
                for x in last.x.min(new.x)..=new.x.max(last.x) {
                    for y in last.y.min(new.y)..=new.y.max(last.y) {
                        walls.insert(Point { x, y });
                    }
                }
                new
            });
        }

        walls
    };

    let sand: HashSet<Point> = HashSet::new();
    let mut world = World {
        floor: false,
        origin: Point { x: 500, y: 0 },
        sand,
        walls,
        max_depth: height,
    };

    println!("P1: {}", world.clone().spawn());
    world.floor = true;
    println!("P2: {}", world.spawn() + 1);

    Ok(())
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 490..510 {
                if self.walls.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else if self.sand.contains(&Point { x, y }) {
                    write!(f, "o")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, " ")
    }
}

impl Point {
    fn fall(&self, world: &World) -> Option<Self> {
        let new_x = self.x;
        for y in self.y.. {
            if world.is_blocked(new_x, y + 1) {
                if !world.is_blocked(new_x - 1, y + 1) {
                    return Point { x: new_x - 1, y: y }.fall(world);
                } else if !world.is_blocked(new_x + 1, y + 1) {
                    return Point { x: new_x + 1, y: y }.fall(world);
                } else if world.origin.x == new_x && world.origin.y == y {
                    return None;
                } else {
                    return Some(Point { x: new_x, y });
                }
            }
            if y > world.max_depth + 10 {
                return None;
            }
        }
        None
    }
}
impl World {
    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.walls.contains(&Point { x, y })
            || self.sand.contains(&Point { x, y })
            || (self.floor && self.max_depth + 2 == y)
    }

    fn spawn(&mut self) -> usize {
        while let Some(landing) = self.origin.fall(self) {
            self.sand.insert(landing);
        }
        self.sand.len()
    }
}

#[derive(Clone)]
struct World {
    floor: bool,
    origin: Point,
    walls: HashSet<Point>,
    sand: HashSet<Point>,
    max_depth: usize,
}

fn err_str(error: impl Error) -> &'static str {
    "Borken :("
}

#[derive(Debug)]
struct Wall {
    path: Vec<Point>,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((left, right)) = s.split_once(',') else {return Err("Bork point")};

        Ok(Point {
            x: left.parse().map_err(err_str)?,
            y: right.parse().map_err(err_str)?,
        })
    }
}

impl FromStr for Wall {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Point> = s.split(" -> ").filter_map(|p| p.parse().ok()).collect();
        Ok(Wall { path: data })
    }
}
