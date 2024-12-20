use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs,
};

use aoc2024::{map::Map, pos::Pos};
use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]

struct Dir(u8);

impl Dir {
    fn east() -> Dir {
        Dir(1)
    }
    fn left(&self) -> Dir {
        Dir((self.0 + 3).rem_euclid(4))
    }
    fn right(&self) -> Dir {
        Dir((self.0 + 1).rem_euclid(4))
    }
    fn as_pos(&self) -> Pos {
        match self.0 {
            0 => Pos { x: 0, y: -1 },
            1 => Pos { x: 1, y: 0 },
            2 => Pos { x: 0, y: 1 },
            3 => Pos { x: -1, y: 0 },
            _ => panic!("invalid dir"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct State {
    pos: Pos,
    dir: Dir,
    tiles: Vec<Pos>,
}

impl State {
    fn rep(&self) -> (Pos, Dir) {
        (self.pos, self.dir)
    }
}

struct Search<N: Fn(&State, i64) -> Vec<(State, i64)>> {
    queue: PriorityQueue<State, Reverse<i64>>,
    seen: HashMap<(Pos, Dir), i64>,
    next: N,
}

impl<N: Fn(&State, i64) -> Vec<(State, i64)>> Iterator for Search<N> {
    type Item = (State, i64);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((state, Reverse(cost))) = self.queue.pop() {
            let rep = state.rep();
            if let Some(&prev_cost) = self.seen.get(&rep) {
                if prev_cost != cost {
                    assert!(prev_cost < cost);
                    continue;
                }
            } else {
                self.seen.insert(rep, cost);
            }
            let next = (self.next)(&state, cost);
            for (n, c) in next {
                self.queue.push(n, Reverse(c));
            }
            return Some((state, cost));
        }

        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day16.txt")?;

    let map = Map::<u8>::from(contents.as_str());

    let start = map.position(|p| *p == b'S').unwrap();
    let end = map.position(|p| *p == b'E').unwrap();

    println!("start {} end {}", start, end);

    let mut search = Search {
        queue: PriorityQueue::new(),
        seen: HashMap::new(),
        next: |state: &State, cost: i64| {
            let mut res = vec![];

            if let Some(&c) = map.get(state.pos + state.dir.as_pos()) {
                if c != b'#' {
                    let mut tiles = state.tiles.clone();
                    tiles.push(state.pos + state.dir.as_pos());
                    res.push((
                        State {
                            pos: state.pos + state.dir.as_pos(),
                            dir: state.dir,
                            tiles,
                        },
                        cost + 1,
                    ));
                }
            }

            res.push((
                State {
                    pos: state.pos,
                    dir: state.dir.left(),
                    tiles: state.tiles.clone(),
                },
                cost + 1000,
            ));
            res.push((
                State {
                    pos: state.pos,
                    dir: state.dir.right(),
                    tiles: state.tiles.clone(),
                },
                cost + 1000,
            ));
            res
        },
    };

    search.queue.push(
        State {
            pos: start,
            dir: Dir::east(),
            tiles: vec![start],
        },
        Reverse(0),
    );

    let mut arrived = search.filter(|s| s.0.pos == end);
    let best = arrived.next().unwrap();
    let equals = arrived.take_while(|s| s.1 == best.1).collect_vec();
    println!("Part 1: {}", best.1);
    assert_eq!(best.1, 94436);
    println!("equivalent paths: {}", equals.len());

    let mut all_tiles = HashSet::<Pos>::new();
    all_tiles.extend(&best.0.tiles);
    all_tiles.extend(equals.into_iter().flat_map(|e| e.0.tiles));
    let part2 = all_tiles.len();

    println!("Part 2: {}", part2);
    assert_eq!(part2, 481);

    Ok(())
}
