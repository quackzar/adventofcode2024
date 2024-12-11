use std::collections::{BTreeMap, BTreeSet};

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
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect()
}

fn solve_1(input: &str) -> u32 {
    let map = parse(input);
    visited(&map).len() as u32
}

fn visited(map: &BTreeMap<(isize, isize), char>) -> BTreeSet<(isize, isize)> {
    let (pos, _) = map.iter().find(|(pos, c)| **c == '^').unwrap();
    let mut pos = *pos;
    let mut dir = Direction::Up;
    let mut visited = BTreeSet::new();
    visited.insert(pos);
    while !walk(map, &mut pos, &mut dir) {
        visited.insert(pos);
    }
    visited
}

fn walk(
    map: &BTreeMap<(isize, isize), char>,
    (x, y): &mut (isize, isize),
    dir: &mut Direction,
) -> bool {
    match dir {
        Direction::Up => {
            let Some(tile) = map.get(&(*x, *y - 1)) else {
                return true;
            };
            if *tile == '#' {
                *dir = Direction::Right;
            } else {
                *y -= 1;
            }
        }
        Direction::Right => {
            let Some(tile) = map.get(&(*x + 1, *y)) else {
                return true;
            };
            if *tile == '#' {
                *dir = Direction::Down;
            } else {
                *x += 1;
            }
        }
        Direction::Down => {
            let Some(tile) = map.get(&(*x, *y + 1)) else {
                return true;
            };
            if *tile == '#' {
                *dir = Direction::Left;
            } else {
                *y += 1;
            }
        }
        Direction::Left => {
            let Some(tile) = map.get(&(*x - 1, *y)) else {
                return true;
            };
            if *tile == '#' {
                *dir = Direction::Up;
            } else {
                *x -= 1;
            }
        }
    }
    false
}

fn solve_2(input: &str) -> u32 {
    fn is_loop(map: &BTreeMap<(isize, isize), char>, initial: (isize, isize)) -> bool {
        let mut path = BTreeSet::new();
        let mut pos = initial;
        let mut dir = Direction::Up;
        path.insert((pos, dir));
        while !walk(map, &mut pos, &mut dir) {
            if path.contains(&(pos, dir)) {
                return true
            }
            path.insert((pos, dir));
        }
        false
    }

    let mut map = parse(input);
    let (initial, _) = map.iter().find(|(_pos, c)| **c == '^').unwrap();
    let initial = *initial;
    let mut visited = visited(&map);
    let mut sum = 0;
    visited.remove(&initial);
    for pos in visited {
        *map.get_mut(&pos).unwrap() = '#';
        if is_loop(&map, initial) {
            sum += 1;
        }
        *map.get_mut(&pos).unwrap() = '.';
    }
    sum
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
