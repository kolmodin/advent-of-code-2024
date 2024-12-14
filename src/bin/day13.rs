use std::{fs};

use itertools::Itertools;

fn solve(x1: i64, y1: i64, x2: i64, y2: i64, xt: i64, yt: i64) -> Option<i64> {
    let b = (yt * x1 - y1 * xt) / (x1 * y2 - y1 * x2);
    let a = (xt - x2 * b) / x1;
    let xt_calculated = a * x1 + b * x2;
    let yt_calculated = a * y1 + b * y2;
    if xt_calculated == xt && yt_calculated == yt && a >= 0 && b >= 0 {
        Some(a * 3 + b)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day13.txt")?;

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for x in contents.split("\n\n") {
        let s = x
            .chars()
            .map(|c| if c.is_numeric() { c } else { ' ' })
            .collect::<String>();
        let v = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        if let [x1, y1, x2, y2, xt, yt] = v[..] {
            if let Some(cost) = solve(x1, y1, x2, y2, xt, yt) {
                part1_sum += cost;
            }
            if let Some(cost) = solve(x1, y1, x2, y2, xt + 10000000000000, yt + 10000000000000) {
                part2_sum += cost;
            }
        }
    }

    println!("Part 1: {}", part1_sum);
    assert_eq!(part1_sum, 28887);
    println!("Part 2: {}", part2_sum);
    assert_eq!(part2_sum, 96979582619758);

    Ok(())
}
