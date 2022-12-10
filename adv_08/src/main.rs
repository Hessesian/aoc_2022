use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("bigboy.txt")?;
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.trim_end().as_bytes().iter().fold(vec![], |mut acc, x| {
                acc.push(x - b'0');
                acc
            })
        })
        .collect();

    let (width, height) = (grid[0].len(), grid.len());
    let mut visibility: Vec<Vec<u8>> = vec![vec![0; width]; height];

    calc_height(height, width, &grid, &mut visibility);
    calc_width(height, width, &grid, &mut visibility);

    println!(
        "{:?}",
        visibility.iter().flatten().filter(|x| **x != 4).count()
    );
    let mut scenic: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for x in 0..width {
        for y in 0..height {
            calc_scenic(x, y, &mut scenic, &grid, &visibility);
        }
    }
    println!("{:?}", scenic.iter().flatten().max());
    Ok(())
}

fn calc_scenic(
    x: usize,
    y: usize,
    scenic: &mut [Vec<usize>],
    grid: &[Vec<u8>],
    visibility: &[Vec<u8>],
) {
    if visibility[x][y] == 4 {
        return;
    }
    let max_val = grid[x][y];
    let (width, height) = (grid[0].len(), grid.len());
    let mut count = 0;
    for xx in (x + 1)..width {
        count += 1;
        if grid[xx][y] >= max_val {
            break;
        }
    }
    let mut count2 = 0;
    for xx in 1..=x {
        count2 += 1;
        if grid[x - xx][y] >= max_val {
            break;
        }
    }
    let mut count3 = 0;
    for yy in (y + 1)..height {
        count3 += 1;
        if grid[x][yy] >= max_val {
            break;
        }
    }
    let mut count4 = 0;
    for yy in 1..=y {
        count4 += 1;
        if grid[x][y - yy] >= max_val {
            break;
        }
    }
    scenic[x][y] = count * count2 * count3 * count4;
}

fn calc_height(height: usize, width: usize, grid: &Vec<Vec<u8>>, visibility: &mut Vec<Vec<u8>>) {
    for y in 0..height {
        let (mut left_max, mut right_max) = (-1i8, -1i8);
        for x in 0..width {
            let left = grid[x][y] as i8;
            if left > left_max {
                left_max = left;
            } else if left <= left_max {
                visibility[x][y] += 1;
            }

            let right = grid[width - x - 1][y] as i8;
            if right > right_max {
                right_max = right;
            } else if right <= right_max {
                visibility[width - x - 1][y] += 1;
            }
        }
    }
}

fn calc_width(height: usize, width: usize, grid: &Vec<Vec<u8>>, visibility: &mut Vec<Vec<u8>>) {
    for x in 0..width {
        let (mut left_max, mut right_max) = (-1i8, -1i8);
        for y in 0..height {
            let left = grid[x][y] as i8;
            if left > left_max {
                left_max = left;
            } else if left <= left_max {
                visibility[x][y] += 1;
            }

            let right = grid[x][height - y - 1] as i8;
            if right > right_max {
                right_max = right;
            } else if right <= right_max {
                visibility[x][height - y - 1] += 1;
            }
        }
    }
}
