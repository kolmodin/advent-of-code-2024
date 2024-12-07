use std::fs;

use itertools::Itertools;

fn solve(lines: &[(i64, Vec<i64>)], concat: bool) -> i64 {
    let mut sum = 0;

    for (target, nums) in lines {
        let mut prev = vec![nums[0]];

        for &y in &nums[1..] {
            let mut next = Vec::with_capacity(prev.len() * 3);
            for &x in &prev {
                if x + y <= *target {
                    next.push(x + y);
                }
                if x * y <= *target {
                    next.push(x * y);
                }
                if concat {
                    let mut x = x;
                    let mut my = y;
                    loop {
                        x *= 10;
                        my /= 10;
                        if my == 0 {
                            break;
                        }
                    }
                    let z = x + y;
                    if z <= *target {
                        next.push(z);
                    }
                }
            }
            prev = next;
        }
        if prev.contains(target) {
            sum += target;
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day07.txt")?;

    let lines = contents
        .lines()
        .map(|ln| {
            let mut nums = ln
                .split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap());
            let sum = nums.next().unwrap();
            (sum, nums.collect_vec())
        })
        .collect_vec();

    let part1_sum = solve(&lines, false);
    println!("Part 1: {}", part1_sum);
    assert_eq!(part1_sum, 7885693428401);

    let part2_sum = solve(&lines, true);
    println!("Part 2: {}", part2_sum);
    assert_eq!(part2_sum, 348360680516005);

    Ok(())
}
