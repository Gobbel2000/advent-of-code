type Tree = ((u32, u32), Vec<u32>);

fn parse_input(input: &str) -> (Vec<u32>, Vec<Tree>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut shapes = Vec::with_capacity(parts.len() - 1);
    for p in parts.iter().take(parts.len() - 1) {
        let n_filled = p.chars().filter(|c| *c == '#').count();
        shapes.push(n_filled as u32);
    }

    let mut trees = Vec::new();
    for l in parts.last().unwrap().lines() {
        let (size_s, counts_s) = l.split_once(": ").unwrap();
        let (x_s, y_s) = size_s.split_once('x').unwrap();
        let size = (x_s.parse::<u32>().unwrap(), y_s.parse::<u32>().unwrap());
        let counts = counts_s
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        trees.push((size, counts));
    }
    (shapes, trees)
}

fn part1(input: String) {
    let (shapes, trees) = parse_input(&input);
    let n = trees
        .iter()
        .filter(|(size, counts)| {
            let size_needed: u32 = counts.iter().zip(&shapes).map(|(c, s)| c * s).sum();
            let size_available = size.0 * size.1;
            size_needed <= size_available
        })
        .count();
    println!("{n}");
}

fn part2(_input: String) {}

util::aoc_main!();
