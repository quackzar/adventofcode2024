#![feature(iterator_try_collect)]

use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet}};
use anyhow::Result;


fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (fst,snd) = solve(&input);
    println!("Solution 1: {fst}");
    println!("Solution 2: {snd}");
}

struct Update {
    pages: Vec<u8>,
}

fn parse(input: &str) -> Result<(BTreeMap<u8, BTreeSet<u8>>, Vec<Update>)> {
    let mut rules: BTreeMap<u8, BTreeSet<u8>> = BTreeMap::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        let Some((left, right)) = line.split_once('|') else {
            break;
        };
        let left : u8 = left.parse()?;
        let right : u8 = right.parse()?;
        if let Some(list) = rules.get_mut(&left) {
            list.insert(right);
        } else {
            rules.insert(left, BTreeSet::from([right]));
        }
    }
    
    let mut updates = Vec::new();
    for line in lines {
        let pages = line.split_terminator(',').map(|n| n.parse::<u8>()).try_collect()?;
        let update  = Update { pages };
        updates.push(update);
    }

    Ok((rules, updates))
}


fn solve(input: &str) -> (u32, u32) {
    let mut correct: u32 = 0;
    let mut incorrect: u32 = 0;
    let (rules, updates) = parse(input).unwrap();

    for mut update in updates {
        let is_conforming = update.pages.is_sorted_by(|a,b| {
            if let Some(set) = rules.get(b) {
                !set.contains(a)
            } else {
                true
            }
        });
        if is_conforming {
            let n = update.pages.len();
            correct += update.pages[n/2] as u32;
        } else {

            update.pages.sort_by(|a,b|{
                if let Some(set) = rules.get(a) {
                    if set.contains(b) {
                        return Ordering::Less
                    }

                };
                if let Some(set) = rules.get(b) {
                    if set.contains(b) {
                        return Ordering::Greater
                    }
                };
                Ordering::Equal
            });

            let n = update.pages.len();
            incorrect += update.pages[n/2] as u32;
        }
    }

    (correct, incorrect)
}





#[cfg(test)]
const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn test_1() {
    assert_eq!(solve(INPUT).0, 143);
}

#[test]
fn test_2() {
    assert_eq!(solve(INPUT).1, 123);
}
