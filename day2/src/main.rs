#[derive(Debug)]
struct Report {
    levels: Vec<u8>,
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            let levels = line
                .split_terminator(" ")
                .map(|num| num.parse().unwrap())
                .collect();
            Report { levels }
        })
        .collect()
}

fn is_safe(report: &Report) -> bool {
    let diffs = report.levels.windows(2).map(|win| {
        let &[x, y] = win else { panic!() };
        x as i8 - y as i8
    });

    let ascend = diffs.clone().filter(|diff| (1..=3).contains(diff)).count();
    let descend = diffs.filter(|diff| (-3..=-1).contains(diff)).count();

    // total diffs - good levles
    report.levels.len() - 1 == ascend.max(descend)
}


fn problem_dampened(mut report: Report) -> bool {
    let mut removed = None;
    let mut i = 0;
    loop {
        if is_safe(&report) {
            return true;
        }
        if let Some(r) = removed {
            report.levels.insert(i, r);
            i += 1;
        }

        if i >= report.levels.len() {
            return false;
        }
        removed = Some(report.levels.remove(i));
    }
}

fn solve_1(input: &str) -> u32 {
    let reports = parse(input);
    reports.iter().filter(|r| is_safe(r)).count() as u32
}

fn solve_2(input: &str) -> u32 {
    let reports = parse(input);
    reports
        .into_iter()
        .map(|r| problem_dampened(r) as u32)
        .sum()
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

#[cfg(test)]
const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[test]
fn test_part1() {
    let res = solve_1(TEST_INPUT);
    assert_eq!(res, 2);
}

#[test]
fn test_part2() {
    let res = solve_2(TEST_INPUT);
    assert_eq!(res, 4);
}
