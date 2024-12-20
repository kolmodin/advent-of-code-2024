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

struct Mover<'a> {
    map: &'a [u8],
    seen: HashSet<Pos>,
    order: Vec<Pos>,
    width: i64,
    dir: Pos,
}

impl<'a> Mover<'a> {
    fn is_movable(mut self, p: Pos) -> Option<Vec<Pos>> {
        if self.recurse(p) {
            Some(self.order)
        } else {
            None
        }
    }
    fn recurse(&mut self, p: Pos) -> bool {
        if self.seen.contains(&p) {
            return true;
        }
        let c = self.map[p.to_linear(self.width)];
        match c {
            b'.' => true,
            b'#' => false,
            b'[' | b']' => {
                let sibling = if c == b'[' { p.right() } else { p.left() };
                if !self.recurse(sibling + self.dir) {
                    return false;
                }
                self.seen.insert(sibling);
                self.order.push(sibling);

                if !self.recurse(p + self.dir) {
                    return false;
                }
                self.seen.insert(p);
                self.order.push(p);

                true
            }
            _ => panic!("unknown char in map"),
        }
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
            if let Some(group) = (Mover {
                map: &map,
                dir,
                order: Default::default(),
                seen: Default::default(),
                width,
            }
            .is_movable(next_robot))
            {
                for p in group {
                    map.swap(p.to_linear(width), (p + dir).to_linear(width));
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
