use std::collections::BTreeMap;

use regex::Regex;


fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let regex = Regex::new(r"([0-9]+)[ ]+([0-9]+)").unwrap();
    input.lines()
        .map(|line| {
            let (_, [a,b]) = regex.captures(line).unwrap().extract();
            let a : u32 = a.parse().unwrap();
            let b : u32 = b.parse().unwrap();
            (a,b)
        }).unzip()
}

fn solve_1(input: &str) -> u32 {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    left.iter().zip(right.iter()).map(|(&x,&y)| x.abs_diff(y)).sum()
}

fn solve_2(input: &str) -> u32 {
    let (left, right) = parse(input);
    let mut cache = BTreeMap::new();
    let mut sim = 0;
    for id in left {
        if let Some(times) = cache.get(&id) {
            sim += times * id;
        } else {
            let times = right.iter().filter(|x| **x == id).count() as u32;
            cache.insert(id, times);
            sim += times * id;
        }
    }
    sim
}

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve_1(&input);
    println!("Solution 1: {res}");

    let res = solve_2(&input);
    println!("Solution 2: {res}");
}


#[test]
fn test_part1() {
    let test_input: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    let res = solve_1(test_input);
    assert_eq!(res, 11);
}


#[test]
fn test_part2() {
    let test_input: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    let res = solve_2(test_input);
    assert_eq!(res, 31);
}
