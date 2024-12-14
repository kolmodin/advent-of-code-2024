use std::fs;

use aoc2024::pos::Pos;
use itertools::Itertools;

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vec: Pos,
}

impl Robot {
    fn step(&self, t: i64, bounds: Pos) -> Pos {
        Pos {
            x: (self.pos.x + t * self.vec.x).rem_euclid(bounds.x),
            y: (self.pos.y + t * self.vec.y).rem_euclid(bounds.y),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day14.txt")?;

    let mut robots = vec![];
    for ln in contents.lines() {
        let s = ln
            .chars()
            .map(|c| if c.is_numeric() || c == '-' { c } else { ' ' })
            .collect::<String>();
        let nums = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        if let &[x, y, vx, vy] = nums.as_slice() {
            robots.push(Robot {
                pos: Pos::new(y, x),
                vec: Pos::new(vy, vx),
            });
        }
    }

    let bounds = Pos::new(103, 101);

    let part1_quadrants: usize = robots
        .iter()
        .map(|r| r.step(100, bounds))
        .filter(|p| p.x != bounds.x / 2 && p.y != bounds.y / 2)
        .counts_by(|p| (p.x / (bounds.right().x / 2), p.y / (bounds.down().y / 2)))
        .values()
        .product();

    println!("Part 1: {}", part1_quadrants);
    assert_eq!(part1_quadrants, 218965032);

    for t in 1.. {
        let pos = robots.iter().map(|r| r.step(t, bounds));
        let mut s = vec![' '; 103 * 101];
        for p in pos {
            s[p.to_linear(bounds.x)] = 'x';
        }

        let str = s
            .chunks(bounds.x as usize)
            .map(|ln| ln.iter().collect::<String>())
            .join("\n");

        if str.contains("xxxxxxxxxx") {
            println!("Part 2: {}", t);
            // println!("{}", str);
            break;
        }
    }

    Ok(())
}
