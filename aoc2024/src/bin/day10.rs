use std::collections::HashSet;

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

fn adj(grid: &[&[u8]], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let n = grid[y][x];
    let mut adj = Vec::with_capacity(4);
    if x > 0 && grid[y][x - 1] == n + 1 {
        adj.push((x - 1, y))
    }
    if y > 0 && grid[y - 1][x] == n + 1 {
        adj.push((x, y - 1))
    }
    if x + 1 < grid[0].len() && grid[y][x + 1] == n + 1 {
        adj.push((x + 1, y))
    }
    if y + 1 < grid.len() && grid[y + 1][x] == n + 1 {
        adj.push((x, y + 1))
    }
    adj
}

fn solve(input: String, trailhead: fn(&[&[u8]], (usize, usize)) -> u32) -> u32 {
    let grid = parse(&input);
    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if *p == b'0' {
                sum += trailhead(&grid, (x, y));
            }
        }
    }
    sum
}

fn part1(input: String) {
    fn score(grid: &[&[u8]], start: (usize, usize)) -> u32 {
        (1..=9)
            .fold(HashSet::from([start]), |frontier, _| {
                frontier.iter().flat_map(|p| adj(grid, *p)).collect()
            })
            .len() as u32
    }
    println!("{}", solve(input, score))
}

fn part2(input: String) {
    fn rating(grid: &[&[u8]], start: (usize, usize)) -> u32 {
        (1..=9)
            .fold(vec![start], |frontier, _| {
                frontier.iter().flat_map(|p| adj(grid, *p)).collect()
            })
            .len() as u32
    }
    println!("{}", solve(input, rating))
}

util::aoc_main!();
