#![feature(let_chains)]
use std::{borrow::{Borrow, Cow}, collections::{BTreeMap, BTreeSet}, rc::Rc};


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
    let map = parse(input);
    let trailheads: Vec<_> = map
        .iter()
        .filter(|&(_pos, height)| *height == 0)
        .map(|(pos, _)| (*pos))
        .collect();

    let mut total_scores = 0;
    for start in trailheads {
        let (score ,_) = score(start, &map);
        total_scores += score;
    }

    total_scores
}

fn score(start: (isize, isize), map: &BTreeMap<(isize, isize), u8>) -> (u64, u64) {
    let mut tops = BTreeSet::new();
    let mut trails = BTreeSet::new();

    let mut stack = Vec::new();
    let initial = Rc::new(vec![start]);
    stack.push(initial);
    while let Some(mut trail) = stack.pop() {
        let (x, y ) = *trail.last().unwrap();
        let current_height = map.get(&(x, y)).unwrap();
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let pos = (x + dx, y + dy);
            if let Some(&h) = map.get(&pos) && h == current_height + 1 {
                Rc::make_mut(&mut trail).push(pos);
                if h == 9 {
                    tops.insert(pos);
                    trails.insert(trail.clone());
                } else {
                    stack.push(trail.clone());
                }
            }
        }
    }
    (tops.len() as u64, trails.len() as u64)
}



fn solve_2(input: &str) -> u64 {
    let map = parse(input);
    let trailheads: Vec<_> = map
        .iter()
        .filter(|&(_pos, height)| *height == 0)
        .map(|(pos, _)| (*pos))
        .collect();

    let mut total_ratings = 0;
    for start in trailheads {
        let (_, rating) = score(start, &map);
        total_ratings += rating;
    }

    total_ratings
}

type Position = (isize, isize);
type Map = BTreeMap<Position, u8>;


fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let height = c.to_digit(10).unwrap() as u8;
                let pos = (x as isize, y as isize);
                (pos, height)
            })
        })
        .collect()
}

#[cfg(test)]
const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[test]
fn part1() {
    let res = solve_1(EXAMPLE);
    assert_eq!(res, 36);
}

#[test]
fn part2() {
    let res = solve_2(EXAMPLE);
    assert_eq!(res, 81);
}
