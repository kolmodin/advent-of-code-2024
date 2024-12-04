use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day04.txt")?;

    let width = contents.find('\n').unwrap() as i32;
    let height = contents.lines().count() as i32;
    let m = contents.lines().join("");

    let offsets: Vec<(i32, i32)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let search = "XMAS";

    let mut part1_matches = 0;

    for offset in offsets {
        for start_y in 0..height {
            'x: for start_x in 0..width {
                for i in 0..search.len() {
                    let x = start_x + (i as i32) * offset.0;
                    let y = start_y + (i as i32) * offset.1;
                    if x < 0
                        || x >= width
                        || y < 0
                        || y >= height
                        || search.as_bytes()[i] != m.as_bytes()[(y * width + x) as usize]
                    {
                        continue 'x;
                    }
                }
                part1_matches += 1;
            }
        }
    }

    println!("Part 1: {}", part1_matches);
    assert_eq!(part1_matches, 2358);

    // Only diagonals.
    let offsets: Vec<(i32, i32)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let search = "MAS";
    let mut part2_matches = 0;

    let mut midpoints = HashSet::new();

    for offset in offsets {
        for start_y in 0..height {
            'x: for start_x in 0..width {
                for i in 0..search.len() {
                    let x = start_x + (i as i32) * offset.0;
                    let y = start_y + (i as i32) * offset.1;
                    if x < 0
                        || x >= width
                        || y < 0
                        || y >= height
                        || search.as_bytes()[i] != m.as_bytes()[(y * width + x) as usize]
                    {
                        continue 'x;
                    }
                }
                let mid_x = start_x + offset.0;
                let mid_y = start_y + offset.1;
                let mid = (mid_x, mid_y);
                if midpoints.contains(&mid) {
                    part2_matches += 1;
                } else {
                    midpoints.insert(mid);
                }
            }
        }
    }

    println!("Part 2: {}", part2_matches);
    assert_eq!(part2_matches, 1737);

    Ok(())
}
