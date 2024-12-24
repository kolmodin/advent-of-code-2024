use std::{fs, usize};

use aoc2024::pos::Pos;
use itertools::Itertools;

fn solve(block_map: &[usize], corrupt_count: usize, bounds: Pos) -> Option<usize> {
    let width = bounds.x as usize;
    let height = bounds.y as usize;
    let mut cost = vec![usize::MAX; width * height];
    let mut seen = vec![false; width * height];

    cost[0] = 0;

    while let Some(i) = (0..width * height)
        .filter(|&i| !seen[i])
        .min_by_key(|&i| cost[i])
    {
        let c = cost[i];
        if c == usize::MAX {
            // Next position is unreachable, we've finished.
            break;
        }
        let p = Pos::from_linear(i as i64, width as i64);
        for neighbour in p.cross() {
            if neighbour.within_bounds(&bounds) {
                let n = neighbour.to_linear(width as i64);
                if block_map[n] > corrupt_count {
                    cost[n] = std::cmp::min(cost[n], c + 1);
                }
            }
        }
        seen[i] = true;
    }

    let c = cost[width * height - 1];
    if c == usize::MAX { None } else { Some(c) }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day18.txt")?;

    let blocks = contents
        .lines()
        .map(|ln| {
            let (x, y) = ln.split_once(',').unwrap();
            Pos::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec();

    let width = 71;
    let height = 71;
    let bounds = Pos::new(width as i64, height as i64);
    let mut block_map = vec![usize::MAX; width * height];

    for (i, p) in blocks.iter().enumerate() {
        block_map[p.to_linear(width as i64)] = i + 1;
    }

    let part1_cost = solve(&block_map, 1024, bounds).unwrap();
    println!("Part 1: {}", part1_cost);
    assert_eq!(part1_cost, 354);

    let corrupt_idx = (0..=blocks.len()).collect_vec();
    let first_fail =
        corrupt_idx.partition_point(|idx| solve(&block_map, *idx + 1, bounds).is_some());
    let part2_coord = blocks[first_fail];
    println!("Part 2: {},{}", part2_coord.x, part2_coord.y);
    assert_eq!(part2_coord, Pos::new(36, 17));

    Ok(())
}
