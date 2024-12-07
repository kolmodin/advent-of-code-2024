use std::{collections::HashSet, fs, ops::Div};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn step(&self, dir: &Dir) -> Self {
        Pos {
            x: self.x + dir.0.x,
            y: self.y + dir.0.y,
        }
    }

    fn within_bound(&self, edge: &Pos) -> bool {
        self.x >= 0 && self.x < edge.x && self.y >= 0 && self.y < edge.y
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Dir(Pos);

impl Dir {
    fn up() -> Dir {
        Dir(Pos { x: 0, y: -1 })
    }

    fn rotate_right(&self) -> Dir {
        Dir(Pos {
            x: -self.0.y,
            y: self.0.x,
        })
    }
}

enum Patrol {
    Exited,
    Looping,
}

fn patrol(map: Vec<u8>, start: Pos, bound: Pos) -> (Patrol, HashSet<Pos>) {
    let mut visited_dir = HashSet::new();
    let mut visited = HashSet::new();

    let mut pos = start;
    let mut dir = Dir::up();

    loop {
        visited.insert(pos);
        visited_dir.insert((pos, dir));

        let next = pos.step(&dir);
        if visited_dir.contains(&(next, dir)) {
            return (Patrol::Looping, visited);
        }
        if !next.within_bound(&bound) {
            return (Patrol::Exited, visited);
        }
        let blocked = map[(next.y * bound.y + next.x) as usize] == b'#';
        if blocked {
            dir = dir.rotate_right();
        } else {
            pos = next;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day06.txt")?;

    let width = contents.find('\n').unwrap() as i32;
    let height = contents.lines().count() as i32;
    let bound = Pos {
        x: width,
        y: height,
    };
    let original_map = contents.lines().join("").into_bytes();
    let start_pos = original_map
        .iter()
        .position(|&b| b == b'^')
        .map(|i| Pos {
            x: i as i32 % width,
            y: (i as i32).div(width),
        })
        .unwrap();

    let (_, visited) = patrol(original_map.clone(), start_pos, bound);
    let part1_visited = visited.len();

    println!("Part 1: {} positions visited", part1_visited);
    assert_eq!(part1_visited, 4559);

    let mut part2_loops = 0;
    for obsticle in visited {
        if start_pos == obsticle {
            continue;
        }
        let mut map = original_map.clone();
        map[(obsticle.y * width + obsticle.x) as usize] = b'#';
        let (patrol, _) = patrol(map, start_pos, bound);
        if let Patrol::Looping = patrol {
            part2_loops += 1;
        }
    }

    println!("Part 2: {} obsticles possible", part2_loops);
    assert_eq!(part2_loops, 1604);

    Ok(())
}
