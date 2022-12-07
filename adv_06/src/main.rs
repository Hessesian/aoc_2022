use std::{fs::read_to_string, ops::BitOr};

fn main() {
    let input = read_to_string("bigboy.txt").unwrap();

    println!("{}", unique(&input, 4));
    println!("{}", unique(&input, 14));
}

fn unique(input: &str, len: usize) -> usize {
    let mut mask = 0u32;
    let last = 0u32;
    'main: for (i, c) in input
        .as_bytes()
        .iter()
        .map(|c| 1u32 << (*c - b'a'))
        .enumerate()
    {
        mask |= c;
        println!("added {}", input.as_bytes()[i] as char);
        if len <= i +1{
            println!("ones {}", mask.count_ones());
            if mask.count_ones() as usize == len {
                return i+1;
            } else {
                mask &= !(1u32 << (input.as_bytes()[i + 1 - len] - b'a'));
                println!("removed {}", input.as_bytes()[i + 1 - len] as char);
            }
        }
    }
    0
}

#[test]
fn test_samples() {
    assert_eq!(5, unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, unique("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));

    assert_eq!(19, unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, unique("nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
}

#[test]
fn test_bits(){
    let mut mask = 0u32;
    mask |= 1u32 << (b'b' - b'a');
    mask |= 1u32 << (b'v' - b'a');
    mask |= 1u32 << (b'w' - b'a');
    mask |= 1u32 << (b'b' - b'a');
    mask |= 1u32 << (b'j' - b'a');
    mask &= !(1u32 << (b'b' - b'a'));

    assert_eq!(mask.count_ones(), 4);
}
