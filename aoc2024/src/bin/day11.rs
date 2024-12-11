use std::collections::HashMap;

fn parse(input: String) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn part1(input: String) {
    let mut stones = parse(input);
    for _ in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len());
        for s in &stones {
            match s {
                0 => new_stones.push(1),
                n => {
                    let digits = s.ilog10() + 1;
                    if digits % 2 == 0 {
                        let cutoff = 10u64.pow(digits / 2);
                        new_stones.push(n / cutoff);
                        new_stones.push(n % cutoff);
                    } else {
                        new_stones.push(n * 2024)
                    }
                }
            }
        }
        stones = new_stones;
    }
    println!("{}", stones.len());
}

fn expansion(s: u64, rounds: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    // Recursion anchor
    if rounds == 0 {
        return 1;
    }
    // Calculation is already cached
    if let Some(res) = cache.get(&(s, rounds)) {
        return *res;
    }

    // Recurse
    let res = match s {
        0 => expansion(1, rounds - 1, cache),
        n => {
            let digits = s.ilog10() + 1;
            if digits % 2 == 0 {
                let cutoff = 10u64.pow(digits / 2);
                expansion(n / cutoff, rounds - 1, cache) +
                expansion(n % cutoff, rounds - 1, cache)
            } else {
                expansion(n * 2024, rounds - 1, cache)
            }
        }
    };
    // Save in cache
    cache.insert((s, rounds), res);
    res
}

fn part2(input: String) {
    let stones = parse(input);
    let mut cache = HashMap::new();
    let sum: u64 = stones.iter().map(|s| expansion(*s, 75, &mut cache)).sum();
    println!("{sum}");
}

util::aoc_main!();
