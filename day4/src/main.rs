#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

use std::{fmt::Display, ops::{Coroutine, CoroutineState, Index}, pin::Pin};

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = solve_1(&input);
    println!("Solution 1: {res}");

    // let res = solve_2(&input);
    // println!("Solution 2: {res}");
}

fn new_parser() -> impl Coroutine<char, Yield = (), Return = ()>{
    #[coroutine] |_: char| {
        'main: loop {
            for k in "XMAS".chars() {
                if (yield) != k {
                    continue 'main;
                };
            }
            return;
        }
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
        let rows = input.lines().count();
        let cols = (input.len()) / rows;

        // could just copy the string, might need to handle line endings.
        for line in input.lines() {
            for c in line.chars() {
                data.push(c)
            }
        }
        Self {
            data,
            rows,
            cols
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.cols {
            for j in 0..self.rows {
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
        let mut parser = new_parser();
        for j in 0..matrix.cols {
            let c = matrix[i][j];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }
    // backwards
    for i in 0..matrix.rows {
        let mut parser = new_parser();
        for j in (0..matrix.cols).rev() {
            let c = matrix[i][j];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }
    // down
    for j in 0..matrix.cols {
        let mut parser = new_parser();
        for i in 0..matrix.rows {
            let c = matrix[i][j];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }
    // up
    for j in 0..matrix.cols {
        let mut parser = new_parser();
        for i in (0..matrix.rows).rev() {
            let c = matrix[i][j];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }

    // \ forward?
    for j in 0..matrix.cols {
        let mut parser = new_parser();
        for i in 0..(matrix.cols - j) {
            let c = matrix[i][j+i];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }

    // \ backward?
    for j in 0..matrix.cols {
        let mut parser = new_parser();
        for i in (0..matrix.cols).rev() {
            let c = matrix[i][j+i];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }

    // / forward?
    for j in 0..matrix.cols {
        let mut parser = new_parser();
        for i in 0..matrix.cols {
            let c = matrix[j+i][i];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }
    // / backward?
    for j in 0..matrix.cols {
        let mut parser = new_parser();
        for i in (0..matrix.cols).rev() {
            let c = matrix[j+i][i];
            if let CoroutineState::Complete(_) = Pin::new(&mut parser).resume(c) {
                found += 1;
                parser = new_parser();
            }
        }
    }
    found
}



#[cfg(test)]
const INPUT : &str = "\
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
