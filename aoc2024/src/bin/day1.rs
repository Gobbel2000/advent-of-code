use std::str::FromStr;
use std::collections::HashMap;

fn part1(input: String) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace()
            .map(|p| u32::from_str(p).unwrap());
        left.push(parts.next().unwrap());
        right.push(parts.next().unwrap());
    }
    left.sort_unstable();
    right.sort_unstable();
    let diff: u32 = left.iter().zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    println!("{diff}");
}

fn part2(input: String) {
    let mut left = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace()
            .map(|p| u32::from_str(p).unwrap());
        left.push(parts.next().unwrap());
        *right.entry(parts.next().unwrap()).or_default() += 1;
    }
    let similar: u32 = left.iter()
        .map(|n| n * right.get(n).copied().unwrap_or_default())
        .sum();
    println!("{similar}");
}

util::aoc_main!();
