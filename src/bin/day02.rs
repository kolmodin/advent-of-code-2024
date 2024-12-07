use std::fs;

use itertools::Itertools;

fn increasing(x: i64) -> bool {
    (1..=3).contains(&x)
}

fn decreasing(x: i64) -> bool {
    (-3..=-1).contains(&x)
}

fn check(arr: &[i64], can_skip: bool, pred: &impl Fn(i64) -> bool) -> bool {
    if !can_skip {
        for i in 0..arr.len() - 1 {
            if !pred(arr[i + 1] - arr[i]) {
                return false;
            }
        }
        return true;
    }

    let mut sub = vec![];
    for skip in 0..arr.len() {
        sub.clear();
        for (i, x) in arr.iter().enumerate() {
            if i != skip {
                sub.push(*x);
            }
        }

        if check(&sub, false, pred) {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day02.txt")?;

    let reports: Vec<Vec<i64>> = contents
        .lines()
        .map(|ln| {
            ln.split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let part1_safe = reports
        .iter()
        .filter(|levels| check(levels, false, &increasing) || check(levels, false, &decreasing))
        .count();

    println!("Part 1: {}", part1_safe);
    assert_eq!(part1_safe, 432);

    let part2_safe = reports
        .iter()
        .filter(|levels| check(levels, true, &increasing) || check(levels, true, &decreasing))
        .count();

    println!("Part 2: {}", part2_safe);
    assert_eq!(part2_safe, 488);

    Ok(())
}
