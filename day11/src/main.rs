#![feature(linked_list_cursors)]
#![feature(allocator_api)]
#![feature(iter_collect_into)]
#![feature(let_chains)]

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use std::{
    alloc::{Allocator, Global},
    collections::{BTreeMap, LinkedList, VecDeque},
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

fn solve_1(input: &str) -> u64 {
    let mut stones = parse(input, std::alloc::Global);

    for _ in 0..25 {
        blink(&mut stones);
    }

    stones.len() as u64
}

fn num_digits(num: u64) -> u32 {
    (num as f64).log10() as u32 + 1
}

fn blink<A: Allocator>(stones: &mut LinkedList<u64, A>) {
    let mut cursor = stones.cursor_front_mut();

    while let Some(stone) = cursor.current() {
        match *stone {
            0 => *stone = 1,
            n if num_digits(n) & 1 == 0 => {
                let half_digits = num_digits(n) / 2;
                let multiplier = 10_u64.pow(half_digits);
                let left = n / multiplier;
                let right = n - left * multiplier;

                *stone = left;
                cursor.insert_after(right);
                cursor.move_next();
            }
            n => *stone = n * 2024,
        }
        cursor.move_next();
    }
}

fn solve_2(input: &str) -> u64 {
    let stones = parse(input, Global);
    let mut total = 0;


    let mut cache = BTreeMap::new();
    for stone in stones {
        let total_steps = 75;
        let mut queue = Vec::new();
        queue.push((stone, total_steps));

        while let Some((stone, step)) = queue.pop() {
            if step == 0 {
                cache.insert((stone, step), 1);
                continue;
            }

            match stone {
                0 => {
                    if let Some(&val) = cache.get(&(1, step - 1)) {
                        cache.insert((stone, step), val);
                    } else {
                        queue.push((stone, step));
                        queue.push((1, step - 1));
                    }
                }
                n if num_digits(n) & 1 == 0 => {
                    let half_digits = num_digits(n) / 2;
                    let multiplier = 10_u64.pow(half_digits);
                    let left = n / multiplier;
                    let right = n - left * multiplier;

                    if let Some(&left_val) = cache.get(&(left, step - 1))
                    && let Some(&right_val) = cache.get(&(right, step - 1)) {
                        cache.insert((stone, step), left_val + right_val);
                    } else {
                        queue.push((stone, step));
                        if !cache.contains_key(&(left, step-1)) {
                            queue.push((left, step-1));
                        }
                        if !cache.contains_key(&(right, step-1)) {
                            queue.push((right, step-1));
                        }

                    }


                }
                n => {
                    if let Some(&val) = cache.get(&(n * 2024, step - 1)) {
                        cache.insert((stone, step), val);
                    } else {
                        queue.push((stone, step));
                        queue.push((n * 2024, step - 1));
                    }
                }
            }
        }

        total += cache.get(&(stone, total_steps)).unwrap();

    }
    total
}

fn parse<A: Allocator>(input: &str, alloc: A) -> LinkedList<u64, A> {
    let mut list: LinkedList<_, A> = LinkedList::new_in(alloc);
    input
        .split_terminator(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect_into(&mut list);

    list
}

#[cfg(test)]
const EXAMPLE: &str = "125 17";

#[test]
fn part1() {
    let res = solve_1(EXAMPLE);
    assert_eq!(res, 55312);
}

#[test]
fn part2() {
    let res = solve_2(EXAMPLE);
    assert_eq!(res, 55312);
}
