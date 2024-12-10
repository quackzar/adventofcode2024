use itertools::{Itertools, repeat_n};

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
    solve(input, &[Operation::Add, Operation::Mul])
}

fn solve_2(input: &str) -> u64 {
    solve(input, &[Operation::Add, Operation::Mul, Operation::Cat])
}

fn solve(input: &str, ops: &[Operation]) -> u64 {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let (res, args) = line.split_once(":").unwrap();
            let result = res.parse().unwrap();
            let arguments = args
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            Equation { result, arguments }
        })
        .collect();

    let mut cali_results = 0;
    for eq in equations {
        let n = eq.arguments.len();
        for mut stack in repeat_n(ops, n - 1).multi_cartesian_product() {
            let mut check = eq.arguments[0];
            let mut i = 1;
            while let Some(op) = stack.pop() {
                let arg = eq.arguments[i];
                match op {
                    Operation::Mul => check *= arg,
                    Operation::Add => check += arg,
                    Operation::Cat => {
                        let mut hold = check.to_string();
                        hold.push_str(&arg.to_string());
                        check = hold.parse().unwrap()
                    }
                }
                //short-circuit
                if check > eq.result {
                    break;
                }
                i += 1;
            }
            if check == eq.result {
                cali_results += eq.result;
                break;
            }
        }
    }
    cali_results
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Mul,
    Add,
    Cat,
}

#[derive(Debug)]
struct Equation {
    result: u64,
    arguments: Vec<u64>,
}

#[cfg(test)]
const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn test_1() {
    assert_eq!(solve_1(EXAMPLE), 3749);
}

#[test]
fn test_2() {
    assert_eq!(solve_2(EXAMPLE), 11387);
}
