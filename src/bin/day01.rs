use itertools::Itertools;
use std::fs;
use std::iter::zip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("inputs/day01.txt")?;

    let mut col1: Vec<i64> = vec![];
    let mut col2: Vec<i64> = vec![];

    for ln in content.lines() {
        match ln.split_whitespace().collect::<Vec<_>>()[..] {
            [val1, val2] => {
                col1.push(val1.parse()?);
                col2.push(val2.parse()?);
            }
            _ => panic!("unexpected number of items"),
        }
    }

    col1.sort();
    col2.sort();

    let part1: i64 = zip(&col1, &col2).map(|(x, y)| (x - y).abs()).sum();

    println!("Part 1: {}", part1);
    assert_eq!(part1, 2166959);

    let col2_freq = col2.iter().counts();

    let part2: i64 = col1
        .iter()
        .map(|&x| {
            col2_freq
                .get(&x)
                .map(|&y| x * i64::try_from(y).expect("usize to i64"))
                .unwrap_or(0)
        })
        .sum();

    println!("Part 2: {}", part2);
    assert_eq!(part2, 23741109);

    Ok(())
}
