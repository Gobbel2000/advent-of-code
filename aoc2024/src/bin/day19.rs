use regex::Regex;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    (towels.split(", ").collect(), designs.lines().collect())
}

fn part1(input: String) {
    let (towels, designs) = parse(&input);
    let pat = format!("^({})*$", towels.join("|"));
    let re = Regex::new(&pat).unwrap();
    let count = designs.iter().filter(|d| re.is_match(d)).count();
    println!("{count}");
}

fn n_arrangements<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(n) = cache.get(design) {
        return *n;
    }
    let n = towels
        .iter()
        .filter(|t| design.starts_with(*t))
        .map(|t| n_arrangements(&design[t.len()..], towels, cache))
        .sum();
    cache.insert(design, n);
    n
}

fn part2(input: String) {
    let (towels, designs) = parse(&input);
    let sum: u64 = designs
        .iter()
        .map(|d| n_arrangements(d, &towels, &mut FxHashMap::default()))
        .sum();
    println!("{sum}");
}

util::aoc_main!();
