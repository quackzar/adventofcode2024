use std::{array::from_fn, collections::{BTreeMap, BTreeSet}, iter};

use itertools::Itertools;

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve_1(&input);
    println!("Solution 1: {res}");
    let res = solve_2(&input);
    println!("Solution 2: {res}");
}

fn solve_1(input: &str) -> u32 {
    let (mut map, freqs) = parse(input);

    fn antinodes(a: Position, b: Position) -> [Position; 2] {
        let (x0, y0) = a;
        let (x1, y1) = b;
        let dx = x0 - x1;
        let dy = y0 - y1;
        [(x0 - 2*dx, y0 - 2*dy), (x1 + 2*dx, y1 + 2*dy)]
    }

    let &(n, m) = map.keys().max().unwrap();

    let locs: Vec<_> = freqs
        .into_iter()
        .flat_map(|(_, locs)| {
            locs.into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| antinodes(a, b))
        })
        .unique()
        .filter(|(x, y)| (0..n+1).contains(x) && (0..m+1).contains(y))
        .collect();

    for loc in locs.iter() {
        *map.get_mut(loc).unwrap() = '#';
    }

    locs.len() as u32
}


fn solve_2(input: &str) -> u32 {
    let (mut map, freqs) = parse(input);
    let &(n, m) = map.keys().max().unwrap();

    let antinodes =|a: Position, b: Position| {
        let map = &map;
        let (x0, y0) = a;
        let (x1, y1) = b;
        let dx = x0 - x1;
        let dy = y0 - y1;
        iter::successors(Some((x0, y0)), move |&(x,y)| {
            let pos = (x - dx, y - dy);
            if map.contains_key(&pos) {
                Some(pos)
            } else {
                None
            }
        }).chain(iter::successors(Some((x1, y1)), move |&(x,y)| {
            let pos = (x + dx, y + dy);
            if map.contains_key(&pos) {
                Some(pos)
            } else {
                None
            }
        }))
    };


    let locs: Vec<_> = freqs
        .into_iter()
        .flat_map(|(_, locs)| {
            locs.into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| antinodes(a, b))
        })
        .unique()
        .filter(|(x, y)| (0..n+1).contains(x) && (0..m+1).contains(y))
        //.filter(|pos| map.get(pos).cloned() == Some('.'))
        // weird that this is a none-rule but ok
        .collect();

    for loc in locs.iter() {
        *map.get_mut(loc).unwrap() = '#';
    }

    print_map(&map);

    locs.len() as u32
}

fn print_map(map: &Map) {
    let &(n, m) = map.keys().max().unwrap();
    for j in 0..n+1 {
        for i in 0..m+1 {
            let c = map.get(&(i, j)).unwrap();
            print!("{c}")
        }
        println!()
    }
}

type Position = (isize, isize);
type Map = BTreeMap<Position, char>;
type ReverseMap = BTreeMap<char, Vec<Position>>;
fn parse(input: &str) -> (Map, ReverseMap) {
    let map: Map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    let freqs: BTreeSet<char> = map
        .values()
        .cloned()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let freqs = freqs
        .iter()
        .map(|&c| {
            let locs = map
                .iter()
                .filter(|&(_, &f)| f == c)
                .map(|(&k, _)| k)
                .collect();
            (c, locs)
        })
        .collect();

    (map, freqs)
}

#[cfg(test)]
const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[test]
fn test_1() {
    assert_eq!(solve_1(EXAMPLE), 14);
}


#[test]
fn test_2() {
    assert_eq!(solve_2(EXAMPLE), 34);
}
