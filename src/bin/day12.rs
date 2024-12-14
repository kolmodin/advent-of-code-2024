use std::{collections::HashSet, fs};

use aoc2024::pos::{Bounds, Pos};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day12.txt")?;

    let width = contents.find('\n').unwrap() as i64;
    let height = contents.lines().count() as i64;
    let bounds = Pos {
        y: height,
        x: width,
    };
    let contents = contents.split('\n').join("").into_bytes();

    let mut seen = HashSet::new();
    let mut regions = vec![];
    for y in 0..height {
        for x in 0..width {
            let start_pos = Pos::new(x, y);
            if seen.contains(&start_pos) {
                continue;
            }

            let mut region: HashSet<Pos> = HashSet::new();
            let mut queue = vec![start_pos];
            let mut perimiter: Vec<Pos> = vec![];

            while let Some(p) = queue.pop() {
                if !p.within_bounds(&bounds) {
                    perimiter.push(p);
                    continue;
                }
                if contents[p.to_linear(width)] != contents[start_pos.to_linear(width)] {
                    perimiter.push(p);
                    continue;
                }
                if seen.contains(&p) {
                    continue;
                }
                seen.insert(p);
                region.insert(p);
                queue.extend(p.cross());
            }
            regions.push((start_pos, region, perimiter));
        }
    }

    let part1 = regions
        .iter()
        .map(|(_, r, p)| r.len() * p.len())
        .sum::<usize>();
    println!("Part 1: {}", part1);
    assert_eq!(part1, 1477762);

    let mut part2 = 0;
    for r in regions {
        let boxed = Bounds::from_iter(r.1.iter()).unwrap().expand();
        let name = contents[r.0.to_linear(width)];
        let mut sides = 0;
        for d in &[Pos::up, Pos::down] {
            let mut within_side = false;
            for y in boxed.along_y() {
                for x in boxed.along_x() {
                    let p = Pos::new(x, y);
                    if !r.1.contains(&p) {
                        within_side = false;
                        continue;
                    }
                    let d = (d)(&p);
                    let out2 = !d.within_bounds(&bounds) || contents[d.to_linear(width)] != name;
                    if out2 == within_side {
                        continue;
                    }
                    if out2 {
                        sides += 1;
                    }
                    within_side = out2;
                }
            }
        }

        for d in &[Pos::left, Pos::right] {
            let mut within_side = false;
            for x in boxed.along_x() {
                for y in boxed.along_y() {
                    let p = Pos::new(x, y);
                    if !r.1.contains(&p) {
                        within_side = false;
                        continue;
                    }
                    let d = (d)(&p);
                    let out2 = !d.within_bounds(&bounds) || contents[d.to_linear(width)] != name;
                    if out2 == within_side {
                        continue;
                    }
                    if out2 {
                        sides += 1;
                    }
                    within_side = out2;
                }
            }
        }

        part2 += r.1.len() * sides;
    }

    println!("Part 2: {}", part2);
    assert_eq!(part2, 923480);

    Ok(())
}
