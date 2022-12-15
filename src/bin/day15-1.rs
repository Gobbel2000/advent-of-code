use std::fs;
use std::process::exit;
use std::ops::Range;

use regex::Regex;
use lazy_static::lazy_static;

static INPUT: &str = "input/day15.txt";
const SIZE: i32 = 4_000_000;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    dist: u32,
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let sensors = parse_input(&input);
    for y in 0..=SIZE {
        let ranges = get_ranges(&sensors, y);
        if has_gap(&ranges) {
            let x = condense_ranges(&ranges)[0].end as i64;
            println!("{}", x*(SIZE as i64) + y as i64);
            break;
        }
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    lazy_static! {
        static ref PAT: Regex = Regex::new(
            r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
    }
    let mut sensors = Vec::new();
    for line in input.lines() {
        let caps = PAT.captures(line).unwrap_or_else(|| {
            eprintln!("Malformed line: {}", line);
            exit(5);
        });
        let pos = (caps.get(1).unwrap().as_str().parse().unwrap(),
                   caps.get(2).unwrap().as_str().parse().unwrap());
        let beacon = (caps.get(3).unwrap().as_str().parse().unwrap(),
                      caps.get(4).unwrap().as_str().parse().unwrap());
        let dist = manhattan(pos, beacon);
        sensors.push(Sensor { pos, dist });
    }
    return sensors;
}

fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn get_ranges(sensors: &Vec<Sensor>, y: i32) -> Vec<Range<i32>> {
    let mut ranges: Vec<Range<i32>> = sensors.iter()
        .filter_map(|s| {
            let width = s.dist.checked_sub(s.pos.1.abs_diff(y))? as i32;
            Some((s.pos.0 - width)..(s.pos.0 + width + 1))
        })
        .collect();
    ranges.sort_unstable_by_key(|r| r.start);
    return ranges;
}

fn condense_ranges(ranges: &Vec<Range<i32>>) -> Vec<Range<i32>> {
    let mut condensed = Vec::new();
    if ranges.len() == 0 {
        return condensed;
    }
    let mut cur = ranges[0].clone();
    for r in ranges[1..].iter() {
        if r.start > cur.end {
            condensed.push(cur);
            cur = r.clone();
        } else if cur.end < r.end {
            cur.end = r.end;
        }
    }
    condensed.push(cur);

    return condensed;
}

// Faster version of condense_ranges that only returns whether there is a gap
// Assumes that ranges is not empty
fn has_gap(ranges: &Vec<Range<i32>>) -> bool {
    let mut end = ranges[0].end;
    for r in ranges[1..].iter() {
        if r.start > end {
            return true;
        }
        if end < r.end {
            end = r.end;
        }
    }
    return false;
}
