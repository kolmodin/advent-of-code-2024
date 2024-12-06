use std::{collections::HashMap, fs};

use itertools::Itertools;

fn rule_ok(update: &[i32], rule: &(i32, i32)) -> bool {
    let x = update.iter().position(|&x| x == rule.0);
    let y = update.iter().position(|&y| y == rule.1);

    if let Some(x) = x {
        if let Some(y) = y {
            return x < y;
        }
    }
    return true;
}

fn rules_ok(update: &[i32], rules: &[(i32, i32)]) -> bool {
    rules.iter().all(|rule| rule_ok(update, rule))
}

fn midpoint(update: &[i32]) -> i32 {
    assert!(update.len() % 2 == 1);
    update[update.len() / 2]
}

/* Kahn's topological sort algorithm */
fn top_sort(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    // The right page points to the left.
    // k:node v:outgoing edges
    let mut nodes: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        if !update.contains(&rule.0) || !update.contains(&rule.1) {
            continue;
        }
        nodes.entry(rule.0).or_insert_with(|| vec![]);
        nodes
            .entry(rule.1)
            .and_modify(|k| k.push(rule.0))
            .or_insert_with(|| vec![rule.0]);
    }

    let mut no_outs = nodes
        .iter()
        .filter_map(|(n, out)| if out.is_empty() { Some(*n) } else { None })
        .collect_vec();

    let mut sorted = vec![];
    while let Some(top) = no_outs.pop() {
        sorted.push(top);
        nodes.remove(&top);
        for (&node, outgoing) in nodes.iter_mut() {
            outgoing.retain(|&n| n != top);
            if outgoing.is_empty() {
                no_outs.push(node);
            }
        }
    }

    sorted
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("inputs/day05.txt")?;

    let (order_rules, updates) = contents.split_once("\n\n").unwrap();

    let mut rules: Vec<(i32, i32)> = vec![];
    for ln in order_rules.lines() {
        let nums = ln
            .split("|")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec();
        rules.push((nums[0], nums[1]));
    }

    let updates = updates
        .lines()
        .map(|ln| {
            ln.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let (ordered, unordered) = updates
        .iter()
        .partition::<Vec<_>, _>(|update| rules_ok(update, &rules));

    let part1_sum: i32 = ordered.iter().map(|u| midpoint(u)).sum();

    println!("Part 1: {}", part1_sum);
    assert_eq!(part1_sum, 4185);

    let part2_sum: i32 = unordered
        .iter()
        .map(|update| midpoint(&top_sort(update, &rules)))
        .sum();

    println!("Part 2: {}", part2_sum);
    assert_eq!(part2_sum, 4480);

    Ok(())
}
