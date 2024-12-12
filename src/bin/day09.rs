use std::fs;

use itertools::Itertools;

struct Block {
    id: usize,
    size: u8,
    sub: Vec<Block>,
}

struct Checksum {
    pos: usize,
    sum: usize,
}

impl Checksum {
    fn new() -> Self {
        Self { pos: 0, sum: 0 }
    }
    fn add(&mut self, id: usize) {
        self.sum += self.pos * id;
        self.pos += 1;
    }
    fn add_empty(&mut self, count: usize) {
        self.pos += count;
    }
}

fn part1(mut diskmap: Vec<u8>) -> usize {
    let mut j = diskmap.len() - 1;

    let mut checksum = Checksum::new();
    'outer: for i in (0..diskmap.len()).step_by(2) {
        // First handle ready blocks
        while diskmap[i] > 0 {
            let id = i / 2;
            checksum.add(id);
            diskmap[i] -= 1;
        }

        // Then move blocks to free space
        while diskmap[i + 1] > 0 {
            while diskmap[j] == 0 && j > i {
                j -= 2;
            }
            if j <= i {
                break 'outer;
            }

            let id = j / 2;
            checksum.add(id);
            diskmap[j] -= 1;
            diskmap[i + 1] -= 1;
        }
    }

    checksum.sum
}

fn part2(diskmap: &[u8]) -> usize {
    let mut blocks = vec![];
    let mut free = vec![];
    for i in (0..diskmap.len()).step_by(2) {
        blocks.push(Block {
            id: i / 2,
            size: diskmap[i],
            sub: vec![],
        });
        if i == diskmap.len() - 1 {
            free.push(0);
        } else {
            free.push(diskmap[i + 1]);
        }
    }

    'outer: for j in (0..blocks.len()).rev() {
        for i in 0..j {
            if free[i] >= blocks[j].size {
                let mut block = Block {
                    id: 0,
                    size: 0,
                    sub: vec![],
                };
                std::mem::swap(&mut block, &mut blocks[j]);

                blocks[j].sub.append(&mut block.sub);
                free[i] -= block.size;
                free[j - 1] += block.size;
                blocks[i].sub.push(block);
                continue 'outer;
            }
        }
    }

    let mut checksum = Checksum::new();

    for i in 0..blocks.len() {
        let block = &blocks[i];
        for _ in 0..block.size {
            checksum.add(block.id);
        }
        for b in &block.sub {
            for _ in 0..b.size {
                checksum.add(b.id);
            }
        }
        checksum.add_empty(free[i].into());
    }

    checksum.sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day09.txt")?;

    let disk = contents
        .lines()
        .join("")
        .into_bytes()
        .iter()
        .map(|c| c - b'0')
        .collect_vec();

    let part1_checksum = part1(disk.clone());
    println!("Part 1 checksum: {}", part1_checksum);
    assert_eq!(part1_checksum, 6346871685398);

    let part2_checksum = part2(&disk);
    println!("Part 2 checksum: {}", part2_checksum);
    assert_eq!(part2_checksum, 6373055193464);

    Ok(())
}
