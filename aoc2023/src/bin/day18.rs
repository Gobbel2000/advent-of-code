use std::str::FromStr;

enum Dir {
    N,
    E,
    S,
    W,
}

impl FromStr for Dir {
    type Err = String;
    // Part 1
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::N),
            "R" => Ok(Dir::E),
            "D" => Ok(Dir::S),
            "L" => Ok(Dir::W),
            _ => Err(format!("Invalid direction char {s}")),
        }
    }
}

impl TryFrom<u32> for Dir {
    type Error = String;
    // Part 2
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Dir::E),
            1 => Ok(Dir::S),
            2 => Ok(Dir::W),
            3 => Ok(Dir::N),
            _ => Err(format!("Invalid direction number {n}"))
        }
    }
}

struct Edge {
    dir: Dir,
    length: u32,
}

impl Edge {
    fn from_str_part1(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();
        let dir = parts.next()?.to_string().parse().ok()?;
        let length = parts.next()?.parse::<u32>().ok()?;
        Some(Edge {
            dir,
            length,
        })
    }

    fn from_str_part2(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();
        let not_color = parts.nth(2)?;
        let hex = not_color.trim_start_matches("(#").trim_end_matches(')');
        let num = u32::from_str_radix(hex, 16).ok()?;
        Some(Edge {
            dir: Dir::try_from(num & 0xf).ok()?,
            length: num >> 4,
        })
    }
}

fn area(edges: &[Edge]) -> u64 {
    // Initialize with 1 for starting point
    let mut area: i64 = 1;
    // Horizontal position
    let mut pos = 0;
    for edge in edges {
        let l = edge.length as i64;
        match edge.dir {
            Dir::S => area += l * (pos + 1),
            Dir::N => area -= l * pos,
            Dir::E => { area += l; pos += l },
            Dir::W => pos -= l,
        }
    }
    // If the path went counter-clockwise, area would end up negative
    area.unsigned_abs()
}

fn part1(input: String) {
    let inputs: Vec<Edge> = input.lines()
        .map(|l| Edge::from_str_part1(l).unwrap())
        .collect(); 
    println!("{}", area(&inputs));
}

fn part2(input: String) {
    let inputs: Vec<Edge> = input.lines()
        .map(|l| Edge::from_str_part2(l).unwrap())
        .collect();
    println!("{}", area(&inputs));
}

util::aoc_main!("day18.txt");
