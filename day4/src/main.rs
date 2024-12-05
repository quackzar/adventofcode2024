#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

use std::{
    fmt::Display,
    isize,
    ops::{Coroutine, CoroutineState, Index},
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

fn new_parser() -> impl Coroutine<char, Yield = (), Return = ()> {
    #[coroutine]
    |_: char| 'main: loop {
        for k in "XMAS".chars() {
            if (yield) != k {
                continue 'main;
            };
        }
        return;
    }
}

struct Matrix {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn from_input(input: &str) -> Self {
        let mut data = Vec::new();
        let mut rows = 0;
        let mut cols= 0;

        // could just copy the string, might need to handle line endings.
        for line in input.lines() {
            rows += 1;
            for c in line.chars() {
                data.push(c);
                cols += 1;
            }
        }

        let cols = cols / rows;
        Self { data, rows, cols }
    }

    fn get(&self, i: isize, j: isize) -> Option<char> {
        if i < 0 {
            return None;
        };
        if j < 0 {
            return None;
        };

        let n = i as usize * self.rows;
        let m = n + self.cols;
        self.data.get(n..m)?.get(j as usize).cloned()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = self[i][j];
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<usize> for Matrix {
    type Output = [char];

    fn index(&self, index: usize) -> &Self::Output {
        let n = index * self.rows;
        let m = n + self.cols;
        &self.data[n..m]
    }
}

fn solve_1(input: &str) -> u32 {
    let matrix = Matrix::from_input(input);
    let mut found = 0;
    // forwards
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            let mut forward = new_parser();
            Pin::new(&mut forward).resume('_');
            let mut backward = new_parser();
            Pin::new(&mut backward).resume('_');
            let mut up = new_parser();
            Pin::new(&mut up).resume('_');
            let mut down = new_parser();
            Pin::new(&mut down).resume('_');
            let mut diag0 = new_parser();
            Pin::new(&mut diag0).resume('_');
            let mut diag1 = new_parser();
            Pin::new(&mut diag1).resume('_');
            let mut diag2 = new_parser();
            Pin::new(&mut diag2).resume('_');
            let mut diag3 = new_parser();
            Pin::new(&mut diag3).resume('_');
            let i = i as isize;
            let j = j as isize;
            for k in 0..4 {
                if let Some(c) = matrix.get(i, j + k) {
                    match Pin::new(&mut forward).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i, j - k) {
                    match Pin::new(&mut backward).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i + k, j) {
                    match Pin::new(&mut down).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i - k, j) {
                    match Pin::new(&mut up).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i + k, j + k) {
                    match Pin::new(&mut diag0).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i + k, j - k) {
                    match Pin::new(&mut diag1).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i - k, j + k) {
                    match Pin::new(&mut diag2).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
                if let Some(c) = matrix.get(i - k, j - k) {
                    match Pin::new(&mut diag3).resume(c) {
                        CoroutineState::Yielded(_) => (),
                        CoroutineState::Complete(_) => found += 1,
                    }
                };
            }
        }
    }
    found
}



fn solve_2(input: &str) -> u32 {
    let matrix = Matrix::from_input(input);
    let mut found = 0;

    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            let mut parser = new_parser();
            let i = i as isize;
            let j = j as isize;


            let a = matrix.get(i,j) == Some('A')
                && matrix.get(i+1,j+1) == Some('M')
                && matrix.get(i-1,j-1) == Some('S')
                && matrix.get(i+1,j-1) == Some('M')
                && matrix.get(i-1,j+1) == Some('S');

            let b = matrix.get(i,j) == Some('A')
                && matrix.get(i+1,j+1) == Some('S')
                && matrix.get(i-1,j-1) == Some('M')
                && matrix.get(i+1,j-1) == Some('M')
                && matrix.get(i-1,j+1) == Some('S');

            let c = matrix.get(i,j) == Some('A')
                && matrix.get(i+1,j+1) == Some('S')
                && matrix.get(i-1,j-1) == Some('M')
                && matrix.get(i+1,j-1) == Some('S')
                && matrix.get(i-1,j+1) == Some('M');

            let d = matrix.get(i,j) == Some('A')
                && matrix.get(i+1,j+1) == Some('M')
                && matrix.get(i-1,j-1) == Some('S')
                && matrix.get(i+1,j-1) == Some('S')
                && matrix.get(i-1,j+1) == Some('M');




            if a || b || c || d {
                found += 1;
            }
        }
    }

    found
}

#[cfg(test)]
const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn test_1() {
    assert_eq!(solve_1(INPUT), 18);
}

#[test]
fn test_2() {
    assert_eq!(solve_2(INPUT), 9);
}
