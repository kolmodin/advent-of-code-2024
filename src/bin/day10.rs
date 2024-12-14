use std::{collections::HashSet, fs};

use itertools::Itertools;

use aoc2024::pos::Pos;

#[derive(Debug)]
struct Trail {
    head: Pos,
    pos: Pos,
    height: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day10.txt")?;

    println!("{}", contents);

    let width = contents.find('\n').unwrap() as i64;
    let height = contents.lines().count() as i64;
    let bounds = Pos {
        y: height,
        x: width,
    };

    let original_map = contents
        .lines()
        .join("")
        .into_bytes()
        .into_iter()
        .map(|c| c - b'0')
        .collect_vec();

    let mut trails = vec![];

    for y in 0..height {
        for x in 0..width {
            let pos = Pos::new(y, x);
            if original_map[pos.to_linear(width)] != 0 {
                continue;
            }

            trails.push(Trail {
                head: pos,
                pos,
                height: 0,
            });
        }
    }

    println!("Up to {} trail starts", trails.len());

    let mut part1_trails = HashSet::new();
    let mut part2_distinct_tail_sum = 0;

    let offsets = vec![
        Pos { y: -1, x: 0 }, // up
        Pos { y: 0, x: 1 },
        Pos { y: 1, x: 0 },
        Pos { y: 0, x: -1 },
    ];

    while let Some(trail) = trails.pop() {
        if trail.height == 9 {
            part1_trails.insert((trail.head, trail.pos));
            part2_distinct_tail_sum += 1;
            continue;
        }

        for offset in &offsets {
            let next = trail.pos + *offset;
            if !next.within_bounds(&bounds) {
                continue;
            }
            let c = original_map[next.to_linear(width)];
            if c == trail.height + 1 {
                trails.push(Trail {
                    head: trail.head,
                    pos: next,
                    height: c,
                })
            }
        }
    }

    println!("Part 1: {} scores", part1_trails.len());
    assert_eq!(part1_trails.len(), 552);
    println!("Part 2: {} unique trails", part2_distinct_tail_sum);
    assert_eq!(part2_distinct_tail_sum, 1225);

    Ok(())
}
