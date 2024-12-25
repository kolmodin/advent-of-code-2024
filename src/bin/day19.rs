use std::fs;

use itertools::Itertools;

fn matching(design: &str, pattern: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }
    pattern
        .iter()
        .any(|p| design.starts_with(p) && matching(&design[p.len()..], pattern))
}

fn count(design: &str, pattern: &[&str]) -> usize {
    let mut paths = vec![0; design.len() + 1];

    *paths.last_mut().unwrap() = 1;

    for i in (0..design.len()).rev() {
        paths[i] = pattern
            .iter()
            .filter(|&p| design[i..design.len()].starts_with(p))
            .map(|&p| paths[i + p.len()])
            .sum();
    }
    *paths.first().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day19.txt")?;

    let (words, towels) = contents.split_once("\n\n").unwrap();
    let words = words.split(", ").collect_vec();
    let towels = towels.lines().collect_vec();

    let part1_matching = towels.iter().filter(|w| matching(w, &words)).count();
    println!("Part 1: {}", part1_matching);
    assert_eq!(part1_matching, 236);

    let part2_combinations: usize = towels.iter().map(|&t| count(t, &words)).sum();
    println!("Part 2: {}", part2_combinations);
    assert_eq!(part2_combinations, 643685981770598);

    Ok(())
}
