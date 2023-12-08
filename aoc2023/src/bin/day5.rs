use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MapRange {
    src_start: u32,
    length: u32,
    diff: i64,
}

impl FromStr for MapRange {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        let dst_start: i64 = nums.next().unwrap().parse()?;
        let src_start = nums.next().unwrap().parse()?;
        let length = nums.next().unwrap().parse()?;
        assert!(nums.next().is_none());

        Ok(MapRange {
            src_start,
            diff: dst_start - (src_start as i64),
            length,
        })
    }
}

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<MapRange>,
}

impl RangeMap {
    fn map(&self, key: u32) -> u32 {
        for r in &self.ranges {
            if key < r.src_start {
                break;
            }
            if key < r.src_start + r.length {
                return (key as i64 + r.diff) as u32;
            }
        }
        return key;
    }

    fn inverse(&self, key: u32) -> u32 {
        let key = key as i64;
        for r in &self.ranges {
            let mapped_start = r.src_start as i64 + r.diff;
            if key >= mapped_start && key < mapped_start + r.length as i64 {
                return (key as i64 - r.diff) as u32;
            }
        }
        return key as u32;
    }
}

fn parse_input(input: String) -> (Vec<u32>, Vec<RangeMap>) {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    assert!(seeds_line.starts_with("seeds:"));
    let seeds: Vec<u32> = seeds_line[6..].split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut maps: Vec<RangeMap> = Vec::new();
    for line in lines.skip(1) {
        if line.is_empty() {
            maps.last_mut().unwrap().ranges.sort();
            continue;
        }
        if line.ends_with("map:") {
            maps.push(RangeMap { ranges: Vec::new() });
            continue;
        }
        let range = line.parse().unwrap();
        maps.last_mut().unwrap().ranges.push(range);
    }
    maps.last_mut().unwrap().ranges.sort();
    (seeds, maps)
}

fn part1(input: String) {
    let (seeds, maps) = parse_input(input); 
    let mut dist = u32::MAX;
    for s in seeds {
        let mut val = s;
        for m in &maps {
            val = m.map(val);
        }
        if val < dist {
            dist = val;
        }
    }
    println!("{dist}");
}

fn part2(input: String) {
    let (seeds, maps) = parse_input(input); 

    // Collect the start points of all ranges in all maps and go back through all mappings
    // The lowest location number must be located at the start of some range, either of the input
    // seed ranges or one of the mapping ranges.
    let mut range_starts: Vec<u32> = Vec::new();
    for m in maps.iter().rev() {
        for start in range_starts.iter_mut() {
            *start = m.inverse(*start);
        }
        // Add new range starts from current map
        for r in &m.ranges {
            range_starts.push(r.src_start);
            range_starts.push(r.src_start + r.length);
        }
    }
    // Add the start points of each seed range
    for s in seeds.iter().step_by(2) {
        range_starts.push(*s);
    }
    println!("Range starts: {}", range_starts.len());

    let mut dist = u32::MAX;
    for start in range_starts {
        if !seeds.chunks_exact(2).any(|chunk|
            start >= chunk[0] && start < chunk[0] + chunk[1]
        ) {
            // Range start is not a seed
            continue;
        }
        let mut val = start;
        for m in &maps {
            val = m.map(val);
        }
        if val < dist {
            dist = val;
        }
    }
    println!("{dist}");
}

util::aoc_main!("day5.txt");
