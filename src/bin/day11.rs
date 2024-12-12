use std::collections::HashMap;

use itertools::Itertools;

fn once(nums: &[u64]) -> Vec<u64> {
    let mut result = vec![];
    for &n in nums {
        if n == 0 {
            result.push(1);
            continue;
        }
        let f = format!("{}", n);
        if f.len() % 2 == 0 {
            let f1 = f[0..f.len() / 2].parse().unwrap();
            let f2 = f[f.len() / 2..].parse().unwrap();
            result.push(f1);
            result.push(f2);
            continue;
        }
        result.push(n * 2024);
    }

    result
}

fn once2(nums: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut hm = HashMap::new();

    for (n, count) in nums.iter() {
        let m = once(&[*n]);
        for i in m {
            *hm.entry(i).or_default() += count;
        }
    }

    hm
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nums = vec![1950139, 0, 3, 837, 6116, 18472, 228700, 45];

    let mut part1 = nums.clone();
    for _ in 0..25 {
        part1 = once(&part1);
    }
    println!("Part 1: {}", part1.len());

    let mut part2 = nums.into_iter().counts();
    for _ in 0..75 {
        part2 = once2(&part2);
    }
    let part2_sum: usize = part2.values().sum();
    println!("Part 2: {}", part2_sum);

    Ok(())
}
