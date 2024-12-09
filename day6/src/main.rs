use std::{collections::{BTreeMap, BTreeSet}, isize};

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve_1(&input);
    println!("Solution 1: {res}");
    let res = solve_2(&input);
    println!("Solution 2: {res}");
}


fn parse(input: &str) -> BTreeMap<(isize, isize), char> {
     input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            ((x as isize, y as isize), c)
        })
    }).collect()
}

fn solve_1(input: &str) -> u32 {
    let map = parse(input);
    let (pos,_) = map.iter().find(|(pos,c)| **c=='^').unwrap();

    let mut dir =  Direction::Up;
    let mut visited = BTreeSet::new();
    let mut x = pos.0;
    let mut y = pos.1;
    loop {
        visited.insert((x,y));
        match dir {
            Direction::Up => {
                let Some(tile) = map.get(&(x,y-1)) else {break};
                if *tile == '#' {
                    dir = Direction::Right;
                    continue;
                }
                y -= 1;
            },
            Direction::Right => {
                let Some(tile) = map.get(&(x+1,y)) else {break};
                if *tile == '#' {
                    dir = Direction::Down;
                    continue;
                }
                x += 1;
            },
            Direction::Down => {
                let Some(tile) = map.get(&(x,y+1)) else {break};
                if *tile == '#' {
                    dir = Direction::Left;
                    continue;
                }
                y += 1;
            },
            Direction::Left => {
                let Some(tile) = map.get(&(x-1,y)) else {break};
                if *tile == '#' {
                    dir = Direction::Up;
                    continue;
                }
                x -= 1;
            },
        }
    }
    visited.len() as u32
}

fn solve_2(input: &str) -> u32 {
    let map = parse(input);
    let (pos,_) = map.iter().find(|(pos,c)| **c=='^').unwrap();

    let mut dir =  Direction::Up;
    let mut visited = BTreeMap::new();
    let mut x = pos.0;
    let mut y = pos.1;

    let mut revisits = 0;
    loop {
        if let Some(redir) =  visited.get(&(x,y)) {
            if dir == Direction::Left && *redir == Direction::Up ||
                dir == Direction::Up && *redir == Direction::Right ||
                dir == Direction::Right && *redir == Direction::Down ||
                dir == Direction::Down && *redir == Direction::Left {
                    revisits += 1;
            }
        }
        visited.insert((x,y), dir);
        match dir {
            Direction::Up => {
                let Some(tile) = map.get(&(x,y-1)) else {break};
                if *tile == '#' {
                    dir = Direction::Right;
                    continue;
                }
                y -= 1;
            },
            Direction::Right => {
                let Some(tile) = map.get(&(x+1,y)) else {break};
                if *tile == '#' {
                    dir = Direction::Down;
                    continue;
                }
                x += 1;
            },
            Direction::Down => {
                let Some(tile) = map.get(&(x,y+1)) else {break};
                if *tile == '#' {
                    dir = Direction::Left;
                    continue;
                }
                y += 1;
            },
            Direction::Left => {
                let Some(tile) = map.get(&(x-1,y)) else {break};
                if *tile == '#' {
                    dir = Direction::Up;
                    continue;
                }
                x -= 1;
            },
        }
    }
    revisits

}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction { Up, Down, Left, Right }

#[cfg(test)]
const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_1() {
    assert_eq!(solve_1(EXAMPLE), 41);
}


#[test]
fn test_2() {
    assert_eq!(solve_2(EXAMPLE), 6);
}
