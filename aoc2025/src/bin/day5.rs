use std::ops::Range;

fn parse_input(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let ranges: Vec<_> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            a.parse().unwrap()..b.parse::<u64>().unwrap() + 1
        })
        .collect();
    let nums = input
        .lines()
        .skip(ranges.len() + 1)
        .map(|n| n.parse().unwrap())
        .collect();
    (ranges, nums)
}

fn part1(input: String) {
    let (ranges, nums) = parse_input(&input);
    let count = nums
        .iter()
        .filter(|n| ranges.iter().any(|r| r.contains(n)))
        .count();
    println!("{count}");
}

fn part2(input: String) {
    let (ranges, _) = parse_input(&input);
    // Ranges are added to this Vec always sorted by start and non-overlapping
    let mut merged: Vec<Range<u64>> = Vec::with_capacity(ranges.len());
    for r in ranges {
        // Find index of first intersecting range
        let first_int_o = merged.iter().position(|m| {
            // Intersection range (if any)
            let int_start = r.start.max(m.start);
            let int_end = r.end.min(m.end);
            int_start < int_end
        });
        if let Some(first_int) = first_int_o {
            // Exclusive
            let last_int = merged.len()
                - merged
                    .iter()
                    .rev()
                    .position(|m| {
                        let int_start = r.start.max(m.start);
                        let int_end = r.end.min(m.end);
                        int_start < int_end
                    })
                    .unwrap();
            // New range replacing all intersecting ranges
            let new = r.start.min(merged[first_int].start)..r.end.max(merged[last_int - 1].end);
            merged[first_int] = new;
            for i in (first_int + 1)..last_int {
                merged.remove(i);
            }
        } else {
            // Does not overlap with anything. Find index that keeps sorting
            let idx = merged
                .iter()
                .position(|m| m.start > r.start)
                .unwrap_or(merged.len());
            merged.insert(idx, r);
        }
    }
    let count = merged.iter().map(|r| r.end - r.start).sum::<u64>();
    println!("{count}");
}

util::aoc_main!();
