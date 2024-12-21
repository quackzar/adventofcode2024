use std::{fmt::Display, u16};

use nonmax::NonMaxU16;

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve_1(&input);
    println!("Solution 1: {res}");
    let res = solve_2(&input);
    println!("Solution 2: {res}");
}

fn solve_1(input: &str) -> u64 {
    let mut disk = parse(input);
    println!("{disk}");
    compact(&mut disk);
    println!("{disk}");
    checksum(&disk)
}

fn solve_2(input: &str) -> u64 {
    let disk = parse(input);
    println!("{disk}");
    let mut rich = enrich(&disk);
    defragment(&mut rich);
    let disk = impoverish(&rich);
    println!("{disk}");
    checksum(&disk)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Slot {
    Used(NonMaxU16),
    Free,
}

struct Disk {
    data: Box<[Slot]>,
}

fn parse(input: &str) -> Disk {
    let mut chars = input.chars();
    let mut disk: Vec<Slot> = Vec::new();
    let mut id = 0;
    while let (Some(used), Some(free)) = (chars.next(), chars.next()) {
        let used = used.to_digit(10).expect("should be digit");
        let free = free.to_digit(10).expect("should be digit");
        let file = NonMaxU16::new(id).unwrap();
        disk.extend((0..used).map(|_| Slot::Used(file)));
        disk.extend((0..free).map(|_| Slot::Free));
        id += 1;
    }

    // input is uneven :(
    if let Some(c) = input.chars().last() {
        let used = c.to_digit(10).expect("should be digit");
        let file = NonMaxU16::new(id).unwrap();
        disk.extend((0..used).map(|_| Slot::Used(file)));
    }

    Disk {
        data: disk.into_boxed_slice(),
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.data {
            match c {
                Slot::Used(c) => write!(f, "{c}")?,
                Slot::Free => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

fn compact(disk: &mut Disk) {
    let disk = &mut disk.data;
    let mut needle = 0;
    let mut last = disk.len() - 1;

    loop {
        if needle == last {
            break;
        }
        match disk.get(needle) {
            Some(Slot::Used(_)) => {
                needle += 1;
                continue;
            }
            Some(Slot::Free) => {
                let file = 'inner: loop {
                    if let Slot::Used(file) = disk[last] {
                        break 'inner file;
                    }
                    last -= 1;
                };
                disk[needle] = Slot::Used(file);
                disk[last] = Slot::Free;
                last -= 1;
            }
            None => break,
        }
    }
}

#[derive(Clone, Copy)]
struct Segment {
    size: u16,
    file: Slot,
}

struct RichDisk { // this is almost just the compact format again
    data: Vec<Segment>,
}

fn enrich(disk: &Disk) -> RichDisk {
    let mut size = 1;
    let mut richdisk = Vec::new();
    let mut data = disk.data.iter();
    let mut file = *data.next().unwrap();

    for slot in data {
        if *slot == file {
            size += 1;
        } else {
            richdisk.push(Segment { size, file });
            size = 1;
            file = *slot;
        }
    }
    richdisk.push(Segment { size, file });

    RichDisk { data: richdisk }
}

fn impoverish(rich: &RichDisk) -> Disk {
    let mut disk = Vec::new();
    for &Segment { size, file } in &rich.data {
        disk.extend((0..size).map(|_| file));
    }
    Disk {
        data: disk.into_boxed_slice(),
    }
}

fn defragment(disk: &mut RichDisk) {
    let disk = &mut disk.data;
    let mut needle = 0;

    'outer: loop {
        match disk.get(needle) {
            Some(Segment {
                size,
                file: Slot::Free,
            }) => {
                let freespace = *size;
                let mut last = disk.len();
                let segment = 'inner: loop {
                    last -= 1;
                    if last <= needle { needle += 1; continue 'outer;}
                    match disk[last] {
                        segment @ Segment {
                            size,
                            file: Slot::Used(_),
                        } if size <= freespace => break 'inner segment,
                        _ => (),
                    };
                };
                disk[needle] = segment;
                disk[last] = Segment { file: Slot::Free, ..segment};
                let newfree = freespace - segment.size;
                if newfree > 0 {
                    disk.insert(needle, Segment {
                        size: newfree,
                        file: Slot::Free,
                    });
                }
            }
            Some(Segment {
                size: _,
                file: Slot::Used(_),
            }) => {
                needle += 1;
                continue;
            }
            None => break,
        }
    }
}

fn checksum(disk: &Disk) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    while let Some(slot) = disk.data.get(i) {
        if let Slot::Used(file) = slot {
            let file: u16 = (*file).into();
            sum += (file as u64) * i as u64;
        }
        i += 1;
    }
    sum
}

#[cfg(test)]
const EXAMPLE: &str = "2333133121414131402";

#[test]
fn test_1() {
    assert_eq!(solve_1(EXAMPLE), 1928);
}

#[test]
fn test_2() {
    assert_eq!(solve_2(EXAMPLE), 2858);
}
