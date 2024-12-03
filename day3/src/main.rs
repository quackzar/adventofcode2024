#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

use std::{
    ops::{Coroutine, CoroutineState},
    pin::Pin,
};

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve_1(&input);
    println!("Solution 1: {res}");

    let res = solve_2(&input);
    println!("Solution 2: {res}");
}

fn new_parser() -> impl Coroutine<char, Yield = (), Return = Option<u32>> {
    #[coroutine]
    |c: char| {
        if c != 'm' {
            return None;
        };
        if (yield) != 'u' {
            return None;
        };
        if (yield) != 'l' {
            return None;
        };
        if (yield) != '(' {
            return None;
        };

        // parse left number
        let c = yield;
        if !c.is_ascii_digit() {
            return None;
        };
        let mut buf = String::new();
        buf.push(c);

        let c = yield;
        let c = if c != ',' {
            if !c.is_ascii_digit() {
                return None;
            };
            buf.push(c);
            yield
        } else {
            c
        };
        let c = if c != ',' {
            if !c.is_ascii_digit() {
                return None;
            };
            buf.push(c);
            yield
        } else {
            c
        };
        let left: u32 = buf.parse().unwrap();

        if c != ',' {
            return None;
        };

        // parse right number
        let c = yield;
        if !c.is_ascii_digit() {
            return None;
        };
        buf.clear();
        buf.push(c);

        let c = yield;
        let c = if c != ')' {
            if !c.is_ascii_digit() {
                return None;
            };
            buf.push(c);
            yield
        } else {
            c
        };

        let c = if c != ')' {
            if !c.is_ascii_digit() {
                return None;
            };
            buf.push(c);
            yield
        } else {
            c
        };

        if (c) != ')' {
            return None;
        };
        let right: u32 = buf.parse().unwrap();

        Some(left * right)
    }
}

fn solve_1(input: &str) -> u32 {
    let mut parser = new_parser();

    let mut sum = 0;
    for c in input.chars() {
        match Pin::new(&mut parser).resume(c) {
            CoroutineState::Yielded(_) => continue,
            CoroutineState::Complete(res) => {
                if let Some(num) = res {
                    sum += num;
                }
                parser = new_parser();
            }
        }
    }
    sum
}

fn new_do_checker() -> impl Coroutine<char, Yield = bool, Return = ()> {
    #[coroutine]
    |c: char| {
        let mut enabled = true;
        loop {
            'main: {
                if enabled {
                    for k in "don't()".chars() {
                        let c = yield enabled;
                        if c != k {
                            break 'main;
                        }
                    }
                    enabled = false;
                } else {
                    for k in "do()".chars() {
                        let c = yield enabled;
                        if c != k {
                            break 'main;
                        }
                    }
                    enabled = true;
                }
            }
        }
    }
}

fn solve_2(input: &str) -> u32 {
    let mut parser = new_parser();
    let mut doer = new_do_checker();

    let mut sum = 0;
    for c in input.chars() {
        let is_enabled = match Pin::new(&mut doer).resume(c) {
            CoroutineState::Yielded(is_enabled) => is_enabled,
            _ => unreachable!(),
        };

        if is_enabled {
            match Pin::new(&mut parser).resume(c) {
                CoroutineState::Yielded(_) => continue,
                CoroutineState::Complete(res) => {
                    if let Some(num) = res {
                        sum += num;
                    }
                    parser = new_parser();
                }
            }
        }
    }
    sum
}

#[cfg(test)]

#[test]
fn test_part1() {
    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let res = solve_1(TEST_INPUT);
    assert_eq!(res, 161);
}

#[test]
fn test_part2() {
    const TEST_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let res = solve_2(TEST_INPUT);
    assert_eq!(res, 48);
}
