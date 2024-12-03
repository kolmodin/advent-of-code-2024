use std::fs;

use itertools::Itertools;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("inputs/day03.txt")?;
    let content = content.lines().join("");

    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut part1_sum = 0;
    for c in mul_re.captures_iter(&content) {
        let (_, [x, y]) = c.extract();

        let x: i64 = x.parse()?;
        let y: i64 = y.parse()?;

        part1_sum += x * y;
    }
    println!("Part 1: {}", part1_sum);
    assert_eq!(part1_sum, 182619815);

    let mut part2_sum = 0;
    let dos = content.split("do()").skip(1);
    for d in dos {
        let doit = d.split("don't()").next().unwrap();
        for c in mul_re.captures_iter(&doit) {
            let (_, [x, y]) = c.extract();

            let x: i64 = x.parse()?;
            let y: i64 = y.parse()?;
            part2_sum += x * y;
        }
    }
    println!("Part 2: {}", part2_sum);
    assert_eq!(part2_sum, 80747545);

    Ok(())
}
