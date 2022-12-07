use std::{collections::VecDeque, error::Error, fs::read_to_string, str::FromStr};

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    size: usize,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
enum FileType {
    File(File),
    Dir(Dir),
}

#[derive(Debug, Clone)]
enum Cmds {
    Cd(String),
    Ls(Vec<FileType>),
}

#[derive(Debug, Clone)]
struct Disk {
    root: Dir,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;

    let cmds: Vec<Cmds> = input
        .split("\n$ ")
        .flat_map(|l| l.parse::<Cmds>().ok())
        .collect();
    let mut disk = Disk::new();
    disk.populate(&cmds);
    disk.root.populate_size();
    println!("Part1: {}", disk.root.size_max(100_000));
    let min_size = 30_000_000 - (70_000_000 - disk.root.size);
    println!(
        "Part2: {} for needed {min_size}",
        disk.root.at_least(min_size)
    );
    Ok(())
}

impl Disk {
    fn populate(&mut self, cmds: &[Cmds]) {
        let mut current_path: VecDeque<String> = VecDeque::new();
        for cmd in cmds.iter() {
            match cmd {
                Cmds::Cd(path) => {
                    if path == ".." {
                        current_path.pop_front();
                    } else {
                        current_path.push_front(path.into());
                    }
                }
                Cmds::Ls(files) => {
                    let slice = current_path.make_contiguous();
                    let dir = &mut self.get_dir(slice);
                    for file in files {
                        match file {
                            FileType::File(f) => dir.files.push(f.clone()),
                            FileType::Dir(d) => dir.dirs.push(d.clone()),
                        }
                    }
                }
            }
        }
    }

    fn get_dir(&mut self, path: &[String]) -> &mut Dir {
        let mut current = &mut self.root;

        for name in path.iter().rev() {
            current = current.dirs.iter_mut().find(|d| d.name == **name).unwrap();
        }
        current
    }

    pub(crate) fn new() -> Self {
        Disk {
            root: Dir {
                name: "/".into(),
                size: 0,
                files: vec![],
                dirs: vec![],
            },
        }
    }
}

impl FromStr for Cmds {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("cd") {
            let Some((_left, right)) = s.split_once(' ') else { return Err("Fail CD");};
            Ok(Cmds::Cd(right.into()))
        } else {
            let Some((_left, right)) = s.split_once('\n') else { return Err("Fail LS");};
            let files: Vec<FileType> = right
                .lines()
                .flat_map(|l| l.parse::<FileType>().ok())
                .collect();
            Ok(Cmds::Ls(files))
        }
    }
}

impl FromStr for FileType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((left, right)) = s.split_once(' ') else {return Err("Bad file type");};
        if let "dir" = left {
            Ok(FileType::Dir(Dir::from_name(right)))
        } else {
            Ok(FileType::File(File {
                name: right.into(),
                size: left.parse().map_err(|_| "Bad file size")?,
            }))
        }
    }
}

impl Dir {
    fn from_name(name: &str) -> Self {
        Dir {
            dirs: vec![],
            files: vec![],
            size: 0,
            name: name.into(),
        }
    }

    fn populate_size(&mut self) -> usize {
        let mut size = self.files.iter().map(|f| f.size).sum();
        size += self
            .dirs
            .iter_mut()
            .map(|d| d.populate_size())
            .sum::<usize>();
        self.size = size;
        size
    }

    fn size_max(&self, max_size: usize) -> usize {
        let children = self
            .dirs
            .iter()
            .fold(0, |acc, d| acc + d.size_max(max_size));
        if self.size > max_size {
            children
        } else {
            self.size + children
        }
    }

    fn at_least(&self, min_size: usize) -> usize {
        let best_match = self.dirs.iter().fold(self.size, |acc, d| {
            let child = d.at_least(min_size);
            if acc >= child && child >= min_size {
                child
            } else {
                acc
            }
        });
        best_match
    }
}

impl File {}

#[test]
fn test_sample() {
    let input = read_to_string("sample").unwrap();
    let cmds: Vec<Cmds> = input
        .split("\n$ ")
        .flat_map(|l| l.parse::<Cmds>().ok())
        .collect();
    let mut disk = Disk::new();
    disk.populate(&cmds);
    disk.root.populate_size();
    assert_eq!(95437, disk.root.size_max(100000));
    assert_eq!(disk.root.size, 48381165);
    let min_size = 30_000_000 - (70_000_000 - disk.root.size);
    assert_eq!(8381165, min_size);
}
