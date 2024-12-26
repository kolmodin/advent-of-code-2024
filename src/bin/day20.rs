use std::{fs};

use aoc2024::{map::Map, pos::Pos};
use itertools::Itertools;

fn manhattan(p: &Pos, dist: usize) -> impl Iterator<Item = Pos> {
    let dirs = [
        (*p + Pos::north() * dist, Pos::south() + Pos::east()),
        (*p + Pos::east() * dist, Pos::south() + Pos::west()),
        (*p + Pos::south() * dist, Pos::north() + Pos::west()),
        (*p + Pos::west() * dist, Pos::north() + Pos::east()),
    ];
    let mut manhattan = Vec::with_capacity(dist * 4);
    for d in 0..dist {
        for (start, dir) in dirs {
            manhattan.push(start + dir * d);
        }
    }

    manhattan.into_iter()
}

/** Manhattan distance up to and including the given distance around a Pos. */
fn manhattan_up_to(p: &Pos, dist: usize) -> impl Iterator<Item = Pos> {
    (1..=dist).flat_map(|d| manhattan(p, d))
}

fn solve(map: &Map<u8>, dist: &Map<usize>, manhattan: usize) -> Vec<usize> {
    let bounds = map.bounds();
    let mut cheats = vec![];
    for p in map.iter_pos() {
        if dist[&p] == usize::MAX {
            continue;
        }
        for c in manhattan_up_to(&p, manhattan) {
            if !c.within_bounds(&bounds) || dist[&c] == usize::MAX {
                continue;
            }
            let man = p.manhattan(c);
            if dist[&c] > dist[&p] + man {
                let diff = dist[&c] - dist[&p] - man;
                cheats.push(diff);
            }
        }
    }
    cheats
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day20.txt")?;

    let map = Map::from(contents.as_str());
    let bounds = map.bounds();

    let start = map.position(|c| *c == b'S').unwrap();
    let _end = map.position(|c| *c == b'E').unwrap();

    let mut dist = Map::new_with_size_as(&map, usize::MAX);
    dist[&start] = 0;

    let mut queue = vec![start];

    while let Some(p) = queue.pop() {
        let mut next = p
            .cross()
            .filter(|p| p.within_bounds(&bounds) && map[p] != b'#' && dist[p] == usize::MAX)
            .collect_vec();
        assert!(next.len() == 1 || map[&p] == b'E');

        if let Some(next) = next.pop() {
            dist[&next] = dist[&p] + 1;
            queue.push(next);
        }
    }

    let part1 = solve(&map, &dist, 2)
        .into_iter()
        .filter(|&c| c >= 100)
        .count();
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1286);

    let part2 = solve(&map, &dist, 20)
        .into_iter()
        .filter(|&c| c >= 100)
        .count();
    println!("Part 2: {}", part2);
    assert_eq!(part2, 989316);

    Ok(())
}
