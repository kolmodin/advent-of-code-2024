use core::panic;
use std::{collections::HashSet, fs};

use aoc2024::pos::Pos;
use itertools::{Itertools, iterate};

fn part1(map: &str, path: &str) -> i64 {
    let width = map.find('\n').unwrap() as i64;
    let height = map.lines().count() as i64;
    let path = path.lines().join("").chars().collect_vec();

    let mut map = map.split('\n').join("").into_bytes();
    let mut robot = Pos::from_linear(map.iter().position(|c| *c == b'@').unwrap() as i64, width);

    for c in path {
        let dir = match c {
            '^' => Pos::north(),
            'v' => Pos::south(),
            '<' => Pos::west(),
            '>' => Pos::east(),
            _ => panic!("unknown direction in path"),
        };

        let next_robot = robot + dir;
        if map[next_robot.to_linear(width)] == b'O' {
            let free = iterate(next_robot, |p| *p + dir)
                .take_while(|p| map[p.to_linear(width)] != b'#')
                .find(|p| map[p.to_linear(width)] == b'.');
            if let Some(free) = free {
                map.swap(next_robot.to_linear(width), free.to_linear(width));
            }
        }
        if map[next_robot.to_linear(width)] == b'.' {
            map.swap(robot.to_linear(width), next_robot.to_linear(width));
            robot = next_robot;
        }
    }

    let mut part1_sum = 0;
    for y in 0..height {
        for x in 0..width {
            if map[(y * width + x) as usize] == b'O' {
                part1_sum += y * 100 + x;
            }
        }
    }

    part1_sum
}

fn is_movable(map: &[u8], p: Pos, group: &mut HashSet<Pos>, width: i64, dir: Pos) -> bool {
    if group.contains(&p) {
        return true;
    }
    let c = map[p.to_linear(width)];
    match c {
        b'.' => true,
        b'#' => false,
        b'[' | b']' => {
            group.insert(p);
            if !is_movable(map, p + dir, group, width, dir) {
                return false;
            }

            let sibling = if c == b'[' { p.right() } else { p.left() };
            group.insert(sibling);
            is_movable(map, sibling + dir, group, width, dir)
        }
        _ => panic!("unknown char in map"),
    }
}

fn part2(map: &str, path: &str) -> i64 {
    let width = 2 * map.find('\n').unwrap() as i64;
    let height = map.lines().count() as i64;
    let path = path.lines().join("").chars().collect_vec();

    let mut map = map
        .split('\n')
        .join("")
        .into_bytes()
        .into_iter()
        .flat_map(|c| match c {
            b'#' => "##".bytes(),
            b'O' => "[]".bytes(),
            b'.' => "..".bytes(),
            b'@' => "@.".bytes(),
            _ => panic!("unknown char in map"),
        })
        .collect_vec();

    let mut robot = Pos::from_linear(map.iter().position(|c| *c == b'@').unwrap() as i64, width);

    for c in path {
        let dir = match c {
            '^' => Pos::north(),
            'v' => Pos::south(),
            '<' => Pos::west(),
            '>' => Pos::east(),
            _ => panic!("unknown direction in path"),
        };

        let next_robot = robot + dir;
        if [b'[', b']'].contains(&map[next_robot.to_linear(width)]) {
            let mut group = HashSet::<Pos>::new();
            if is_movable(&map, next_robot, &mut group, width, dir) {
                let recorded = group
                    .iter()
                    .map(|p| (*p, map[p.to_linear(width)]))
                    .collect_vec();
                for p in &group {
                    map[p.to_linear(width)] = b'.';
                }
                for (p, c) in recorded {
                    map[(p + dir).to_linear(width)] = c;
                }
            }
        }
        if map[next_robot.to_linear(width)] == b'.' {
            map.swap(robot.to_linear(width), next_robot.to_linear(width));
            robot = next_robot;
        }
    }

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if map[(y * width + x) as usize] == b'[' {
                sum += y * 100 + x;
            }
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day15.txt")?;

    let (map, path) = contents.split_once("\n\n").unwrap();
    let part1_sum = part1(map, path);
    println!("Part 1: {}", part1_sum);
    assert_eq!(part1_sum, 1509074);

    let part2_sum = part2(map, path);
    println!("Part 2: {}", part2_sum);
    assert_eq!(part2_sum, 1521453);

    Ok(())
}
