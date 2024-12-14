use std::{collections::HashSet, fs};

use itertools::{Itertools, iterate};

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day08.txt")?;

    let width = contents.find('\n').unwrap() as i32;
    let height = contents.lines().count() as i32;

    let original_map = contents.lines().join("").into_bytes();

    let nodes = original_map
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != b'.')
        .collect_vec();

    let in_bounds = |p: &(i32, i32)| p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height;
    let from_linear = |i| (i as i32 % width, i as i32 / width);

    let mut part1_antinodes = HashSet::new();
    let mut part2_antinodes = HashSet::new();

    for &(i, n1) in nodes.iter() {
        for &(j, n2) in nodes.iter() {
            if i >= j || n1 != n2 {
                continue;
            }
            let a = from_linear(i);
            let b = from_linear(j);
            let add = |a: &(i32, i32), b: &(i32, i32)| (a.0 + b.0, a.1 + b.1);
            let sub = |a: &(i32, i32), b: &(i32, i32)| (a.0 - b.0, a.1 - b.1);
            let diff = sub(&b, &a);

            part1_antinodes.extend(Some(sub(&a, &diff)).filter(in_bounds));
            part1_antinodes.extend(Some(add(&b, &diff)).filter(in_bounds));

            let d = gcd(diff.0, diff.1);
            let diff = (diff.0 / d, diff.1 / d);
            part2_antinodes.extend(iterate(a, |x| sub(x, &diff)).take_while(in_bounds));
            part2_antinodes.extend(iterate(b, |x| add(x, &diff)).take_while(in_bounds));
        }
    }

    println!("Part 1: {}", part1_antinodes.len());
    assert_eq!(part1_antinodes.len(), 220);
    println!("Part 2: {}", part2_antinodes.len());
    assert_eq!(part2_antinodes.len(), 813);

    Ok(())
}
